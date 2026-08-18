use std::{
    fs::File,
    io::Write as IoWrite,
};

use ::abomonation::Abomonation;

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

struct AbomonationWriter {
    file: File,

    /// Shadow VMA only. Its bytes are never read or written.
    opaque_mmap: OpaqueMmap,

    /// Current logical trace-file position.
    offset: u64,
}

impl<T> Writer<T> for AbomonationWriter
where
    T: Abomonation,
{
    fn write(
        &mut self,
        value: &T,
    ) -> Result<(), WriterError> {
        let mut payload = Vec::new();

        // SAFETY:
        // Required by Abomonation's serialization API.
        unsafe {
            ::abomonation::encode(
                value,
                &mut payload,
            )?;
        }

        let total_len = frame_len(payload.len())
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "trace frame length overflow",
                )
            })?;

        /*
         * Mirror the reader's two-stage mmap behavior.
         *
         * Stage 1:
         * The reader first has to see the frame header.
         */
        self.opaque_mmap.ensure_range(
            self.offset,
            HEADER_SIZE,
        )?;

        let payload_len =
            u64::try_from(payload.len())?;

        // Write directly to the file. The mmap is deliberately
        // untouched.
        self.file.write_all(
            &payload_len.to_le_bytes()
        )?;

        self.file.write_all(&[0; 8])?;

        /*
         * Stage 2:
         * Once the reader knows payload_len, it checks whether the
         * entire frame fits in the current mmap. Mirror that here.
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
    T: Abomonation + 'static,
{
    let file = File::create(path)?;

    Ok(AbomonationWriter {
        file,
        opaque_mmap:
        OpaqueMmap::new()?,
        offset: 0,
    })
}
