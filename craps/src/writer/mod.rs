#[cfg(feature = "serde-json")]
pub mod serde_json;
#[cfg(feature = "abomonation")]
pub mod abomonation;
#[cfg(feature = "rkyv")]
pub mod rkyv;

use std::error::Error;

pub type WriterError = Box<dyn Error + Send + Sync>;

pub trait Writer<T> {
    fn write(&mut self, value: &T) -> Result<(), WriterError>;

    fn flush(&mut self) -> Result<(), WriterError>;
}

#[cfg(feature = "serde-json")]
pub(crate) use serde_json::from_path;

#[cfg(feature = "abomonation")]
pub(crate) use abomonation::from_path;

#[cfg(feature = "rkyv")]
pub(crate) use rkyv::from_path;
