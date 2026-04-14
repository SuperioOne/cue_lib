use super::{
  checksum::calc_upc_a_checksum,
  error::{UpcParseError, UpcParseErrorKind},
};
use crate::core::digit::Digits;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpcA {
  code: Digits<11>,
  checksum: u8,
}

impl UpcA {
  pub fn new(code: Digits<11>) -> Self {
    let checksum = calc_upc_a_checksum(&code);
    Self { code, checksum }
  }

  pub fn as_ascii_bytes(&self) -> [u8; 12] {
    let mut value = [0u8; 12];
    (&mut value[0..11]).copy_from_slice(&self.code.as_ascii_bytes());
    value[11] = self.checksum + b'0';

    value
  }

  pub fn as_bytes(&self) -> [u8; 12] {
    let mut value = [0u8; 12];
    (&mut value[0..11]).copy_from_slice(self.code.as_bytes());
    value[11] = self.checksum;

    value
  }

  #[inline]
  pub fn digit_system(&self) -> u8 {
    self.code.as_bytes()[0]
  }

  #[inline]
  pub fn left_part(&self) -> &[u8; 5] {
    self.code.as_bytes()[1..6]
      .try_into()
      .expect("LLLLL part never panics")
  }

  #[inline]
  pub fn right_part(&self) -> &[u8; 5] {
    self.code.as_bytes()[6..11]
      .try_into()
      .expect("RRRRR part never panics")
  }
}

impl core::str::FromStr for UpcA {
  type Err = UpcParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 12 {
      return Err(UpcParseError::new(UpcParseErrorKind::InvalidLength));
    }

    let code = Digits::<11>::from_str(&s[..11])
      .map_err(|_| UpcParseError::new(UpcParseErrorKind::InvalidCharacter))?;
    let checksum = match s.as_bytes().last() {
      Some(value @ b'0'..=b'9') => Ok(value - b'0'),
      _ => Err(UpcParseError::new(UpcParseErrorKind::InvalidCharacter)),
    }?;

    if calc_upc_a_checksum(&code) == checksum {
      Ok(Self { code, checksum })
    } else {
      Err(UpcParseError::new(UpcParseErrorKind::ChecksumFail))
    }
  }
}

impl core::fmt::Display for UpcA {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let bytes = self.as_ascii_bytes();
    let ascii_text = unsafe { core::str::from_utf8_unchecked(&bytes) };
    f.write_str(ascii_text)
  }
}

impl Ord for UpcA {
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.code.cmp(&other.code)
  }
}

impl PartialOrd for UpcA {
  #[inline]
  fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl From<Digits<11>> for UpcA {
  #[inline]
  fn from(value: Digits<11>) -> Self {
    Self::new(value)
  }
}
