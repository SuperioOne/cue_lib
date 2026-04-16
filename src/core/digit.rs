use super::error::DigitsParseError;
use core::ops::{Index, Range};

/// Variable-length, stack allocated bytes between 0 (0x00) and 9 (0x09)
///
/// ```
/// use cue_lib::core::Digits;
///
/// let digits = Digits::new(&[6,9]).unwrap();
///
/// assert_eq!(&[6,9], digits.as_bytes());
/// assert_eq!([b'6',b'9'], digits.as_ascii_bytes());
/// ```
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Digits<const L: usize> {
  inner: [u8; L],
}

impl<const L: usize> Digits<L> {
  /// Creates new [Digits] and checks if all bytes are in range between `0x00` and `0x09`
  pub fn new(bytes: &[u8; L]) -> Option<Self> {
    if bytes.iter().any(|v| *v > 9) {
      None
    } else {
      Some(unsafe { Self::new_unchecked(bytes) })
    }
  }

  /// Creates new [Digits] without checking if bytes are actually in range or not
  #[inline]
  pub unsafe fn new_unchecked(bytes: &[u8]) -> Self {
    let mut result = Self { inner: [0; L] };
    result.inner.copy_from_slice(&bytes[..L]);
    result
  }

  /// Returns internal bytes
  #[inline]
  pub const fn as_bytes(&self) -> &[u8; L] {
    &self.inner
  }

  /// Returns copy of internal bytes converted to ASCII number characters
  #[inline]
  pub fn as_ascii_bytes(&self) -> [u8; L] {
    let mut ascii_bytes = self.inner.clone();
    for byte in ascii_bytes.iter_mut() {
      *byte += b'0';
    }

    ascii_bytes
  }

  /// Returns single byte from given position
  #[inline]
  pub fn get(&self, idx: usize) -> Option<&u8> {
    self.inner.get(idx)
  }

  /// Consumes [Digits] and returns raw bytes
  #[inline]
  pub const fn to_bytes(self) -> [u8; L] {
    self.inner
  }

  /// Consumes [Digits] and returns ASCII number characters
  #[inline]
  pub fn to_ascii_bytes(mut self) -> [u8; L] {
    for byte in self.inner.iter_mut() {
      *byte += b'0';
    }

    self.inner
  }

  pub const fn as_u64(&self) -> u64 {
    let mut idx = L - 1;
    let mut total: u64 = 0;
    let mut base: u64 = 1;

    while idx != 0 {
      total += (self.inner[idx] as u64) * base;
      idx -= 1;
      base *= 10;
    }

    total += (self.inner[0] as u64) * base;
    total
  }
}

impl<const L: usize> AsRef<[u8; L]> for Digits<L> {
  #[inline]
  fn as_ref(&self) -> &[u8; L] {
    &self.inner
  }
}

impl<const L: usize> Index<usize> for Digits<L> {
  type Output = u8;

  #[inline]
  fn index(&self, index: usize) -> &Self::Output {
    // Intentional unwrap
    self.get(index).unwrap()
  }
}

impl<const L: usize> Index<Range<usize>> for Digits<L> {
  type Output = [u8];

  #[inline]
  fn index(&self, index: Range<usize>) -> &Self::Output {
    // Intentional unwrap
    self.inner.get(index).unwrap()
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

/// Impls number conversion for safe-bounds only
macro_rules! impl_as_number_fns {
  ($len:literal, safe_bounds = [$next:tt $($token:tt)*]) => {
    impl Digits<$len> {
      impl_as_number_fns!(@internal $len [$next $($token)*] );
    }
  };

  (@internal $len:literal [u8 $($token:tt)*]) => {
      #[inline]
      pub const fn as_u8(&self) -> u8 {
        self.as_u64() as u8
      }

      impl_as_number_fns!(@internal $len [$($token)*]);
  };

  (@internal $len:literal [u16 $($token:tt)*]) => {
      #[inline]
      pub const fn as_u16(&self) -> u16 {
        self.as_u64() as u16
      }

      impl_as_number_fns!(@internal $len [$($token)*]);
  };

  (@internal $len:literal [u32 $($token:tt)*]) => {
      #[inline]
      pub const fn as_u32(&self) -> u32 {
        self.as_u64() as u32
      }

      impl_as_number_fns!(@internal $len [$($token)*]);
  };

  (@internal $len:literal []) => { };
}

impl_as_number_fns!(9, safe_bounds = [u32]);
impl_as_number_fns!(8, safe_bounds = [u32]);
impl_as_number_fns!(7, safe_bounds = [u32]);
impl_as_number_fns!(6, safe_bounds = [u32]);
impl_as_number_fns!(5, safe_bounds = [u32]);
impl_as_number_fns!(4, safe_bounds = [u16 u32]);
impl_as_number_fns!(3, safe_bounds = [u16 u32]);
impl_as_number_fns!(2, safe_bounds = [u8 u16 u32]);
impl_as_number_fns!(1, safe_bounds = [u8 u16 u32]);
