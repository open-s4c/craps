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
mod opaque_mmap;

use std::error::Error;

pub type WriterError =
Box<dyn Error + Send + Sync>;

pub trait Writer<T> {
    fn write(
        &mut self,
        value: &T,
    ) -> Result<(), WriterError>;

    fn flush(
        &mut self,
    ) -> Result<(), WriterError>;
}

#[cfg(feature = "serde-json")]
pub use serde_json::from_path;

#[cfg(feature = "abomonation")]
pub use abomonation::from_path;

#[cfg(feature = "rkyv")]
pub use rkyv::from_path;
