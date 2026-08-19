use std::io::{
    Error,
    ErrorKind,
};

use memmap2::{
    MmapMut,
    MmapOptions,
};

use crate::frame::MMAP_SIZE;

use super::WriterError;

fn invalid_data(
    message: impl Into<String>,
) -> WriterError {
    Box::new(
        Error::new(
            ErrorKind::InvalidData,
            message.into(),
        )
    )
}

/// An mmap whose contents are deliberately never accessed.
///
/// Its only purpose during recording is to reproduce the virtual
/// memory footprint/remapping behavior of the replay reader.
pub(crate) struct OpaqueMmap {
    mmap: Option<MmapMut>,

    /// Logical trace-file offset corresponding to the beginning
    /// of the current mmap window.
    logical_offset: u64,
}

impl OpaqueMmap {
    pub(crate) fn new() -> Result<Self, WriterError> {
        let mmap = MmapOptions::new()
            .len(MMAP_SIZE)
            .map_anon()?;

        Ok(Self {
            mmap: Some(mmap),
            logical_offset: 0,
        })
    }

    /// Ensures that `[offset, offset + len)` fits in the logical
    /// fixed-size mmap window.
    ///
    /// If it does not, the old mmap is unmapped and a new mmap of
    /// exactly MMAP_SIZE bytes is created. The new logical window
    /// begins at `offset`.
    pub(crate) fn ensure_range(
        &mut self,
        offset: u64,
        len: usize,
    ) -> Result<(), WriterError> {
        if len > MMAP_SIZE {
            return Err(
                invalid_data(format!(
                    "trace range of {len} bytes exceeds fixed \
                     mmap size of {MMAP_SIZE} bytes"
                ))
            );
        }

        let len = u64::try_from(len)?;

        let end = offset
            .checked_add(len)
            .ok_or_else(|| {
                invalid_data(
                    "trace range overflow"
                )
            })?;

        let window_end = self
            .logical_offset
            .checked_add(MMAP_SIZE as u64)
            .ok_or_else(|| {
                invalid_data(
                    "mmap window overflow"
                )
            })?;

        if offset >= self.logical_offset
            && end <= window_end
        {
            return Ok(());
        }

        self.remap(offset)
    }

    fn remap(
        &mut self,
        logical_offset: u64,
    ) -> Result<(), WriterError> {
        // Explicitly destroy the old mapping before creating
        // the new one. This avoids briefly having both VMAs alive.
        drop(self.mmap.take());

        let mmap = MmapOptions::new()
            .len(MMAP_SIZE)
            .map_anon()?;

        self.mmap = Some(mmap);
        self.logical_offset = logical_offset;

        Ok(())
    }
}
