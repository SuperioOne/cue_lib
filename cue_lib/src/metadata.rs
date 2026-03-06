pub mod error;

#[cfg(feature = "alloc")]
mod map;
#[cfg(feature = "serde")]
mod serde;
mod vorbis;

#[cfg(feature = "alloc")]
pub use self::map::MetadataMap;
pub use self::vorbis::{VorbisComment, VorbisTagName};
