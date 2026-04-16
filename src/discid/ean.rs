use super::{
  checksum::calc_ean_13_checksum,
  error::{EanParseError, EanParseErrorKind},
};
use crate::core::Digits;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Ean13 {
  value: Digits<13>,
}

impl Ean13 {
  pub fn new(code: Digits<12>) -> Self {
    let checksum = calc_ean_13_checksum(&code);
    let mut value = [0u8; 13];

    (&mut value[..12]).copy_from_slice(code.as_bytes());
    value[12] = checksum;

    Self {
      value: unsafe { Digits::new_unchecked(&value) },
    }
  }

  #[inline]
  pub fn as_ascii_bytes(&self) -> [u8; 13] {
    self.value.as_ascii_bytes()
  }

  #[inline]
  pub const fn as_bytes(&self) -> &[u8; 13] {
    self.value.as_bytes()
  }

  #[inline]
  pub fn gs1(&self) -> Digits<3> {
    unsafe {
      Digits::new_unchecked(
        self.value.as_bytes()[..3]
          .try_into()
          .expect("gs1 never panics"),
      )
    }
  }

  #[inline]
  pub fn checksum(&self) -> u8 {
    self.value[12]
  }

  #[inline]
  pub fn code(&self) -> Digits<9> {
    unsafe {
      Digits::new_unchecked(
        self.value.as_bytes()[3..]
          .try_into()
          .expect("variable code never panics"),
      )
    }
  }
}

impl core::str::FromStr for Ean13 {
  type Err = EanParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if s.len() != 13 {
      return Err(EanParseError::new(EanParseErrorKind::InvalidLength));
    }

    let value = Digits::<12>::from_str(&s[..12])
      .map_err(|_| EanParseError::new(EanParseErrorKind::InvalidCharacter))?;
    let checksum = match s.as_bytes().last() {
      Some(value @ b'0'..=b'9') => Ok(value - b'0'),
      _ => Err(EanParseError::new(EanParseErrorKind::InvalidCharacter)),
    }?;

    let ean_code = Ean13::new(value);

    if ean_code.checksum() == checksum {
      Ok(ean_code)
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
  #[inline]
  fn cmp(&self, other: &Self) -> core::cmp::Ordering {
    self.value.cmp(&other.value)
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
