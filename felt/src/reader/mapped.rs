use std::{
    fs::File,
    io::{
        Error,
        ErrorKind,
    },
    os::fd::AsRawFd,
    ptr::NonNull,
    slice,
};

use crate::frame::{
    HEADER_SIZE,
    MMAP_SIZE,
    frame_len,
};

use super::ReaderError;

#[derive(Clone, Copy)]
pub(crate) struct Frame {
    pub start: u64,
    pub payload_start: u64,
    pub payload_len: usize,
    pub next_offset: u64,
}

fn invalid_data(
    message: impl Into<String>,
) -> ReaderError {
    Box::new(
        Error::new(
            ErrorKind::InvalidData,
            message.into(),
        )
    )
}

fn page_size() -> Result<usize, ReaderError> {
    // SAFETY:
    // sysconf(_SC_PAGESIZE) has no memory-safety requirements.
    let value = unsafe {
        libc::sysconf(libc::_SC_PAGESIZE)
    };

    if value <= 0 {
        return Err(
            Error::last_os_error().into()
        );
    }

    usize::try_from(value)
        .map_err(Into::into)
}

/// A fixed virtual-address range backed by a file.
///
/// The initial mmap lets the kernel choose the address. Every
/// subsequent remap uses MAP_FIXED to replace the mapping at
/// exactly the same virtual address.
struct FixedFileMmap {
    ptr: NonNull<u8>,
}

impl FixedFileMmap {
    fn new(
        file: &File,
    ) -> Result<Self, ReaderError> {
        let ptr = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                MMAP_SIZE,
                libc::PROT_READ
                    | libc::PROT_WRITE,
                libc::MAP_PRIVATE,
                file.as_raw_fd(),
                0,
            )
        };

        if ptr == libc::MAP_FAILED {
            return Err(
                Error::last_os_error().into()
            );
        }

        let ptr = NonNull::new(
            ptr.cast::<u8>()
        )
            .expect(
                "successful mmap returned null"
            );

        Ok(Self {
            ptr,
        })
    }

    fn remap(
        &mut self,
        file: &File,
        file_offset: u64,
    ) -> Result<(), ReaderError> {
        let file_offset:
            libc::off_t =
            file_offset.try_into()
                .map_err(|_| {
                    invalid_data(
                        "mmap file offset does not fit off_t"
                    )
                })?;

        /*
         * Do NOT munmap first.
         *
         * MAP_FIXED atomically replaces the mapping over this
         * exact address range. This guarantees that the virtual
         * address does not drift between windows.
         *
         * The caller guarantees:
         *
         * - self.ptr is the start of our existing mmap,
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
                libc::MAP_PRIVATE
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

        /*
         * MAP_FIXED promises this, but check it because preserving
         * the address is one of the reader's core invariants.
         */
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
 * rules imposed by MappedReader:
 *
 * - immutable access requires &self,
 * - mutable access/remapping requires &mut self.
 */
unsafe impl Send for FixedFileMmap {}
unsafe impl Sync for FixedFileMmap {}

pub(crate) struct MappedReader {
    file: File,
    file_len: u64,

    /*
     * Created immediately with the reader and never changes its
     * virtual address.
     */
    mmap: FixedFileMmap,

    /*
     * Absolute, page-aligned file offset corresponding to mmap[0].
     */
    mmap_offset: u64,

    /*
     * Absolute file offset of the next frame.
     */
    cursor: u64,

    page_size: usize,
}

impl MappedReader {
    pub(crate) fn from_path(
        path: &str,
    ) -> Result<Self, ReaderError> {
        let file = File::open(path)?;

        let page_size =
            page_size()?;

        if MMAP_SIZE == 0 {
            return Err(
                invalid_data(
                    "MMAP_SIZE must be nonzero"
                )
            );
        }

        /*
         * Keeping MMAP_SIZE page-aligned means both the anonymous
         * writer mapping and this file-backed mapping occupy
         * exactly the same number of pages.
         */
        if MMAP_SIZE % page_size != 0 {
            return Err(
                invalid_data(format!(
                    "MMAP_SIZE ({MMAP_SIZE}) must be a \
                     multiple of the system page size \
                     ({page_size})"
                ))
            );
        }

        if HEADER_SIZE > MMAP_SIZE {
            return Err(
                invalid_data(
                    "HEADER_SIZE exceeds MMAP_SIZE"
                )
            );
        }

        /*
         * IMPORTANT:
         *
         * Establish the mmap as part of reader construction.
         *
         * It is deliberately MMAP_SIZE even when the file is
         * smaller. We simply never dereference bytes beyond
         * file_len.
         */
        let mmap =
            FixedFileMmap::new(&file)?;

        let file_len =
            file.metadata()?.len();

        Ok(Self {
            file,
            file_len,
            mmap,
            mmap_offset: 0,
            cursor: 0,
            page_size,
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
            u64::try_from(MMAP_SIZE)
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
                .checked_add(mmap_len)
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
     * Determines the page-aligned mmap file offset needed to hold
     * [start, start + len).
     *
     * File mmap offsets must be page aligned. Because the mapping
     * itself is strictly MMAP_SIZE bytes, a record can technically
     * fail to fit even when len <= MMAP_SIZE if its start is not
     * page aligned and len is very close to MMAP_SIZE.
     */
    fn mapping_offset_for(
        &self,
        start: u64,
        len: usize,
    ) -> Result<u64, ReaderError> {
        if len > MMAP_SIZE {
            return Err(
                invalid_data(format!(
                    "trace range of {len} bytes exceeds \
                     fixed mmap size of {MMAP_SIZE} bytes"
                ))
            );
        }

        let mmap_offset =
            self.aligned_offset(start);

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

    fn ensure_range(
        &mut self,
        start: u64,
        len: usize,
    ) -> Result<(), ReaderError> {
        if self.contains(
            start,
            len,
        ) {
            return Ok(());
        }

        let mmap_offset =
            self.mapping_offset_for(
                start,
                len,
            )?;

        /*
         * MAP_FIXED replaces the existing mapping at the exact
         * same virtual address. No munmap/mmap-address-selection
         * gap occurs here.
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

    fn range(
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

        &self.mmap.as_slice()
            [local..local + len]
    }

    pub(crate) fn payload(
        &self,
        frame: Frame,
    ) -> &[u8] {
        self.range(
            frame.payload_start,
            frame.payload_len,
        )
    }

    pub(crate) fn payload_mut(
        &mut self,
        frame: Frame,
    ) -> &mut [u8] {
        debug_assert!(
            self.contains(
                frame.payload_start,
                frame.payload_len,
            )
        );

        let local =
            usize::try_from(
                frame.payload_start
                    - self.mmap_offset
            )
                .expect(
                    "mapped offset should fit usize"
                );

        &mut self
            .mmap
            .as_mut_slice()
            [local..local + frame.payload_len]
    }

    pub(crate) fn frame(
        &mut self,
    ) -> Result<Option<Frame>, ReaderError> {
        if self.cursor
            == self.file_len
        {
            return Ok(None);
        }

        if self.cursor
            > self.file_len
        {
            return Err(
                invalid_data(
                    "reader cursor is past end of trace"
                )
            );
        }

        /*
         * First establish that a complete header physically
         * exists in the file.
         */
        let header_end =
            self.cursor
                .checked_add(
                    HEADER_SIZE as u64
                )
                .ok_or_else(|| {
                    invalid_data(
                        "frame header overflow"
                    )
                })?;

        if header_end
            > self.file_len
        {
            return Err(
                invalid_data(
                    "truncated frame header"
                )
            );
        }

        /*
         * Stage 1:
         * ensure the header is visible.
         */
        self.ensure_range(
            self.cursor,
            HEADER_SIZE,
        )?;

        let payload_len = {
            let header =
                self.range(
                    self.cursor,
                    HEADER_SIZE,
                );

            let raw =
                u64::from_le_bytes(
                    header[..8]
                        .try_into()
                        .expect(
                            "header contains 8 length bytes"
                        ),
                );

            usize::try_from(raw)?
        };

        let total_len =
            frame_len(payload_len)
                .ok_or_else(|| {
                    invalid_data(
                        "frame length overflow"
                    )
                })?;

        /*
         * Keep the same fixed-size invariant as the writer.
         */
        if total_len
            > MMAP_SIZE
        {
            return Err(
                invalid_data(format!(
                    "trace range of {total_len} bytes \
                     exceeds fixed mmap size of \
                     {MMAP_SIZE} bytes"
                ))
            );
        }

        let next_offset =
            self.cursor
                .checked_add(
                    u64::try_from(
                        total_len
                    )?
                )
                .ok_or_else(|| {
                    invalid_data(
                        "frame offset overflow"
                    )
                })?;

        /*
         * Physical EOF is authoritative.
         *
         * The mmap may extend beyond EOF, but those bytes are
         * never accessed.
         */
        if next_offset
            > self.file_len
        {
            return Err(
                invalid_data(
                    "truncated frame payload"
                )
            );
        }

        /*
         * Stage 2:
         * ensure the complete frame is simultaneously visible in
         * the fixed-size mapping.
         *
         * If a remap is needed, it replaces the old file mapping
         * at exactly the same virtual address.
         */
        self.ensure_range(
            self.cursor,
            total_len,
        )?;

        Ok(Some(
            Frame {
                start:
                self.cursor,

                payload_start:
                self.cursor
                    + HEADER_SIZE as u64,

                payload_len,

                next_offset,
            }
        ))
    }

    pub(crate) fn advance(
        &mut self,
        frame: Frame,
    ) {
        self.cursor =
            frame.next_offset;
    }
}
