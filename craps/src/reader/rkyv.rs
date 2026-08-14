use std::{
    fs::File,
    marker::PhantomData,
};

use memmap2::{
    Mmap,
    MmapOptions,
};

use ::rkyv::{
    Archive,
    api::high::HighValidator,
    bytecheck::CheckBytes,
    rancor::Error as RkyvError,
};

use super::{
    Reader,
    ReaderError,
    framed::{
        Frame,
        frame_at,
    },
};

pub(crate) struct RkyvReader<T>
where
    T: Archive,
{
    // Kept deliberately so the trace FD stays open.
    _file: File,

    mmap: Option<Mmap>,
    offset: usize,

    // We don't own a T. Avoid unnecessarily inheriting
    // T's Send/Sync properties.
    _marker: PhantomData<fn() -> T>,
}

impl<T> RkyvReader<T>
where
    T: Archive,
    T::Archived:
        for<'a> CheckBytes<
        HighValidator<'a, RkyvError>
    >,
{
    fn bytes(&self) -> &[u8] {
        self.mmap
            .as_deref()
            .unwrap_or(&[])
    }

    fn view(
        &self,
        frame: Frame,
    ) -> Result<&T::Archived, ReaderError> {
        let payload =
            &self.bytes()[
                frame.payload_start
                    ..frame.payload_end
                ];

        Ok(
            ::rkyv::access::<
                T::Archived,
                RkyvError,
            >(payload)?
        )
    }
}

impl<T> Reader<T::Archived> for RkyvReader<T>
where
    T: Archive,
    T::Archived:
        for<'a> CheckBytes<
        HighValidator<'a, RkyvError>
    >,
{
    fn peek(
        &mut self,
    ) -> Result<Option<&T::Archived>, ReaderError> {
        let Some(frame) =
            frame_at(self.bytes(), self.offset)?
        else {
            return Ok(None);
        };

        Ok(Some(self.view(frame)?))
    }

    fn next(
        &mut self,
    ) -> Result<Option<&T::Archived>, ReaderError> {
        let Some(frame) =
            frame_at(self.bytes(), self.offset)?
        else {
            return Ok(None);
        };

        self.offset = frame.next_offset;

        Ok(Some(self.view(frame)?))
    }
}

pub(crate) fn from_path<T>(
    path: &str,
) -> Result<
    impl Reader<T::Archived> + Sync + 'static,
    ReaderError,
>
where
    T: Archive + 'static,
    T::Archived:
        for<'a> CheckBytes<
        HighValidator<'a, RkyvError>
    >
    + 'static,
{
    let file = File::open(path)?;

    // mmap of an empty file is not useful/portable,
    // so represent an empty trace with None.
    let mmap = if file.metadata()?.len() == 0 {
        None
    } else {
        Some(
            // SAFETY:
            // The trace file must not be externally modified or
            // truncated while this mapping is alive.
            unsafe {
                MmapOptions::new().map(&file)?
            }
        )
    };

    Ok(RkyvReader {
        _file: file,
        mmap,
        offset: 0,
        _marker: PhantomData,
    })
}
