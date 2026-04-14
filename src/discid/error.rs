/// Represents an error when parsing an ISRC string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IsrcParseError {
  kind: IsrcParseErrorKind,
}

impl IsrcParseError {
  #[inline]
  pub const fn new(kind: IsrcParseErrorKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub const fn kind(&self) -> IsrcParseErrorKind {
    self.kind
  }
}

/// Kinds of errors that can occur while parsing an ISRC string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IsrcParseErrorKind {
  /// The ISRC string has an invalid length.
  InvalidLength,
  /// The ISRC string has an invalid owner code.
  InvalidOwner,
  /// The ISRC string has an invalid country code.
  InvalidCountryCode,
  /// The ISRC string has an invalid serial number.
  InvalidSerial,
  /// The ISRC string has an invalid year.
  InvalidYear,
}

impl core::fmt::Display for IsrcParseErrorKind {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      IsrcParseErrorKind::InvalidLength => f.write_str("ISRC string has invalid length"),
      IsrcParseErrorKind::InvalidOwner => f.write_str("ISRC string has invalid owner code"),
      IsrcParseErrorKind::InvalidCountryCode => f.write_str("ISRC string has invalid country code"),
      IsrcParseErrorKind::InvalidSerial => f.write_str("ISRC string has invalid serial number"),
      IsrcParseErrorKind::InvalidYear => f.write_str("ISRC string has invalid year"),
    }
  }
}

impl core::fmt::Display for IsrcParseError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("invalid ISRC string: {}", self.kind))
  }
}

impl core::error::Error for IsrcParseError {}

/// Represents an error when parsing an EAN string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EanParseError {
  kind: EanParseErrorKind,
}

impl EanParseError {
  #[inline]
  pub const fn new(kind: EanParseErrorKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub const fn kind(&self) -> EanParseErrorKind {
    self.kind
  }
}

/// Kinds of errors that can occur while parsing an EAN string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EanParseErrorKind {
  /// The EAN string contains an invalid character.
  InvalidCharacter,
  /// The EAN string has an invalid length.
  InvalidLength,
  /// The EAN string failed checksum validation.
  ChecksumFail,
}

impl core::fmt::Display for EanParseErrorKind {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      EanParseErrorKind::InvalidCharacter => f.write_str("EAN string contains invalid character"),
      EanParseErrorKind::InvalidLength => f.write_str("EAN string has invalid length"),
      EanParseErrorKind::ChecksumFail => f.write_str("EAN string failed checksum validation"),
    }
  }
}

impl core::fmt::Display for EanParseError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("invalid EAN string: {}", self.kind))
  }
}

impl core::error::Error for EanParseError {}

/// Represents an error when parsing a UPC string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UpcParseError {
  kind: UpcParseErrorKind,
}

impl UpcParseError {
  #[inline]
  pub const fn new(kind: UpcParseErrorKind) -> Self {
    Self { kind }
  }

  #[inline]
  pub const fn kind(&self) -> UpcParseErrorKind {
    self.kind
  }
}

/// Kinds of errors that can occur while parsing a UPC string.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UpcParseErrorKind {
  /// The UPC string contains an invalid character.
  InvalidCharacter,
  /// The UPC string has an invalid length.
  InvalidLength,
  /// The UPC string failed checksum validation.
  ChecksumFail,
}

impl core::fmt::Display for UpcParseErrorKind {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      UpcParseErrorKind::InvalidCharacter => f.write_str("UPC string contains invalid character"),
      UpcParseErrorKind::InvalidLength => f.write_str("UPC string has invalid length"),
      UpcParseErrorKind::ChecksumFail => f.write_str("UPC string failed checksum validation"),
    }
  }
}

impl core::fmt::Display for UpcParseError {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_fmt(format_args!("invalid UPC string: {}", self.kind))
  }
}

impl core::error::Error for UpcParseError {}
