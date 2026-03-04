pub mod error;

#[cfg(feature = "alloc")]
mod map;
mod vorbis;

#[cfg(feature = "alloc")]
pub use self::map::MetadataMap;
pub use self::vorbis::{VorbisComment, VorbisTagName};
