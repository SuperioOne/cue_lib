use super::error::{TimeStampParseError, TimestampParseErrorKind};
use crate::internal::range::impl_numeric_range_type;
use core::time::Duration;

const SECONDS: u128 = 1000;
const MINUTE: u128 = 60 * 1000;
const FRAME: u128 = 1000 / 75;

/// Value between 0 and 59
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Second(u8);

/// Value between 0 and 74.
/// For conversion: 75 frames are 1 second.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Frame(u8);

impl_numeric_range_type!(Second, u8, max = 59, len = 2, display_leading_zeros = 2);
impl_numeric_range_type!(Frame, u8, max = 74, len = 2, display_leading_zeros = 2);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Default)]
pub struct CueTimestamp {
  minute: u64,
  second: u8,
  frame: u8,
}

impl CueTimestamp {
  pub const fn new(minute: u64, second: Second, frame: Frame) -> Self {
    Self {
      frame: frame.into_inner(),
      minute,
      second: second.into_inner(),
    }
  }

  #[inline]
  pub const fn as_millis(&self) -> u128 {
    (self.second as u128) * SECONDS + (self.minute as u128) * MINUTE + (self.frame as u128) * FRAME
  }

  #[inline]
  pub fn as_duration(&self) -> Duration {
    Duration::from_mins(self.minute)
      + Duration::from_secs(self.second as u64)
      + Duration::from_millis(self.frame as u64 * FRAME as u64)
  }

  #[inline]
  pub const fn from_millis(value: u128) -> Self {
    let mut remaining_ms = value;

    let minute = remaining_ms.saturating_div(MINUTE);
    remaining_ms = minute * MINUTE;

    let second = remaining_ms.saturating_div(SECONDS);
    remaining_ms = second * SECONDS;

    let frame = remaining_ms / FRAME;

    Self {
      minute: minute as u64,
      second: second as u8,
      frame: frame as u8,
    }
  }
}

impl Ord for CueTimestamp {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    match self.minute.cmp(&other.minute) {
      core::cmp::Ordering::Equal => {}
      order => return order,
    };

    match self.second.cmp(&other.second) {
      core::cmp::Ordering::Equal => {}
      order => return order,
    };

    self.frame.cmp(&other.frame)
  }
}

impl PartialOrd for CueTimestamp {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl From<Duration> for CueTimestamp {
  #[inline]
  fn from(value: Duration) -> Self {
    Self::from_millis(value.as_millis())
  }
}

impl core::fmt::Display for CueTimestamp {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!(
      "{mm:0>2}:{ss:0>2}:{ff:0>2}",
      mm = self.minute,
      ss = self.second,
      ff = self.frame
    ))
  }
}

impl Into<Duration> for CueTimestamp {
  #[inline]
  fn into(self) -> Duration {
    self.as_duration()
  }
}

impl core::str::FromStr for CueTimestamp {
  type Err = TimeStampParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // 0:00:00
    if s.len() < 7 {
      return Err(TimeStampParseError::new(
        TimestampParseErrorKind::InvalidLength,
      ));
    }

    let second_start = s.len() - 6;
    let frame_start = s.len() - 3;

    let second: Second = match s.as_bytes().get(second_start) {
      Some(b':') => Second::from_str(&s[(second_start + 1)..frame_start])
        .map_err(|_| TimeStampParseError::new(TimestampParseErrorKind::InvalidSecond))?,
      _ => {
        return Err(TimeStampParseError::new(
          TimestampParseErrorKind::InvalidCharacter,
        ));
      }
    };

    let frame: Frame = match s.as_bytes().get(frame_start) {
      Some(b':') => Frame::from_str(&s[(frame_start + 1)..])
        .map_err(|_| TimeStampParseError::new(TimestampParseErrorKind::InvalidFrame))?,
      _ => {
        return Err(TimeStampParseError::new(
          TimestampParseErrorKind::InvalidCharacter,
        ));
      }
    };

    let minute = u64::from_str_radix(&s[..second_start], 10)
      .map_err(|_| TimeStampParseError::new(TimestampParseErrorKind::InvalidMinute))?;

    Ok(Self {
      second: second.into_inner(),
      minute,
      frame: frame.into_inner(),
    })
  }
}
