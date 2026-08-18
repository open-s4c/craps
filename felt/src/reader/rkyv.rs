use std::marker::PhantomData;

use ::rkyv::{
    Archive,
    Portable,
    api::high::HighValidator,
    bytecheck::CheckBytes,
    rancor::Error as RkyvError,
};

use super::{
    Reader,
    ReaderError,
    mapped::MappedReader,
};

pub struct RkyvReader<T>
where
    T: Archive,
{
    inner: MappedReader,

    _marker: PhantomData<fn() -> T>,
}

impl<T> Reader<T::Archived>
for RkyvReader<T>
where
    T: Archive,

    T::Archived:
    Portable
    + for<'a> CheckBytes<
        HighValidator<'a, RkyvError>
    >,
{
    fn next(
        &mut self,
    ) -> Result<
        Option<&T::Archived>,
        ReaderError,
    > {
        let Some(frame) =
            self.inner.frame()?
        else {
            return Ok(None);
        };

        // Advance before borrowing the mmap for the
        // returned reference.
        self.inner.advance(frame);

        let payload =
            self.inner.payload(frame);

        let archived =
            ::rkyv::access::<
                T::Archived,
                RkyvError,
            >(payload)?;

        Ok(Some(archived))
    }

    fn peek(
        &mut self,
    ) -> Result<
        Option<&T::Archived>,
        ReaderError,
    > {
        let Some(frame) =
            self.inner.frame()?
        else {
            return Ok(None);
        };

        let payload =
            self.inner.payload(frame);

        let archived =
            ::rkyv::access::<
                T::Archived,
                RkyvError,
            >(payload)?;

        Ok(Some(archived))
    }
}

pub fn from_path<T>(
    path: &str,
) -> Result<
    impl Reader<T::Archived> + Sync + 'static,
    ReaderError,
>
where
    T: Archive + 'static,

    T::Archived:
    Portable
    + for<'a> CheckBytes<
        HighValidator<'a, RkyvError>
    >
    + 'static,
{
    Ok(RkyvReader::<T> {
        inner:
        MappedReader::from_path(path)?,

        _marker: PhantomData,
    })
}
