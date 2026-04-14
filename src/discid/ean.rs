use super::{
  checksum::calc_ean_13_checksum,
  error::{EanParseError, EanParseErrorKind},
};
use crate::core::digit::Digits;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ean13 {
  code: Digits<12>,
  checksum: u8,
}

impl Ean13 {
  pub fn new(code: Digits<12>) -> Self {
    let checksum = calc_ean_13_checksum(&code);
    Self { code, checksum }
  }

  pub fn as_ascii_bytes(&self) -> [u8; 13] {
    let mut value = [0u8; 13];
    (&mut value[0..12]).copy_from_slice(&self.code.as_ascii_bytes());
    value[12] = self.checksum + b'0';

    value
  }

  pub fn as_bytes(&self) -> [u8; 13] {
    let mut value = [0u8; 13];
    (&mut value[0..12]).copy_from_slice(self.code.as_bytes());
    value[12] = self.checksum;

    value
  }

  #[inline]
  pub fn gs1(&self) -> &[u8; 3] {
    self.code.as_bytes()[..3]
      .try_into()
      .expect("gs1 never panics")
  }

  #[inline]
  pub fn code(&self) -> &[u8; 9] {
    self.code.as_bytes()[3..]
      .try_into()
      .expect("variable code never panics")
  }
}

impl core::str::FromStr for Ean13 {
  type Err = EanParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 13 {
      return Err(EanParseError::new(EanParseErrorKind::InvalidLength));
    }

    let code = Digits::<12>::from_str(&s[..12])
      .map_err(|_| EanParseError::new(EanParseErrorKind::InvalidCharacter))?;
    let checksum = match s.as_bytes().last() {
      Some(value @ b'0'..=b'9') => Ok(value - b'0'),
      _ => Err(EanParseError::new(EanParseErrorKind::InvalidCharacter)),
    }?;

    if calc_ean_13_checksum(&code) == checksum {
      Ok(Self { code, checksum })
    } else {
      Err(EanParseError::new(EanParseErrorKind::ChecksumFail))
    }
  }
}

impl core::fmt::Display for Ean13 {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let bytes = self.as_ascii_bytes();
    let ascii_text = unsafe { core::str::from_utf8_unchecked(&bytes) };
    f.write_str(ascii_text)
  }
}

impl Ord for Ean13 {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.code.cmp(&other.code)
  }
}

impl PartialOrd for Ean13 {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl From<Digits<12>> for Ean13 {
  #[inline]
  fn from(value: Digits<12>) -> Self {
    Self::new(value)
  }
}
