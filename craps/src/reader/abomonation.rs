use std::{
    any::type_name,
    fs::File,
    io::{
        Error,
        ErrorKind,
    },
    marker::PhantomData,
    mem::align_of,
};

use ::abomonation::Abomonation;

use memmap2::{
    MmapMut,
    MmapOptions,
};

use super::{
    Reader,
    ReaderError,
    framed::{
        Frame,
        frame_at,
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

pub(crate) struct AbomonationReader<T> {
    // Deliberately retained so the trace FD stays open.
    _file: File,

    // Abomonation needs mutable bytes for pointer fixups.
    // This is a private COW mapping.
    mmap: Option<MmapMut>,

    offset: usize,

    // If peek() already decoded the current frame,
    // next() must not decode it a second time.
    decoded_offset: Option<usize>,

    _marker: PhantomData<fn() -> T>,
}

impl<T> AbomonationReader<T>
where
    T: Abomonation,
{
    fn bytes(&self) -> &[u8] {
        self.mmap
            .as_deref()
            .unwrap_or(&[])
    }

    fn root_ptr(
        &self,
        frame: Frame,
    ) -> *const T {
        // SAFETY:
        // frame_at verified the offset lies in the mmap.
        unsafe {
            self.bytes()
                .as_ptr()
                .add(frame.payload_start)
                .cast::<T>()
        }
    }

    fn ensure_decoded(
        &mut self,
        frame: Frame,
    ) -> Result<*const T, ReaderError> {
        if self.decoded_offset
            == Some(frame.payload_start)
        {
            return Ok(self.root_ptr(frame));
        }

        let root_ptr = self.root_ptr(frame);

        // The framing guarantees 16-byte payload offsets,
        // but check the concrete root type as well.
        if (root_ptr as usize) % align_of::<T>() != 0 {
            return Err(invalid_data(format!(
                "Abomonation payload is not aligned for {}",
                type_name::<T>(),
            )));
        }

        let ptr = {
            let mmap = self
                .mmap
                .as_mut()
                .expect(
                    "a non-empty frame requires an mmap",
                );

            let payload =
                &mut mmap[
                    frame.payload_start
                        ..frame.payload_end
                    ];

            // SAFETY:
            //
            // - payload was produced by the matching
            //   Abomonation writer,
            // - it belongs to this reader's private COW mmap,
            // - the root alignment was checked above,
            // - the mmap remains alive while the returned
            //   reference may be used.
            //
            // Abomonation still has additional inherent
            // safety/alignment limitations documented by
            // the crate itself.
            let (value, remaining) =
                unsafe {
                    ::abomonation::decode::<T>(
                        payload,
                    )
                }
                    .ok_or_else(|| {
                        invalid_data(
                            "Abomonation decode failed",
                        )
                    })?;

            if !remaining.is_empty() {
                return Err(invalid_data(
                    "Abomonation frame was not fully consumed",
                ));
            }

            value as *const T
        };

        self.decoded_offset =
            Some(frame.payload_start);

        Ok(ptr)
    }
}

impl<T> Reader<T> for AbomonationReader<T>
where
    T: Abomonation,
{
    fn peek(
        &mut self,
    ) -> Result<Option<&T>, ReaderError> {
        let Some(frame) =
            frame_at(self.bytes(), self.offset)?
        else {
            return Ok(None);
        };

        let ptr = self.ensure_decoded(frame)?;

        // SAFETY:
        // `ptr` points inside self.mmap, and the returned
        // lifetime is tied to the mutable borrow of self.
        Ok(Some(
            unsafe { &*ptr }
        ))
    }

    fn next(
        &mut self,
    ) -> Result<Option<&T>, ReaderError> {
        let Some(frame) =
            frame_at(self.bytes(), self.offset)?
        else {
            return Ok(None);
        };

        let ptr = self.ensure_decoded(frame)?;

        self.offset = frame.next_offset;

        // The next frame has not been decoded yet.
        self.decoded_offset = None;

        // SAFETY:
        // Same reasoning as peek().
        Ok(Some(
            unsafe { &*ptr }
        ))
    }
}

pub(crate) fn from_path<T>(
    path: &str,
) -> Result<
    impl Reader<T> + Sync + 'static,
    ReaderError,
>
where
    T: Abomonation + 'static,
{
    let file = File::open(path)?;

    let mmap = if file.metadata()?.len() == 0 {
        None
    } else {
        Some(
            // SAFETY:
            // The backing trace file must not be externally
            // modified or truncated while this mapping exists.
            unsafe {
                MmapOptions::new()
                    .map_copy(&file)?
            }
        )
    };

    Ok(AbomonationReader {
        _file: file,
        mmap,
        offset: 0,
        decoded_offset: None,
        _marker: PhantomData,
    })
}
