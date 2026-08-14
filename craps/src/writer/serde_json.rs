use std::{
    fs::File,
    io::{BufWriter, Write as IoWrite},
};

use serde::Serialize;

use super::{Writer, WriterError};

struct JsonWriter<W> {
    inner: W,
}

impl<W> JsonWriter<W> {
    fn new(inner: W) -> Self {
        Self { inner }
    }
}

impl<T, W> Writer<T> for JsonWriter<W>
where
    T: Serialize,
    W: IoWrite,
{
    fn write(&mut self, value: &T) -> Result<(), WriterError> {
        ::serde_json::to_writer(&mut self.inner, value)?;
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
    T: Serialize + 'static,
{
    let file = File::create(path)?;
    let writer = BufWriter::new(file);

    Ok(JsonWriter::new(writer))
}
