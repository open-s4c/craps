use std::{
    error::Error as StdError,
    fs::File,
    io::{
        Error,
        ErrorKind,
    },
    os::fd::AsRawFd,
    ptr::NonNull,
    slice,
};

pub(crate) const ALIGNMENT: usize = 16;
pub(crate) const HEADER_SIZE: usize = 16;

// Must be large enough to contain any individual serialized record.
pub(crate) const MMAP_SIZE: usize = 1024 * 1024;

pub(crate) fn padding_len(
    payload_len: usize,
) -> usize {
    (ALIGNMENT
        - (payload_len % ALIGNMENT))
        % ALIGNMENT
}

pub(crate) fn frame_len(
    payload_len: usize,
) -> Option<usize> {
    HEADER_SIZE
        .checked_add(payload_len)?
        .checked_add(
            padding_len(payload_len)
        )
}

pub(crate) type MappedError =
Box<dyn StdError + Send + Sync>;

fn invalid_data(
    message: impl Into<String>,
) -> MappedError {
    Box::new(
        Error::new(
            ErrorKind::InvalidData,
            message.into(),
        )
    )
}

fn page_size(
) -> Result<usize, MappedError> {
    // SAFETY:
    // sysconf(_SC_PAGESIZE) has no memory-safety requirements.
    let value = unsafe {
        libc::sysconf(
            libc::_SC_PAGESIZE
        )
    };

    if value <= 0 {
        return Err(
            Error::last_os_error().into()
        );
    }

    usize::try_from(value)
        .map_err(Into::into)
}

fn validate_layout(
    page_size: usize,
) -> Result<(), MappedError> {
    if MMAP_SIZE == 0 {
        return Err(
            invalid_data(
                "MMAP_SIZE must be nonzero"
            )
        );
    }

    if MMAP_SIZE % page_size != 0 {
        return Err(
            invalid_data(format!(
                "MMAP_SIZE ({MMAP_SIZE}) must be a \
                 multiple of the system page size \
                 ({page_size})"
            ))
        );
    }

    if HEADER_SIZE < 8 {
        return Err(
            invalid_data(
                "HEADER_SIZE must contain the \
                 8-byte payload length"
            )
        );
    }

    if HEADER_SIZE > MMAP_SIZE {
        return Err(
            invalid_data(
                "HEADER_SIZE exceeds MMAP_SIZE"
            )
        );
    }

    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MappingKind {
    Private,
    Shared,
}

impl MappingKind {
    fn flags(
        self,
    ) -> libc::c_int {
        match self {
            Self::Private => {
                libc::MAP_PRIVATE
            }

            Self::Shared => {
                libc::MAP_SHARED
            }
        }
    }
}

/// A fixed virtual-address range backed by a file.
///
/// The initial mmap lets the kernel choose the address. Every
/// subsequent remap uses MAP_FIXED to replace the mapping at
/// exactly the same virtual address.
///
/// The mapping is always exactly MMAP_SIZE bytes.
struct FixedFileMmap {
    ptr: NonNull<u8>,
    kind: MappingKind,
}

impl FixedFileMmap {
    fn new(
        file: &File,
        kind: MappingKind,
    ) -> Result<Self, MappedError> {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                MMAP_SIZE,
                libc::PROT_READ
                    | libc::PROT_WRITE,
                kind.flags(),
                file.as_raw_fd(),
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            return Err(
                Error::last_os_error().into()
            );
        }

        let ptr =
            NonNull::new(
                ptr.cast::<u8>()
            )
                .expect(
                    "successful mmap returned null"
                );

        Ok(Self {
            ptr,
            kind,
        })
    }

    fn remap(
        &mut self,
        file: &File,
        file_offset: u64,
    ) -> Result<(), MappedError> {
        let file_offset:
            libc::off_t =
            file_offset
                .try_into()
                .map_err(|_| {
                    invalid_data(
                        "mmap file offset does not fit off_t"
                    )
                })?;

        /*
         * Do NOT munmap first.
         *
         * MAP_FIXED atomically replaces the mapping over this
         * exact address range.
         *
         * The mapping length remains exactly MMAP_SIZE and the
         * virtual address never changes.
         *
         * The caller guarantees:
         *
         * - self.ptr is the start of the existing mapping,
         * - the replacement length is exactly MMAP_SIZE,
         * - file_offset is page aligned,
         * - no references into the previous mapping are alive.
         */
        let ptr = unsafe {
            libc::mmap(
                self.ptr
                    .as_ptr()
                    .cast::<libc::c_void>(),
                MMAP_SIZE,
                libc::PROT_READ
                    | libc::PROT_WRITE,
                self.kind.flags()
                    | libc::MAP_FIXED,
                file.as_raw_fd(),
                file_offset,
            )
        };

        if ptr == libc::MAP_FAILED {
            return Err(
                Error::last_os_error().into()
            );
        }

        if ptr
            != self.ptr
            .as_ptr()
            .cast::<libc::c_void>()
        {
            return Err(
                invalid_data(
                    "MAP_FIXED returned a different address"
                )
            );
        }

        Ok(())
    }

    fn as_slice(
        &self,
    ) -> &[u8] {
        // SAFETY:
        //
        // - ptr refers to a live MMAP_SIZE mapping,
        // - the mapping remains alive for the returned borrow,
        // - MMAP_SIZE is the exact mapping length.
        unsafe {
            slice::from_raw_parts(
                self.ptr.as_ptr(),
                MMAP_SIZE,
            )
        }
    }

    fn as_mut_slice(
        &mut self,
    ) -> &mut [u8] {
        // SAFETY:
        //
        // Same reasoning as as_slice(), with exclusive access
        // provided by &mut self.
        unsafe {
            slice::from_raw_parts_mut(
                self.ptr.as_ptr(),
                MMAP_SIZE,
            )
        }
    }
}

impl Drop for FixedFileMmap {
    fn drop(
        &mut self,
    ) {
        // SAFETY:
        //
        // ptr is the beginning of the fixed mapping and the
        // mapping has always remained exactly MMAP_SIZE bytes.
        unsafe {
            libc::munmap(
                self.ptr
                    .as_ptr()
                    .cast::<libc::c_void>(),
                MMAP_SIZE,
            );
        }
    }
}

/*
 * Raw mmap memory is safe to move/share under the synchronization
 * rules imposed by FixedFileMapper:
 *
 * - immutable access requires &self,
 * - mutable access/remapping requires &mut self.
 */
unsafe impl Send for FixedFileMmap {}
unsafe impl Sync for FixedFileMmap {}

pub(crate) struct FixedFileMapper {
    file: File,

    /*
     * Created immediately with the mapper and never changes its
     * virtual address or size.
     */
    mmap: FixedFileMmap,

    /*
     * Absolute, page-aligned file offset corresponding to mmap[0].
     */
    mmap_offset: u64,

    page_size: usize,

    kind: MappingKind,

    /*
     * Physical file length currently provisioned.
     *
     * For a reader this never changes.
     *
     * For a writer this normally extends through the complete
     * current MMAP_SIZE window and is trimmed by flush().
     */
    provisioned_len: u64,
}

impl FixedFileMapper {
    pub(crate) fn new_private(
        file: File,
    ) -> Result<Self, MappedError> {
        Self::new(
            file,
            MappingKind::Private,
        )
    }

    pub(crate) fn new_shared(
        file: File,
    ) -> Result<Self, MappedError> {
        Self::new(
            file,
            MappingKind::Shared,
        )
    }

    fn new(
        file: File,
        kind: MappingKind,
    ) -> Result<Self, MappedError> {
        let page_size =
            page_size()?;

        validate_layout(
            page_size
        )?;

        let provisioned_len =
            file.metadata()?.len();

        /*
         * Establish the VMA immediately.
         *
         * A writer may initially map an empty file. It must
         * provision backing through ensure_range() before touching
         * any bytes in the mapping.
         */
        let mmap =
            FixedFileMmap::new(
                &file,
                kind,
            )?;

        Ok(Self {
            file,
            mmap,
            mmap_offset: 0,
            page_size,
            kind,
            provisioned_len,
        })
    }

    fn contains(
        &self,
        start: u64,
        len: usize,
    ) -> bool {
        let Ok(len) =
            u64::try_from(len)
        else {
            return false;
        };

        let Ok(mmap_len) =
            u64::try_from(
                MMAP_SIZE
            )
        else {
            return false;
        };

        let Some(end) =
            start.checked_add(len)
        else {
            return false;
        };

        let Some(mapped_end) =
            self.mmap_offset
                .checked_add(
                    mmap_len
                )
        else {
            return false;
        };

        start >= self.mmap_offset
            && end <= mapped_end
    }

    fn aligned_offset(
        &self,
        offset: u64,
    ) -> u64 {
        let page_size =
            self.page_size as u64;

        offset
            - (offset % page_size)
    }

    /*
     * Determine the page-aligned mmap file offset needed to
     * contain [start, start + len).
     *
     * The mapping itself remains exactly MMAP_SIZE bytes.
     */
    fn mapping_offset_for(
        &self,
        start: u64,
        len: usize,
    ) -> Result<u64, MappedError> {
        if len > MMAP_SIZE {
            return Err(
                invalid_data(format!(
                    "trace range of {len} bytes exceeds \
                     fixed mmap size of {MMAP_SIZE} bytes"
                ))
            );
        }

        let mmap_offset =
            self.aligned_offset(
                start
            );

        let prefix =
            usize::try_from(
                start - mmap_offset
            )?;

        let required =
            prefix
                .checked_add(len)
                .ok_or_else(|| {
                    invalid_data(
                        "mmap range overflow"
                    )
                })?;

        if required > MMAP_SIZE {
            return Err(
                invalid_data(format!(
                    "trace range of {len} bytes at file \
                     offset {start} does not fit in fixed \
                     mmap size of {MMAP_SIZE} bytes after \
                     page alignment"
                ))
            );
        }

        Ok(mmap_offset)
    }

    /*
     * A MAP_SHARED writer must have real file backing before
     * mapped bytes are touched.
     *
     * Provision the entire mapping window so file growth happens
     * only when entering a new window rather than once per frame.
     */
    fn ensure_backing(
        &mut self,
        mmap_offset: u64,
    ) -> Result<(), MappedError> {
        if self.kind
            != MappingKind::Shared
        {
            return Ok(());
        }

        let required =
            mmap_offset
                .checked_add(
                    u64::try_from(
                        MMAP_SIZE
                    )?
                )
                .ok_or_else(|| {
                    invalid_data(
                        "mapped file length overflow"
                    )
                })?;

        if self.provisioned_len
            >= required
        {
            return Ok(());
        }

        self.file.set_len(
            required
        )?;

        self.provisioned_len =
            required;

        Ok(())
    }

    pub(crate) fn ensure_range(
        &mut self,
        start: u64,
        len: usize,
    ) -> Result<(), MappedError> {
        if self.contains(
            start,
            len,
        ) {
            /*
             * This also matters after a writer flush() has
             * truncated the underlying file while retaining the
             * same VMA.
             */
            self.ensure_backing(
                self.mmap_offset
            )?;

            return Ok(());
        }

        let mmap_offset =
            self.mapping_offset_for(
                start,
                len,
            )?;

        /*
         * Provide backing before a MAP_SHARED replacement can be
         * touched.
         */
        self.ensure_backing(
            mmap_offset
        )?;

        /*
         * Replace the mapping at exactly the same address.
         */
        self.mmap.remap(
            &self.file,
            mmap_offset,
        )?;

        self.mmap_offset =
            mmap_offset;

        debug_assert!(
            self.contains(
                start,
                len,
            )
        );

        Ok(())
    }

    pub(crate) fn range(
        &self,
        start: u64,
        len: usize,
    ) -> &[u8] {
        debug_assert!(
            self.contains(
                start,
                len,
            )
        );

        let local =
            usize::try_from(
                start
                    - self.mmap_offset
            )
                .expect(
                    "mapped offset should fit usize"
                );

        &self
            .mmap
            .as_slice()
            [local..local + len]
    }

    pub(crate) fn range_mut(
        &mut self,
        start: u64,
        len: usize,
    ) -> &mut [u8] {
        debug_assert!(
            self.contains(
                start,
                len,
            )
        );

        let local =
            usize::try_from(
                start
                    - self.mmap_offset
            )
                .expect(
                    "mapped offset should fit usize"
                );

        &mut self
            .mmap
            .as_mut_slice()
            [local..local + len]
    }

    pub(crate) fn truncate(
        &mut self,
        len: u64,
    ) -> Result<(), MappedError> {
        if self.kind
            != MappingKind::Shared
        {
            return Err(
                invalid_data(
                    "cannot truncate a private mapping"
                )
            );
        }

        self.file.set_len(
            len
        )?;

        self.provisioned_len =
            len;

        Ok(())
    }
}
