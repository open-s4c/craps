#[cfg(feature = "serde-json")]
pub mod serde_json;
#[cfg(feature = "abomonation")]
pub mod abomonation;
#[cfg(feature = "rkyv")]
pub mod rkyv;

#[cfg(any(
    feature = "abomonation",
    feature = "rkyv",
))]
mod framed;

use std::error::Error;
use std::iter::Peekable;

pub(crate) type ReaderError = Box<dyn Error + Send + Sync>;

pub(crate) trait Reader<T: ?Sized> {
    fn next(
        &mut self,
    ) -> Result<Option<&T>, ReaderError>;

    fn peek(
        &mut self,
    ) -> Result<Option<&T>, ReaderError>;
}

#[cfg(feature = "serde-json")]
pub(crate) use serde_json::from_path;

#[cfg(feature = "abomonation")]
pub(crate) use abomonation::from_path;

#[cfg(feature = "rkyv")]
pub(crate) use rkyv::from_path;

