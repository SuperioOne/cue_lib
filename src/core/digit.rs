use super::error::DigitsParseError;

/// Fixed size, stack allocated bytes between 0 and 9
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Digits<const L: usize> {
  inner: [u8; L],
}

impl<const L: usize> Digits<L> {
  pub fn new(bytes: &[u8; L]) -> Option<Self> {
    if bytes.iter().any(|v| *v > 9) {
      None
    } else {
      Some(unsafe { Self::new_unchecked(bytes) })
    }
  }

  #[inline]
  pub unsafe fn new_unchecked(bytes: &[u8]) -> Self {
    let mut result = Self { inner: [0; L] };
    result.inner.copy_from_slice(&bytes[..L]);
    result
  }

  #[inline]
  pub const fn as_bytes(&self) -> &[u8; L] {
    &self.inner
  }

  #[inline]
  pub fn as_ascii_bytes(&self) -> [u8; L] {
    let mut ascii_bytes = self.inner.clone();
    for byte in ascii_bytes.iter_mut() {
      *byte += b'0';
    }

    ascii_bytes
  }

  #[inline]
  pub fn get(&self, idx: usize) -> Option<&u8> {
    self.inner.get(idx)
  }

  #[inline]
  pub const fn to_bytes(self) -> [u8; L] {
    self.inner
  }

  #[inline]
  pub fn to_ascii_bytes(mut self) -> [u8; L] {
    for byte in self.inner.iter_mut() {
      *byte += b'0';
    }

    self.inner
  }
}

impl<const L: usize> AsRef<[u8; L]> for Digits<L> {
  #[inline]
  fn as_ref(&self) -> &[u8; L] {
    &self.inner
  }
}

impl<const L: usize> AsRef<[u8]> for Digits<L> {
  #[inline]
  fn as_ref(&self) -> &[u8] {
    &self.inner
  }
}

impl<const L: usize> core::fmt::Display for Digits<L> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    let ascii_bytes = self.as_ascii_bytes();
    f.write_str(unsafe { core::str::from_utf8_unchecked(&ascii_bytes) })
  }
}

impl<const L: usize> core::str::FromStr for Digits<L> {
  type Err = DigitsParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let mut digits = Digits { inner: [0; L] };

    if s.len() != L {
      return Err(DigitsParseError);
    }

    for (idx, byte) in s.as_bytes().iter().enumerate() {
      if byte.is_ascii_digit() {
        digits.inner[idx] = *byte - b'0';
      } else {
        return Err(DigitsParseError);
      }
    }

    Ok(digits)
  }
}
