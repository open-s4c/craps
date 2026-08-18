use std::{
    any::type_name,
    marker::PhantomData,
    mem::align_of,
    io::{
        Error,
        ErrorKind,
    },
};

use ::abomonation::Abomonation;

use super::{
    Reader,
    ReaderError,
    mapped::{
        Frame,
        MappedReader,
    },
};

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

pub struct AbomonationReader<T> {
    inner: MappedReader,

    // peek() decodes/fixes up the current frame.
    // next() must not repeat that operation.
    decoded_offset: Option<u64>,

    _marker: PhantomData<fn() -> T>,
}

impl<T> AbomonationReader<T>
where
    T: Abomonation,
{
    fn root_ptr(
        &self,
        frame: Frame,
    ) -> *const T {
        self.inner
            .payload(frame)
            .as_ptr()
            .cast::<T>()
    }

    fn ensure_decoded(
        &mut self,
        frame: Frame,
    ) -> Result<*const T, ReaderError> {
        if self.decoded_offset
            == Some(frame.start)
        {
            return Ok(
                self.root_ptr(frame)
            );
        }

        let root_ptr =
            self.root_ptr(frame);

        if (root_ptr as usize)
            % align_of::<T>()
            != 0
        {
            return Err(
                invalid_data(format!(
                    "Abomonation payload is not aligned for {}",
                    type_name::<T>(),
                ))
            );
        }

        let ptr = {
            let payload =
                self.inner.payload_mut(frame);

            let (
                value,
                remaining,
            ) = unsafe {
                ::abomonation::decode::<T>(
                    payload,
                )
            }
                .ok_or_else(|| {
                    invalid_data(
                        "Abomonation decode failed"
                    )
                })?;

            if !remaining.is_empty() {
                return Err(
                    invalid_data(
                        "Abomonation frame was not fully consumed"
                    )
                );
            }

            value as *const T
        };

        self.decoded_offset =
            Some(frame.start);

        Ok(ptr)
    }
}

impl<T> Reader<T>
for AbomonationReader<T>
where
    T: Abomonation,
{
    fn peek(
        &mut self,
    ) -> Result<Option<&T>, ReaderError> {
        let Some(frame) =
            self.inner.frame()?
        else {
            return Ok(None);
        };

        let ptr =
            self.ensure_decoded(frame)?;

        Ok(Some(
            unsafe { &*ptr }
        ))
    }

    fn next(
        &mut self,
    ) -> Result<Option<&T>, ReaderError> {
        let Some(frame) =
            self.inner.frame()?
        else {
            return Ok(None);
        };

        let ptr =
            self.ensure_decoded(frame)?;

        self.inner.advance(frame);

        self.decoded_offset = None;

        Ok(Some(
            unsafe { &*ptr }
        ))
    }
}

pub fn from_path<T>(
    path: &str,
) -> Result<
    impl Reader<T> + Sync + 'static,
    ReaderError,
>
where
    T: Abomonation + 'static,
{
    Ok(AbomonationReader {
        inner:
        MappedReader::from_path(path)?,

        decoded_offset: None,
        _marker: PhantomData,
    })
}
