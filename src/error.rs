use crate::{
  core::error::{
    CueStrError, DataTypeParseError, DigitsParseError, FlagParseError, InvalidNumericRange,
    TimeStampParseError, UnknownFileType,
  },
  discid::error::IsrcParseError,
  internal::tokenizer::Position,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CueLibError {
  kind: CueLibErrorKind,
}

impl CueLibError {
  #[inline]
  pub const fn kind(&self) -> CueLibErrorKind {
    self.kind
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CueLibErrorKind {
  ParseError(ParseError),
}

impl From<CueLibErrorKind> for CueLibError {
  #[inline]
  fn from(value: CueLibErrorKind) -> Self {
    Self { kind: value }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ParseError {
  /// Zero-based line number.
  line: usize,

  /// Zero-based column number.
  col: usize,

  /// Inner error details
  kind: ParseErrorKind,
}

impl ParseError {
  #[inline]
  pub const fn kind(&self) -> ParseErrorKind {
    self.kind
  }

  #[inline]
  pub const fn line(&self) -> usize {
    self.line
  }

  #[inline]
  pub const fn column(&self) -> usize {
    self.col
  }

  #[inline]
  pub const fn new(kind: ParseErrorKind, line: usize, col: usize) -> Self {
    Self { kind, line, col }
  }

  #[inline]
  pub(crate) const fn new_with_position(kind: ParseErrorKind, position: &Position) -> Self {
    Self {
      kind,
      line: position.line,
      col: position.column,
    }
  }

  #[inline]
  pub(crate) const fn new_with_line(kind: ParseErrorKind, line: usize) -> Self {
    Self { kind, line, col: 0 }
  }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ParseErrorKind {
  CueStrError(CueStrError),
  DataTypeParseError(DataTypeParseError),
  DigitsParseError(DigitsParseError),
  FlagParseError(FlagParseError),
  InvalidCommandFormat,
  InvalidNumericRange(InvalidNumericRange),
  IsrcParseError(IsrcParseError),
  TimeStampParseError(TimeStampParseError),
  UnknownCommand,
  UnknownFileType(UnknownFileType),
  InvalidCuesheetFormat,
  InvalidCommandUsage,
  EmptyCuesheet,
  MultipleCommand,
  InvalidTrackNo,
  InvalidTrackIndex,
  MissingTrackIndex,
  MissingTrackCommand,
}

impl From<UnknownFileType> for ParseErrorKind {
  #[inline]
  fn from(error: UnknownFileType) -> Self {
    ParseErrorKind::UnknownFileType(error)
  }
}

impl From<DataTypeParseError> for ParseErrorKind {
  #[inline]
  fn from(error: DataTypeParseError) -> Self {
    ParseErrorKind::DataTypeParseError(error)
  }
}

impl From<InvalidNumericRange> for ParseErrorKind {
  #[inline]
  fn from(error: InvalidNumericRange) -> Self {
    ParseErrorKind::InvalidNumericRange(error)
  }
}

impl From<DigitsParseError> for ParseErrorKind {
  #[inline]
  fn from(error: DigitsParseError) -> Self {
    ParseErrorKind::DigitsParseError(error)
  }
}

impl From<TimeStampParseError> for ParseErrorKind {
  #[inline]
  fn from(error: TimeStampParseError) -> Self {
    ParseErrorKind::TimeStampParseError(error)
  }
}

impl From<CueStrError> for ParseErrorKind {
  #[inline]
  fn from(error: CueStrError) -> Self {
    ParseErrorKind::CueStrError(error)
  }
}

impl From<FlagParseError> for ParseErrorKind {
  #[inline]
  fn from(error: FlagParseError) -> Self {
    ParseErrorKind::FlagParseError(error)
  }
}

impl From<IsrcParseError> for ParseErrorKind {
  #[inline]
  fn from(error: IsrcParseError) -> Self {
    ParseErrorKind::IsrcParseError(error)
  }
}

impl From<ParseError> for CueLibError {
  #[inline]
  fn from(error: ParseError) -> Self {
    CueLibError {
      kind: CueLibErrorKind::ParseError(error),
    }
  }
}

impl core::fmt::Display for ParseErrorKind {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      ParseErrorKind::CueStrError(err) => err.fmt(f),
      ParseErrorKind::DataTypeParseError(err) => err.fmt(f),
      ParseErrorKind::DigitsParseError(err) => err.fmt(f),
      ParseErrorKind::EmptyCuesheet => f.write_str("empty cuesheet input"),
      ParseErrorKind::FlagParseError(err) => err.fmt(f),
      ParseErrorKind::IsrcParseError(err) => err.fmt(f),
      ParseErrorKind::InvalidCommandFormat => f.write_str("invalid cuesheet command format"),
      ParseErrorKind::InvalidCommandUsage => f.write_str("invalid cuesheet command usage"),
      ParseErrorKind::InvalidCuesheetFormat => f.write_str("invalid cuesheet format"),
      ParseErrorKind::InvalidNumericRange(err) => err.fmt(f),
      ParseErrorKind::TimeStampParseError(err) => err.fmt(f),
      ParseErrorKind::UnknownCommand => f.write_str("unknown cuesheet command"),
      ParseErrorKind::UnknownFileType(err) => err.fmt(f),
      ParseErrorKind::MultipleCommand => f.write_str("command can only be used once"),
      ParseErrorKind::InvalidTrackNo => f.write_str("invalid track number"),
      ParseErrorKind::InvalidTrackIndex => f.write_str("invalid track index"),
      ParseErrorKind::MissingTrackCommand => f.write_str("at least one track must be specified"),
      ParseErrorKind::MissingTrackIndex => {
        f.write_str("at least one track index must be specified")
      }
    }
  }
}

impl core::fmt::Display for CueLibErrorKind {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      CueLibErrorKind::ParseError(err) => {
        f.write_fmt(format_args!("parse error, {kind}", kind = err.kind,))
      }
    }
  }
}

impl core::fmt::Display for CueLibError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("{}", self.kind))
  }
}

impl core::error::Error for CueLibError {}
