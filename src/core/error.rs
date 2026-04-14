#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnknownFileType;

impl core::fmt::Display for UnknownFileType {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("unknown file type")
  }
}

impl core::error::Error for UnknownFileType {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InvalidNumericRange;

impl core::fmt::Display for InvalidNumericRange {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("numeric range is invalid")
  }
}

impl core::error::Error for InvalidNumericRange {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DigitsParseError;

impl core::fmt::Display for DigitsParseError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("failed to parse digits")
  }
}

impl core::error::Error for DigitsParseError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DataTypeParseError;

impl core::fmt::Display for DataTypeParseError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("failed to parse data type")
  }
}

impl core::error::Error for DataTypeParseError {}

/// Represents an error when parsing a timestamp.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeStampParseError {
  kind: TimestampParseErrorKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FlagParseError;

impl core::fmt::Display for FlagParseError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("failed to parse flag")
  }
}

impl core::error::Error for FlagParseError {}

impl TimeStampParseError {
  #[inline]
  pub const fn new(kind: TimestampParseErrorKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub const fn kind(&self) -> TimestampParseErrorKind {
    self.kind
  }
}

/// Kinds of errors that can occur while parsing a timestamp.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimestampParseErrorKind {
  /// The timestamp has an invalid length.
  InvalidLength,
  /// A character in the timestamp is not valid.
  InvalidCharacter,
  /// The minute value is invalid.
  InvalidMinute,
  /// The second value is invalid.
  InvalidSecond,
  /// The frame value is invalid.
  InvalidFrame,
}

impl core::fmt::Display for TimestampParseErrorKind {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      TimestampParseErrorKind::InvalidLength => f.write_str("timestamp length is incorrect"),
      TimestampParseErrorKind::InvalidCharacter => {
        f.write_str("timestamp contains invalid character")
      }
      TimestampParseErrorKind::InvalidMinute => f.write_str("timestamp minute is invalid"),
      TimestampParseErrorKind::InvalidSecond => f.write_str("timestamp second is invalid"),
      TimestampParseErrorKind::InvalidFrame => f.write_str("timestamp frame is invalid"),
    }
  }
}

impl core::fmt::Display for TimeStampParseError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("invalid timestamp: {}", self.kind))
  }
}

impl core::error::Error for TimeStampParseError {}

/// Represents an error when parsing a cue string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CueStrError {
  kind: CueStrErrorKind,
}

impl CueStrError {
  #[inline]
  pub const fn new(kind: CueStrErrorKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub const fn kind(&self) -> CueStrErrorKind {
    self.kind
  }
}

/// Kinds of errors that can occur while parsing a cue string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CueStrErrorKind {
  /// Missing opening or closing quotes.
  MissingQuotes,
  /// Missing ending double quote.
  MissingEndingQuote,
  /// An unescaped special character was found.
  UnescapedSpecialChar,
  /// Str is not a valid UTF-8.
  InvalidUtf8,
}

impl From<CueStrErrorKind> for CueStrError {
  #[inline]
  fn from(value: CueStrErrorKind) -> Self {
    Self::new(value)
  }
}

impl core::fmt::Display for CueStrErrorKind {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::InvalidUtf8 => f.write_str("string is not a valid utf-8"),
      Self::MissingQuotes => f.write_str("string needs to be quoted"),
      Self::MissingEndingQuote => f.write_str("string is missing ending double quote"),
      Self::UnescapedSpecialChar => f.write_str("unescaped special character found"),
    }
  }
}

impl core::fmt::Display for CueStrError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("invalid cue string: {}", self.kind))
  }
}

impl core::error::Error for CueStrError {}
