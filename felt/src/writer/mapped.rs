use std::{
    fs::OpenOptions,
    io::{
        Error,
        ErrorKind,
    },
};

use crate::mapped::{
    HEADER_SIZE,
    MMAP_SIZE,
    frame_len,
    FixedFileMapper,
};

use super::WriterError;

#[derive(Clone, Copy)]
pub(crate) struct WritableFrame {
    pub start: u64,
    pub payload_start: u64,
    pub payload_len: usize,

    total_len: usize,
    next_offset: u64,
}

fn invalid_data(
    message: impl Into<String>,
) -> WriterError {
    Box::new(
        Error::new(
            ErrorKind::InvalidData,
            message.into(),
        )
    )
}

pub(crate) struct MappedWriter {
    /*
     * Fixed-address MAP_SHARED file mapping.
     */
    mmap: FixedFileMapper,

    /*
     * Absolute logical trace-file offset of the next frame.
     */
    offset: u64,
}

impl MappedWriter {
    pub(crate) fn from_path(
        path: &str,
    ) -> Result<Self, WriterError> {
        if HEADER_SIZE > MMAP_SIZE {
            return Err(
                invalid_data(
                    "HEADER_SIZE exceeds MMAP_SIZE"
                )
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

        /*
         * MAP_SHARED + PROT_READ | PROT_WRITE requires the file
         * descriptor to be opened appropriately.
         */
        let file =
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(path)?;

        /*
         * Establish the VMA immediately.
         *
         * The file starts empty. The shared mapper provisions
         * backing before the mapping is actually touched.
         */
        let mmap =
            FixedFileMapper::new_shared(
                file
            )?;

        Ok(Self {
            mmap,
            offset: 0,
        })
    }

    pub(crate) fn prepare_frame(
        &mut self,
        payload_len: usize,
    ) -> Result<
        WritableFrame,
        WriterError,
    > {
        let total_len =
            frame_len(
                payload_len
            )
                .ok_or_else(|| {
                    invalid_data(
                        "trace frame length overflow"
                    )
                })?;

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

        /*
         * Mirror the reader's two-stage mapping behavior.
         *
         * Stage 1:
         * The reader first makes the header visible.
         */
        self.mmap.ensure_range(
            self.offset,
            HEADER_SIZE,
        )?;

        /*
         * Stage 2:
         * Once the reader knows payload_len, it ensures the whole
         * frame is simultaneously visible.
         *
         * This may MAP_FIXED-remap the VMA, so nothing belonging
         * to this frame is written before this call completes.
         */
        self.mmap.ensure_range(
            self.offset,
            total_len,
        )?;

        let payload_start =
            self.offset
                .checked_add(
                    u64::try_from(
                        HEADER_SIZE
                    )?
                )
                .ok_or_else(|| {
                    invalid_data(
                        "payload offset overflow"
                    )
                })?;

        let next_offset =
            self.offset
                .checked_add(
                    u64::try_from(
                        total_len
                    )?
                )
                .ok_or_else(|| {
                    invalid_data(
                        "trace offset overflow"
                    )
                })?;

        /*
         * Leave the length field zero until commit_frame().
         *
         * In particular, don't expose a valid-looking header
         * before the payload has been completely encoded.
         */
        self.mmap
            .range_mut(
                self.offset,
                HEADER_SIZE,
            )
            .fill(0);

        Ok(
            WritableFrame {
                start:
                self.offset,

                payload_start,

                payload_len,

                total_len,

                next_offset,
            }
        )
    }

    pub(crate) fn payload_mut(
        &mut self,
        frame: WritableFrame,
    ) -> &mut [u8] {
        debug_assert_eq!(
            frame.start,
            self.offset
        );

        self.mmap.range_mut(
            frame.payload_start,
            frame.payload_len,
        )
    }

    pub(crate) fn commit_frame(
        &mut self,
        frame: WritableFrame,
    ) -> Result<(), WriterError> {
        if frame.start
            != self.offset
        {
            return Err(
                invalid_data(
                    "attempted to commit a stale trace frame"
                )
            );
        }

        let payload_end =
            HEADER_SIZE
                .checked_add(
                    frame.payload_len
                )
                .ok_or_else(|| {
                    invalid_data(
                        "payload end overflow"
                    )
                })?;

        if payload_end
            > frame.total_len
        {
            return Err(
                invalid_data(
                    "payload exceeds frame length"
                )
            );
        }

        let payload_len =
            u64::try_from(
                frame.payload_len
            )?;

        {
            let bytes =
                self.mmap.range_mut(
                    frame.start,
                    frame.total_len,
                );

            /*
             * Reserved header bytes.
             */
            bytes[
                8..
                    HEADER_SIZE
                ]
                .fill(0);

            /*
             * Alignment padding.
             */
            bytes[
                payload_end..
                    frame.total_len
                ]
                .fill(0);

            /*
             * Commit the frame last by publishing payload_len.
             */
            bytes[..8]
                .copy_from_slice(
                    &payload_len
                        .to_le_bytes()
                );
        }

        self.offset =
            frame.next_offset;

        Ok(())
    }

    pub(crate) fn abort_frame(
        &mut self,
        frame: WritableFrame,
    ) -> Result<(), WriterError> {
        if frame.start
            != self.offset
        {
            return Err(
                invalid_data(
                    "attempted to abort a stale trace frame"
                )
            );
        }

        /*
         * prepare_frame() may have provisioned another mmap
         * window. Restore physical EOF to the last committed
         * record.
         */
        self.mmap.truncate(
            frame.start
        )?;

        Ok(())
    }

    pub(crate) fn flush(
        &mut self,
    ) -> Result<(), WriterError> {
        /*
         * MAP_SHARED contains no Rust-side buffered output.
         *
         * Provisioning deliberately extends through the whole
         * current MMAP_SIZE window, so trim the unused tail back
         * to the logical trace length.
         */
        self.mmap.truncate(
            self.offset
        )?;

        Ok(())
    }
}
