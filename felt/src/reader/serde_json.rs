use std::{
    fs::File,
    io::BufReader,
};

use serde::de::DeserializeOwned;
use ::serde_json::{
    de::IoRead,
    Deserializer,
    StreamDeserializer,
};

use super::{
    Reader,
    ReaderError,
};

pub struct SerdeJsonReader<T> {
    iter: StreamDeserializer<
        'static,
        IoRead<BufReader<File>>,
        T,
    >,

    // Value consumed from `iter` by peek(), but not yet consumed
    // logically by next().
    peeked: Option<T>,

    // Most recently returned value. It has to live inside `self`
    // because next() returns a reference to it.
    current: Option<T>,
}

impl<T> Reader<T> for SerdeJsonReader<T>
where
    T: DeserializeOwned,
{
    fn next(
        &mut self,
    ) -> Result<Option<&T>, ReaderError> {
        if let Some(value) = self.peeked.take() {
            self.current = Some(value);

            return Ok(self.current.as_ref());
        }

        match self.iter.next() {
            None => {
                self.current = None;
                Ok(None)
            }

            Some(Ok(value)) => {
                self.current = Some(value);
                Ok(self.current.as_ref())
            }

            Some(Err(error)) => {
                self.current = None;
                Err(Box::new(error))
            }
        }
    }

    fn peek(
        &mut self,
    ) -> Result<Option<&T>, ReaderError> {
        if self.peeked.is_none() {
            match self.iter.next() {
                None => {
                    return Ok(None);
                }

                Some(Ok(value)) => {
                    self.peeked = Some(value);
                }

                Some(Err(error)) => {
                    return Err(Box::new(error));
                }
            }
        }

        Ok(self.peeked.as_ref())
    }
}

pub fn from_path<T>(
    path: &str,
) -> Result<SerdeJsonReader<T>, ReaderError>
where
    T: DeserializeOwned + 'static,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let iter =
        Deserializer::from_reader(reader)
            .into_iter::<T>();

    Ok(SerdeJsonReader {
        iter,
        peeked: None,
        current: None,
    })
}
