use std::{
    fs::File,
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

pub(crate) struct MappedReader {
    /*
     * Physical trace length.
     *
     * The mapping may extend beyond this length, but those bytes
     * are never accessed.
     */
    file_len: u64,

    /*
     * The fixed-address MAP_PRIVATE mapping.
     */
    mmap: FixedFileMapper,

    /*
     * Absolute file offset of the next frame.
     */
    cursor: u64,
}

impl MappedReader {
    pub(crate) fn from_path(
        path: &str,
    ) -> Result<Self, ReaderError> {
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

        let file =
            File::open(path)?;

        let file_len =
            file.metadata()?.len();

        /*
         * Establish the full fixed-size mapping immediately.
         */
        let mmap =
            FixedFileMapper::new_private(
                file
            )?;

        Ok(Self {
            file_len,
            mmap,
            cursor: 0,
        })
    }

    pub(crate) fn payload(
        &self,
        frame: Frame,
    ) -> &[u8] {
        self.mmap.range(
            frame.payload_start,
            frame.payload_len,
        )
    }

    pub(crate) fn payload_mut(
        &mut self,
        frame: Frame,
    ) -> &mut [u8] {
        self.mmap.range_mut(
            frame.payload_start,
            frame.payload_len,
        )
    }

    pub(crate) fn frame(
        &mut self,
    ) -> Result<
        Option<Frame>,
        ReaderError,
    > {
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
                    u64::try_from(
                        HEADER_SIZE
                    )?
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
         *
         * Ensure the header is visible.
         */
        self.mmap.ensure_range(
            self.cursor,
            HEADER_SIZE,
        )?;

        let payload_len = {
            let header =
                self.mmap.range(
                    self.cursor,
                    HEADER_SIZE,
                );

            let raw =
                u64::from_le_bytes(
                    header[..8]
                        .try_into()
                        .expect(
                            "header contains 8 length bytes"
                        )
                );

            usize::try_from(raw)?
        };

        let total_len =
            frame_len(
                payload_len
            )
                .ok_or_else(|| {
                    invalid_data(
                        "frame length overflow"
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
         * The mmap can extend past EOF, but bytes beyond file_len
         * are never dereferenced.
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
         *
         * Ensure the complete frame is simultaneously visible in
         * the fixed-size mapping.
         */
        self.mmap.ensure_range(
            self.cursor,
            total_len,
        )?;

        Ok(Some(
            Frame {
                start:
                self.cursor,

                payload_start:
                self.cursor
                    .checked_add(
                        u64::try_from(
                            HEADER_SIZE
                        )?
                    )
                    .ok_or_else(|| {
                        invalid_data(
                            "payload offset overflow"
                        )
                    })?,

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
