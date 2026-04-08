use super::error::InvalidMetadataTagName;
use crate::{
  core::cue_str::CueStr,
  internal::tokenizer::{Token, Tokenizer},
};
use core::cmp::Ordering;

pub fn cmp_ignore_ascii_case<'a, 'b>(a: &'a str, b: &'b str) -> Ordering {
  let a = a.as_bytes();
  let b = b.as_bytes();
  let mut idx = 0;

  if a.is_empty() && !b.is_empty() {
    return Ordering::Less;
  } else if b.is_empty() && !a.is_empty() {
    return Ordering::Greater;
  }

  loop {
    match (a.get(idx), b.get(idx)) {
      (None, None) => return Ordering::Equal,
      (None, Some(_)) => return Ordering::Less,
      (Some(_), None) => return Ordering::Greater,
      (Some(a), Some(b)) => {
        let lhs = a.to_ascii_lowercase();
        let rhs = b.to_ascii_lowercase();

        if lhs < rhs {
          return Ordering::Less;
        } else if lhs > rhs {
          return Ordering::Greater;
        }
      }
    };

    idx += 1;
  }
}

pub fn split_metadata_line<'a>(
  line: &'a str,
) -> Result<(CueStr<'a>, CueStr<'a>), InvalidMetadataTagName> {
  if line.is_empty() {
    return Err(InvalidMetadataTagName);
  }

  let mut tokenizer = Tokenizer::new(line.trim());

  macro_rules! next_token {
    () => {
      tokenizer.next_token().map_err(|_| InvalidMetadataTagName)
    };
  }

  let tag_name = next_token!()?;
  let value = next_token!()?;

  // Trailing tokens other than LFs are not allowed
  if let Some(Token::Text { .. }) = next_token!()? {
    return Err(InvalidMetadataTagName);
  }

  match (tag_name, value) {
    (Some(Token::Text { value: tag }), Some(Token::Text { value })) => Ok((tag, value)),
    _ => Err(InvalidMetadataTagName),
  }
}

macro_rules! impl_metadata_mapping {
  ($type_name:ident, $($name:literal => $value:ident),+) => {
    static LOOKUP_TABLE: &'static [(&'static str, $type_name)] = &[
      $(
        ($name, $type_name { inner: $crate::metadata::MetadataTag::$value })
      ),+
    ];

    #[allow(nonstandard_style)]
    impl $type_name {
      $(
        pub const $value: $type_name = $type_name { inner: $crate::metadata::MetadataTag::$value };
      )+
    }

    impl $crate::metadata::Metadata for $type_name {
      fn as_str(&self) -> &'static str {
        match self.inner {
          $(
            $crate::metadata::MetadataTag::$value => $name,
          )+
          _ => unreachable!()
        }
      }
    }

    impl PartialEq<$crate::metadata::MetadataTag> for $type_name {
      #[inline]
      fn eq(&self, other: &$crate::metadata::MetadataTag) -> bool {
        self.inner.eq(other)
      }
    }

    impl Into<MetadataTag> for $type_name {
      #[inline]
      fn into(self) -> MetadataTag {
        self.inner
      }
    }

    impl core::borrow::Borrow<MetadataTag> for $type_name {
      #[inline]
      fn borrow(&self) -> &MetadataTag {
        &self.inner
      }
    }

    impl core::borrow::Borrow<MetadataTag> for &$type_name {
      #[inline]
      fn borrow(&self) -> &MetadataTag {
        &self.inner
      }
    }

    impl core::borrow::Borrow<str> for $type_name {
      #[inline]
      fn borrow(&self) -> &str {
        $crate::metadata::Metadata::as_str(self)
      }
    }

    impl core::borrow::Borrow<str> for &$type_name {
      #[inline]
      fn borrow(&self) -> &str {
        $crate::metadata::Metadata::as_str(*self)
      }
    }

    impl AsRef<MetadataTag> for $type_name {
      #[inline]
      fn as_ref(&self) -> &MetadataTag {
        &self.inner
      }
    }

    impl AsRef<str> for $type_name {
      #[inline]
      fn as_ref(&self) -> &str {
        $crate::metadata::Metadata::as_str(self)
      }
    }

    impl core::str::FromStr for $type_name {
      type Err = $crate::metadata::error::InvalidMetadataTagName;

      fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
          return Err(InvalidMetadataTagName);
        }

        match LOOKUP_TABLE.binary_search_by(|(name, _)| $crate::metadata::util::cmp_ignore_ascii_case(&name, s)) {
          Ok(idx) => match LOOKUP_TABLE.get(idx) {
            Some((_, value)) => Ok(*value),
            None => Err(InvalidMetadataTagName),
          },
          Err(_) => Err(InvalidMetadataTagName),
        }
      }
    }

  };
}

pub(crate) use impl_metadata_mapping;

#[cfg(test)]
mod test {
  use crate::metadata::util::cmp_ignore_ascii_case;
  use core::cmp::Ordering;

  #[test]
  fn cmp_empty_strings() {
    assert_eq!(cmp_ignore_ascii_case("", ""), Ordering::Equal);
  }

  #[test]
  fn cmp_empty_string_against_non_empty() {
    assert_eq!(cmp_ignore_ascii_case("", "abc"), Ordering::Less);
  }

  #[test]
  fn cmp_non_empty_string_against_empty() {
    assert_eq!(cmp_ignore_ascii_case("abc", ""), Ordering::Greater);
  }

  #[test]
  fn cmp_equal_strings_case_insensitive() {
    assert_eq!(cmp_ignore_ascii_case("abc", "AbC"), Ordering::Equal);
  }

  #[test]
  fn cmp_greater_string_case_insensitive() {
    assert_eq!(cmp_ignore_ascii_case("abcd", "AbC"), Ordering::Greater);
  }

  #[test]
  fn cmp_less_string_case_insensitive() {
    assert_eq!(
      cmp_ignore_ascii_case("abcd_1234", "AbCF1234"),
      Ordering::Less
    );
  }

  #[test]
  fn cmp_single_char_equal() {
    assert_eq!(cmp_ignore_ascii_case("a", "A"), Ordering::Equal);
  }

  #[test]
  fn cmp_single_char_greater() {
    assert_eq!(cmp_ignore_ascii_case("b", "A"), Ordering::Greater);
  }

  #[test]
  fn cmp_single_char_less() {
    assert_eq!(cmp_ignore_ascii_case("a", "B"), Ordering::Less);
  }

  #[test]
  fn cmp_unicode_ascii_mixed() {
    assert_eq!(cmp_ignore_ascii_case("café", "Café"), Ordering::Equal);
  }

  #[test]
  fn cmp_special_characters() {
    assert_eq!(cmp_ignore_ascii_case("a!@#", "A!@#"), Ordering::Equal);
  }
}
