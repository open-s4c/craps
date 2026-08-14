use std::{
    fs::File,
    io::{BufWriter, Write as IoWrite},
};

use rkyv::{
    Serialize,
    api::high::HighSerializer,
    rancor::Error as RkyvError,
    ser::allocator::ArenaHandle,
    util::AlignedVec,
};

use super::{Writer, WriterError};

const ALIGNMENT: usize = 16;
const HEADER_SIZE: usize = 16;

struct RkyvWriter<W> {
    inner: W,
}

impl<W> RkyvWriter<W> {
    fn new(inner: W) -> Self {
        Self { inner }
    }
}

fn padding_len(payload_len: usize) -> usize {
    (ALIGNMENT - (payload_len % ALIGNMENT)) % ALIGNMENT
}

impl<T, W> Writer<T> for RkyvWriter<W>
where
    W: IoWrite,
    T: for<'a> Serialize<
        HighSerializer<
            AlignedVec,
            ArenaHandle<'a>,
            RkyvError,
        >,
    >,
{
    fn write(&mut self, value: &T) -> Result<(), WriterError> {
        let payload = rkyv::to_bytes::<RkyvError>(value)?;
        let payload_len = u64::try_from(payload.len())?;

        // 16-byte frame header.
        //
        // Because every frame starts aligned and the header itself
        // is 16 bytes, the payload starts at a 16-byte-aligned
        // file offset.
        self.inner.write_all(&payload_len.to_le_bytes())?;
        self.inner.write_all(&[0; HEADER_SIZE - size_of::<u64>()])?;

        self.inner.write_all(&payload)?;

        // Ensure that the next frame starts on another
        // 16-byte boundary.
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
    T: for<'a> Serialize<
        HighSerializer<
            AlignedVec,
            ArenaHandle<'a>,
            RkyvError,
        >,
    >,
{
    let file = File::create(path)?;

    Ok(RkyvWriter::new(
        BufWriter::new(file),
    ))
}
