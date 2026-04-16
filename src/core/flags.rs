use super::error::FlagParseError;
use crate::internal::bitflag::define_bitflag;

define_bitflag!(
  /// Implementation of the cue sheet FLAG command specifications
  pub TrackFlag u8,
  values = [
    /// Digital copy permitted
    (DCP, 1),
    /// Four channel audio
    (FOUR_CHANNEL, 1 << 1),
    /// Pre-emphasis enabled on audio track
    (PRE, 1 << 2),
    /// Serial copy management system
    (SCMS, 1 << 3)
  ]
);

impl core::str::FromStr for TrackFlag {
  type Err = FlagParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    const TABLE: [(&str, TrackFlag); 4] = [
      ("DCP", TrackFlag::DCP),
      ("4CH", TrackFlag::FOUR_CHANNEL),
      ("SCMS", TrackFlag::SCMS),
      ("PRE", TrackFlag::PRE),
    ];

    for (name, value) in TABLE {
      if name.eq_ignore_ascii_case(s) {
        return Ok(value);
      }
    }

    Err(FlagParseError)
  }
}

impl TrackFlag {
  const fn as_str(&self) -> Option<&'static str> {
    match *self {
      TrackFlag::DCP => Some("DCP"),
      TrackFlag::FOUR_CHANNEL => Some("4CH"),
      TrackFlag::SCMS => Some("SCMS"),
      TrackFlag::PRE => Some("PRE"),
      _ => None,
    }
  }

  /// Iterates over flags names
  ///
  /// ```
  /// use cue_lib::core::TrackFlag;
  ///
  /// let flags = TrackFlag::DCP | TrackFlag::SCMS;
  ///
  /// for flag_name in flags.iter() {
  ///   println!("{}", flag_name)
  /// }
  /// ```
  pub const fn iter(&self) -> TrackFlagNameIter {
    TrackFlagNameIter {
      inner: *self,
      mask: TrackFlag(1),
    }
  }
}

pub struct TrackFlagNameIter {
  inner: TrackFlag,
  mask: TrackFlag,
}

impl Iterator for TrackFlagNameIter {
  type Item = &'static str;

  fn next(&mut self) -> Option<Self::Item> {
    const EMPTY: TrackFlag = TrackFlag(0);

    while let flag = (self.inner & self.mask)
      && self.mask != EMPTY
    {
      self.mask <<= 1;
      let name = flag.as_str();

      if name.is_some() {
        return name;
      }
    }

    None
  }
}
