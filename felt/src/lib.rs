pub mod writer;
pub mod reader;

pub(crate) const ALIGNMENT: usize = 16;
pub(crate) const HEADER_SIZE: usize = 16;

// Must be large enough to contain any individual serialized record.
pub(crate) const MMAP_SIZE: usize = 1024 * 1024;

pub(crate) fn padding_len(payload_len: usize) -> usize {
    (ALIGNMENT - (payload_len % ALIGNMENT)) % ALIGNMENT
}

pub(crate) fn frame_len(payload_len: usize) -> Option<usize> {
    HEADER_SIZE
        .checked_add(payload_len)?
        .checked_add(padding_len(payload_len))
}
