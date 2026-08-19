use std::{
    fs::File,
    io::{
        BufWriter,
        Write,
    },
};

use serde::Serialize;

use super::{
    Writer,
    WriterError,
};

pub struct JsonWriter {
    writer: BufWriter<File>,
}

impl<T> Writer<T> for JsonWriter
where
    T: Serialize,
{
    fn write(
        &mut self,
        value: &T,
    ) -> Result<(), WriterError> {
        serde_json::to_writer(
            &mut self.writer,
            value,
        )?;

        Ok(())
    }

    fn flush(
        &mut self,
    ) -> Result<(), WriterError> {
        self.writer.flush()?;
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
    T: Serialize + 'static,
{
    Ok(JsonWriter {
        writer: BufWriter::new(
            File::create(path)?
        ),
    })
}
