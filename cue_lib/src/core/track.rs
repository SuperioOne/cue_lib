use super::{error::DataTypeParseError, timestamp::CueTimestamp};
use crate::internal::{enum_str::impl_enum_str, range::impl_numeric_range_type};

impl_enum_str!(
  pub DataType, parse_error = DataTypeParseError,
  values = [
    /// Audio/Music
    (Audio, "AUDIO"),
    /// Karaoke CD+G
    (CDG, "CDG"),
    /// CD-ROM Mode1 Data (cooked)
    (Mode1_2048, "MODE1/2048"),
    /// CD-ROM Mode1 Data (raw)
    (Mode1_2352, "MODE1/2352"),
    /// CD-ROM XA Mode2 Data
    (Mode2_2336, "MODE2/2336"),
    /// CD-ROM XA Mode2 Data
    (Mode2_2352, "MODE2/2352"),
    /// CD-I Mode2 Data
    #[allow(nonstandard_style)]
    (CDI_2336, "CDI/2336"),
    /// CD-I Mode2 Data
    #[allow(nonstandard_style)]
    (CDI_2352, "CDI/2352")
  ]
);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TrackNo(u8);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndexNo(u8);

impl_numeric_range_type!(IndexNo, u8, max = 255, len = 3, display_leading_zeros = 2);
impl_numeric_range_type!(TrackNo, u8, max = 255, len = 3, display_leading_zeros = 2);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub struct TrackIndex {
  pub index_no: IndexNo,
  pub timestamp: CueTimestamp,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub struct Track {
  pub track_no: TrackNo,
  pub data_type: DataType,
}
