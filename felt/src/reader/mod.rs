#[cfg(feature = "serde-json")]
pub mod serde_json;
#[cfg(feature = "abomonation")]
pub mod abomonation;
#[cfg(any(
    feature = "abomonation",
))]
mod mapped;

use std::error::Error;

pub type ReaderError = Box<dyn Error + Send + Sync>;

pub trait Reader<T: ?Sized> {
    fn next(
        &mut self,
    ) -> Result<Option<&T>, ReaderError>;

    fn peek(
        &mut self,
    ) -> Result<Option<&T>, ReaderError>;
}

#[cfg(feature = "serde-json")]
pub use serde_json::from_path;

#[cfg(feature = "abomonation")]
pub use abomonation::from_path;
