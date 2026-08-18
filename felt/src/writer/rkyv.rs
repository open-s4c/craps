use std::{
    fs::File,
    io::Write as IoWrite,
};

use ::rkyv::{
    Serialize,
    api::high::HighSerializer,
    rancor::Error as RkyvError,
    ser::allocator::ArenaHandle,
    util::AlignedVec,
};

use crate::{
    ALIGNMENT,
    HEADER_SIZE,
    frame_len,
    padding_len,
};

use super::{
    Writer,
    WriterError,
    opaque_mmap::OpaqueMmap,
};

struct RkyvWriter {
    file: File,

    /// Shadow VMA only. Its bytes are never read or written.
    opaque_mmap: OpaqueMmap,

    /// Current logical trace-file position.
    offset: u64,
}

impl<T> Writer<T> for RkyvWriter
where
    T: for<'a> Serialize<
        HighSerializer<
            AlignedVec,
            ArenaHandle<'a>,
            RkyvError,
        >,
    >,
{
    fn write(
        &mut self,
        value: &T,
    ) -> Result<(), WriterError> {
        let payload =
            ::rkyv::to_bytes::<RkyvError>(
                value
            )?;

        let total_len = frame_len(payload.len())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "trace frame length overflow",
                )
            })?;

        /*
         * Stage 1:
         * Mirror the reader ensuring that the header is mapped.
         */
        self.opaque_mmap.ensure_range(
            self.offset,
            HEADER_SIZE,
        )?;

        let payload_len =
            u64::try_from(payload.len())?;

        // Actual output goes straight to the file.
        self.file.write_all(
            &payload_len.to_le_bytes()
        )?;

        self.file.write_all(&[0; 8])?;

        /*
         * Stage 2:
         * After seeing payload_len, the reader checks whether the
         * entire frame fits its fixed mmap. Mirror that operation.
         */
        self.opaque_mmap.ensure_range(
            self.offset,
            total_len,
        )?;

        self.file.write_all(&payload)?;

        let padding =
            padding_len(payload.len());

        if padding != 0 {
            self.file.write_all(
                &[0; ALIGNMENT][..padding]
            )?;
        }

        self.offset = self
            .offset
            .checked_add(
                u64::try_from(total_len)?
            )
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "trace offset overflow",
                )
            })?;

        Ok(())
    }

    fn flush(
        &mut self,
    ) -> Result<(), WriterError> {
        self.file.flush()?;
        Ok(())
    }
}

pub fn from_path<T>(
    path: &str,
) -> Result<
    impl Writer<T> + Sync + 'static,
    WriterError,
>
where
    T: for<'a> Serialize<
        HighSerializer<
            AlignedVec,
            ArenaHandle<'a>,
            RkyvError,
        >,
    >,
{
    let file = File::create(path)?;

    Ok(RkyvWriter {
        file,
        opaque_mmap:
        OpaqueMmap::new()?,
        offset: 0,
    })
}
