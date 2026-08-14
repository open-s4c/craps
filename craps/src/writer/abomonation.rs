use std::{
    fs::File,
    io::{BufWriter, Write as IoWrite},
};

use abomonation::Abomonation;

use super::{Writer, WriterError};

const ALIGNMENT: usize = 16;

struct AbomonationWriter<W> {
    inner: W,
}

impl<W> AbomonationWriter<W> {
    fn new(inner: W) -> Self {
        Self { inner }
    }
}

fn padding_len(payload_len: usize) -> usize {
    (ALIGNMENT - (payload_len % ALIGNMENT)) % ALIGNMENT
}

impl<T, W> Writer<T> for AbomonationWriter<W>
where
    T: Abomonation,
    W: IoWrite,
{
    fn write(&mut self, value: &T) -> Result<(), WriterError> {
        let mut payload = Vec::new();

        // SAFETY:
        // Required by Abomonation's serialization API.
        unsafe {
            abomonation::encode(value, &mut payload)?;
        }

        let payload_len = u64::try_from(payload.len())?;

        // 16-byte header -> root value starts at aligned offset.
        self.inner.write_all(&payload_len.to_le_bytes())?;
        self.inner.write_all(&[0; 8])?;

        self.inner.write_all(&payload)?;

        let padding = padding_len(payload.len());

        if padding != 0 {
            self.inner.write_all(&[0; ALIGNMENT][..padding])?;
        }

        Ok(())
    }

    fn flush(&mut self) -> Result<(), WriterError> {
        self.inner.flush()?;
        Ok(())
    }
}

pub fn from_path<T>(
    path: &str,
) -> Result<impl Writer<T> + Sync + 'static, WriterError>
where
    T: Abomonation + 'static,
{
    let file = File::create(path)?;

    Ok(AbomonationWriter::new(
        BufWriter::new(file),
    ))
}
