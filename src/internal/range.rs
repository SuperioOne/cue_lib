/// Helper macro to implement basic arithmetic operations, traits and helper functions to building range types
/// NOTE: This macro is only meant for unsigned (0 to INF) numeric ranges.
macro_rules! impl_numeric_range_type {
  ($type:ident, $inner_type:ty, max = $max:literal, len = $len:literal, display_leading_zeros = $leading:literal ) => {
    impl $type {
      pub const MAX: Self = Self($max);
      pub const MIN: Self = Self(0);

      #[inline]
      pub const fn new(value: $inner_type) -> Option<Self> {
        if value > Self::MAX.0 {
          None
        } else {
          Some(Self(value))
        }
      }

      #[inline]
      pub const fn into_inner(self) -> $inner_type {
        self.0
      }

      #[inline]
      pub const unsafe fn new_unchecked(value: $inner_type) -> Self {
        Self(value)
      }

      #[inline]
      pub const fn saturating_add(self, rhs: $inner_type) -> Self {
        let val = self.0.saturating_add(rhs);
        if val > Self::MAX.0 {
          Self::MAX
        } else {
          Self(val)
        }
      }

      #[inline]
      pub const fn saturating_sub(self, rhs: $inner_type) -> Self {
        Self(self.0.saturating_sub(rhs))
      }

      #[inline]
      pub const fn saturating_mul(self, rhs: $inner_type) -> Self {
        let val = self.0.saturating_mul(rhs);
        if val > Self::MAX.0 {
          Self::MAX
        } else {
          Self(val)
        }
      }

      #[inline]
      pub const fn wrapping_add(self, rhs: $inner_type) -> Self {
        let val = self.0.wrapping_add(rhs);
        if val > Self::MAX.0 {
          Self::MAX
        } else {
          Self(val)
        }
      }

      #[inline]
      pub const fn wrapping_sub(self, rhs: $inner_type) -> Self {
        Self(self.0.wrapping_sub(rhs))
      }

      #[inline]
      pub const fn wrapping_mul(self, rhs: $inner_type) -> Self {
        let val = self.0.wrapping_mul(rhs);
        if val > Self::MAX.0 {
          Self::MAX
        } else {
          Self(val)
        }
      }

      #[inline]
      pub fn as_ascii_bytes(&self) -> [u8; $len] {
        self.as_digits().as_ascii_bytes()
      }

      pub fn as_digits(&self) -> $crate::core::digit::Digits<$len> {
        let mut digits = [0_u8; $len];
        let mut base = (10 as $inner_type).pow($len - 1);
        let mut remaining = self.0;

        for i in 0..$len {
          let digit = remaining / base;
          remaining -= digit * base;
          digits[i] = (digit as u8);
          base /= 10;
        }

        unsafe { $crate::core::digit::Digits::new_unchecked(&digits) }
      }
    }

    impl AsRef<$inner_type> for $type {
      #[inline]
      fn as_ref(&self) -> &$inner_type {
        &self.0
      }
    }

    impl core::borrow::Borrow<$inner_type> for $type {
      #[inline]
      fn borrow(&self) -> &$inner_type {
        &self.0
      }
    }

    impl core::str::FromStr for $type {
      type Err = $crate::core::error::InvalidNumericRange;

      fn from_str(v: &str) -> Result<Self, Self::Err> {
        let value = <$inner_type>::from_str_radix(v, 10)
          .map_err(|_| $crate::core::error::InvalidNumericRange)?;

        match Self::new(value) {
          Some(v) => Ok(v),
          _ => Err($crate::core::error::InvalidNumericRange),
        }
      }
    }

    impl Into<$inner_type> for $type {
      #[inline]
      fn into(self) -> $inner_type {
        self.0
      }
    }

    impl core::fmt::Display for $type {
      fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_fmt(format_args!("{:0>padding$}", self.0, padding = $leading))
      }
    }
  };
}

pub(crate) use impl_numeric_range_type;
