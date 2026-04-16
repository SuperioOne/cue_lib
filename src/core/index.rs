use super::timestamp::CueTimestamp;
use crate::internal::range::impl_numeric_range_type;

/// Index no between 0 and 255
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct IndexNo(u8);

impl_numeric_range_type!(IndexNo, u8, max = 255, len = 3, display_leading_zeros = 2);

/// Indexes in the cue sheet
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Clone, Copy, Debug)]
pub struct TrackIndex {
  pub no: IndexNo,
  pub timestamp: CueTimestamp,
}
