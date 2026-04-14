use super::error::{IsrcParseError, IsrcParseErrorKind};
use crate::internal::range::impl_numeric_range_type;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Serial(u32);

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Year(u8);

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Owner {
  inner: [u8; 3],
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug, Hash)]
pub struct Country {
  inner: [u8; 2],
}

#[derive(Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub struct Isrc {
  pub country: Country,
  pub owner: Owner,
  pub year: Year,
  pub serial: Serial,
}

impl PartialOrd for Isrc {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Isrc {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    match self.country.cmp(&other.country) {
      core::cmp::Ordering::Equal => {}
      ord => return ord,
    }
    match self.owner.cmp(&other.owner) {
      core::cmp::Ordering::Equal => {}
      ord => return ord,
    }
    match self.year.cmp(&other.year) {
      core::cmp::Ordering::Equal => {}
      ord => return ord,
    }
    self.serial.cmp(&other.serial)
  }
}

impl Owner {
  #[inline]
  pub const fn new(value: [AlphaNumeric; 3]) -> Self {
    Self {
      inner: [
        value[0].as_u8().to_ascii_uppercase(),
        value[1].as_u8().to_ascii_uppercase(),
        value[2].as_u8().to_ascii_uppercase(),
      ],
    }
  }

  #[inline]
  pub const fn as_bytes(&self) -> &[u8; 3] {
    &self.inner
  }

  #[inline]
  pub const fn as_str(&self) -> &str {
    // It's guaranteed to be a valid UTF-8 str
    unsafe { core::str::from_utf8_unchecked(&self.inner) }
  }
}

impl core::str::FromStr for Owner {
  type Err = IsrcParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut owner = Self { inner: [0; 3] };

    if s.len() == 3 {
      for (idx, ch) in s.bytes().enumerate() {
        if ch.is_ascii_alphanumeric() {
          owner.inner[idx] = ch.to_ascii_uppercase();
        } else {
          return Err(IsrcParseError::new(IsrcParseErrorKind::InvalidOwner));
        }
      }

      Ok(owner)
    } else {
      Err(IsrcParseError::new(IsrcParseErrorKind::InvalidOwner))
    }
  }
}

impl core::fmt::Display for Owner {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl Country {
  #[inline]
  pub const fn new(value: [Alpha; 2]) -> Self {
    Self {
      inner: [
        value[0].as_u8().to_ascii_uppercase(),
        value[1].as_u8().to_ascii_uppercase(),
      ],
    }
  }

  #[inline]
  pub const fn as_bytes(&self) -> &[u8; 2] {
    &self.inner
  }

  #[inline]
  pub const fn as_str(&self) -> &str {
    // It's guaranteed to be a valid UTF-8 str
    unsafe { core::str::from_utf8_unchecked(&self.inner) }
  }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
pub struct Alpha(u8);

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Debug, Default)]
pub struct AlphaNumeric(u8);

impl AlphaNumeric {
  #[inline]
  pub const fn from_char(c: char) -> Option<Self> {
    if c.is_ascii_alphanumeric() {
      Some(Self(c as u8))
    } else {
      None
    }
  }

  #[inline]
  pub const fn from_u8(c: u8) -> Option<Self> {
    Self::from_char(c as char)
  }

  #[inline]
  pub const unsafe fn from_u8_unchecked(c: u8) -> Self {
    Self(c)
  }

  #[inline]
  pub const fn as_char(&self) -> char {
    self.0 as char
  }

  #[inline]
  pub const fn as_u8(&self) -> u8 {
    self.0
  }
}

impl Alpha {
  #[inline]
  pub const fn from_char(c: char) -> Option<Self> {
    if c.is_ascii_alphabetic() {
      Some(Self(c as u8))
    } else {
      None
    }
  }

  #[inline]
  pub const fn from_u8(c: u8) -> Option<Self> {
    Self::from_char(c as char)
  }

  #[inline]
  pub const unsafe fn from_u8_unchecked(c: u8) -> Self {
    Self(c)
  }

  #[inline]
  pub const fn as_char(&self) -> char {
    self.0 as char
  }

  #[inline]
  pub const fn as_u8(&self) -> u8 {
    self.0
  }
}

impl core::str::FromStr for Country {
  type Err = IsrcParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut country = Self { inner: [0; 2] };

    if s.len() == 2 {
      for (idx, ch) in s.bytes().enumerate() {
        if ch.is_ascii_alphabetic() {
          country.inner[idx] = ch.to_ascii_uppercase();
        } else {
          return Err(IsrcParseError::new(IsrcParseErrorKind::InvalidCountryCode));
        }
      }

      Ok(country)
    } else {
      Err(IsrcParseError::new(IsrcParseErrorKind::InvalidCountryCode))
    }
  }
}

impl core::fmt::Display for Country {
  #[inline]
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str(self.as_str())
  }
}

impl core::str::FromStr for Isrc {
  type Err = IsrcParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 12 {
      return Err(IsrcParseError::new(IsrcParseErrorKind::InvalidLength));
    }

    let country = Country::from_str(&s[0..2])?;
    let owner = Owner::from_str(&s[2..5])?;
    let year =
      Year::from_str(&s[5..7]).map_err(|_| IsrcParseError::new(IsrcParseErrorKind::InvalidYear))?;
    let serial = Serial::from_str(&s[7..])
      .map_err(|_| IsrcParseError::new(IsrcParseErrorKind::InvalidSerial))?;

    Ok(Self {
      country,
      owner,
      year,
      serial,
    })
  }
}

impl Isrc {
  pub fn as_ascii_bytes(&self) -> [u8; 12] {
    let mut value = [0u8; 12];
    (&mut value[0..2]).copy_from_slice(self.country.as_bytes());
    (&mut value[2..5]).copy_from_slice(self.owner.as_bytes());
    (&mut value[5..7]).copy_from_slice(self.year.as_ascii_bytes().as_slice());
    (&mut value[7..]).copy_from_slice(self.serial.as_ascii_bytes().as_slice());

    value
  }
}

impl core::fmt::Display for Isrc {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let bytes = self.as_ascii_bytes();
    f.write_str(unsafe { core::str::from_utf8_unchecked(&bytes) })
  }
}

impl_numeric_range_type!(
  Serial,
  u32,
  max = 99999_u32,
  len = 5,
  display_leading_zeros = 5
);
impl_numeric_range_type!(Year, u8, max = 99_u8, len = 2, display_leading_zeros = 2);
