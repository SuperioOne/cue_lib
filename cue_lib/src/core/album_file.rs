use super::{cue_str::CueStr, error::UnknownFileType};
use crate::internal::enum_str::impl_enum_str;

impl_enum_str!(
  pub KnownFileType, parse_error = UnknownFileType,
  values = [
    ///Intel binary file (least significant byte first)
    (Binary, "BINARY"),

    /// Motorola binary file (most significant byte first)
    (Motorola, "MOTOROLA"),

    /// Audio AIFF file
    (AIFF, "AIFF"),

    /// Audio WAVE file
    (WAVE, "WAVE"),

    /// Audio MP3 file
    (MP3, "MP3"),

    /// Audio FLAC file. (Extension over the original cuesheet format)
    (FLAC, "FLAC")
  ]
);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub struct AlbumFile<'a> {
  pub file_type: KnownFileType,
  pub name: CueStr<'a>,
}
