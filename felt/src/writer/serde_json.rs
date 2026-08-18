use std::{
    fs::File,
    io::Write as IoWrite,
};

use serde::Serialize;

use super::{
    Writer,
    WriterError,
};

struct JsonWriter {
    file: File,
}

impl<T> Writer<T> for JsonWriter
where
    T: Serialize,
{
    fn write(
        &mut self,
        value: &T,
    ) -> Result<(), WriterError> {
        ::serde_json::to_writer(
            &mut self.file,
            value,
        )?;

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
    T: Serialize + 'static,
{
    Ok(JsonWriter {
        file: File::create(path)?,
    })
}
