use std::io::{
    Error,
    ErrorKind,
};

use super::ReaderError;

pub(crate) const ALIGNMENT: usize = 16;
pub(crate) const HEADER_SIZE: usize = 16;

#[derive(Clone, Copy)]
pub(crate) struct Frame {
    pub payload_start: usize,
    pub payload_end: usize,
    pub next_offset: usize,
}

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

pub(crate) fn frame_at(
    bytes: &[u8],
    offset: usize,
) -> Result<Option<Frame>, ReaderError> {
    if offset == bytes.len() {
        return Ok(None);
    }

    if offset > bytes.len() {
        return Err(invalid_data(
            "frame offset is past end of trace",
        ));
    }

    if offset % ALIGNMENT != 0 {
        return Err(invalid_data(
            "frame start is not aligned",
        ));
    }

    let header_end = offset
        .checked_add(HEADER_SIZE)
        .ok_or_else(|| {
            invalid_data("frame header overflow")
        })?;

    if header_end > bytes.len() {
        return Err(invalid_data(
            "truncated frame header",
        ));
    }

    let payload_len = u64::from_le_bytes(
        bytes[offset..offset + 8]
            .try_into()
            .expect("slice has exactly 8 bytes"),
    );

    let payload_len =
        usize::try_from(payload_len)?;

    let payload_start = header_end;

    let payload_end = payload_start
        .checked_add(payload_len)
        .ok_or_else(|| {
            invalid_data("frame payload overflow")
        })?;

    if payload_end > bytes.len() {
        return Err(invalid_data(
            "truncated frame payload",
        ));
    }

    let padding =
        (ALIGNMENT - (payload_len % ALIGNMENT))
            % ALIGNMENT;

    let next_offset = payload_end
        .checked_add(padding)
        .ok_or_else(|| {
            invalid_data("frame padding overflow")
        })?;

    if next_offset > bytes.len() {
        return Err(invalid_data(
            "truncated frame padding",
        ));
    }

    Ok(Some(Frame {
        payload_start,
        payload_end,
        next_offset,
    }))
}
