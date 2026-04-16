use super::{
  checksum::calc_upc_a_checksum,
  error::{UpcParseError, UpcParseErrorKind},
};
use crate::core::Digits;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UpcA {
  value: Digits<12>,
}

impl UpcA {
  pub fn new(code: Digits<11>) -> Self {
    let checksum = calc_upc_a_checksum(&code);
    let mut value = [0u8; 12];

    (&mut value[..11]).copy_from_slice(code.as_bytes());
    value[11] = checksum;

    Self {
      value: unsafe { Digits::new_unchecked(&value) },
    }
  }

  #[inline]
  pub fn as_ascii_bytes(&self) -> [u8; 12] {
    self.value.as_ascii_bytes()
  }

  #[inline]
  pub const fn as_bytes(&self) -> &[u8; 12] {
    self.value.as_bytes()
  }

  #[inline]
  pub fn digit_system(&self) -> u8 {
    self.value[0]
  }

  #[inline]
  pub fn checksum(&self) -> u8 {
    self.value[11]
  }

  #[inline]
  pub fn left_part(&self) -> Digits<5> {
    unsafe { Digits::new_unchecked(&self.value[1..6]) }
  }

  #[inline]
  pub fn right_part(&self) -> Digits<5> {
    unsafe { Digits::new_unchecked(&self.value[6..11]) }
  }
}

impl core::str::FromStr for UpcA {
  type Err = UpcParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 12 {
      return Err(UpcParseError::new(UpcParseErrorKind::InvalidLength));
    }

    let value = Digits::<11>::from_str(&s[..11])
      .map_err(|_| UpcParseError::new(UpcParseErrorKind::InvalidCharacter))?;
    let checksum = match s.as_bytes().last() {
      Some(value @ b'0'..=b'9') => Ok(value - b'0'),
      _ => Err(UpcParseError::new(UpcParseErrorKind::InvalidCharacter)),
    }?;

    let upc = UpcA::new(value);

    if upc.checksum() == checksum {
      Ok(upc)
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
    self.value.cmp(&other.value)
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
