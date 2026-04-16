#[cfg(any(feature = "ean", feature = "upc"))]
mod checksum;

#[cfg(feature = "serde")]
pub mod error;
mod serde;

#[cfg(feature = "ean")]
pub mod ean;
#[cfg(feature = "upc")]
pub mod upc;

pub mod isrc;
