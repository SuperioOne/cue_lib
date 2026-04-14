use crate::core::error::{CueStrError, CueStrErrorKind};
use core::ffi::CStr;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CueStr<'a> {
  QuotedText(&'a str),
  QuotedTextWithEscape(&'a str),
  Text(&'a str),
}

/// Strips first and last characters
macro_rules! inner_text {
  ($input:expr) => {
    &$input[1..($input.len() - 1)]
  };
}

impl<'a> CueStr<'a> {
  pub fn from_raw_str(s: &'a str) -> Result<Self, CueStrError> {
    if s.len() > 1 && s.starts_with('"') {
      if s.ends_with('"') && !s.ends_with("\\\"") {
        let text = inner_text!(s);
        let mut needs_escape = false;
        let mut sequence_iter = text.chars().peekable();

        while let Some(ch) = sequence_iter.next() {
          match ch {
            '"' => {
              return Err(CueStrErrorKind::UnescapedSpecialChar.into());
            }
            '\\' => match sequence_iter.peek() {
              Some('\\' | '"') => {
                needs_escape = true;
                _ = sequence_iter.next();
              }
              _ => {
                return Err(CueStrErrorKind::UnescapedSpecialChar.into());
              }
            },
            _ => continue,
          }
        }

        let cue_str = if needs_escape {
          Self::QuotedTextWithEscape(s)
        } else {
          Self::QuotedText(s)
        };

        Ok(cue_str)
      } else {
        Err(CueStrErrorKind::MissingEndingQuote.into())
      }
    } else if s.contains(|v: char| v.is_whitespace()) {
      Err(CueStrErrorKind::MissingQuotes.into())
    } else {
      Ok(Self::Text(s))
    }
  }

  #[inline]
  /// Returns un-escaped/quoted raw str reference.
  pub const fn as_raw_str(&self) -> &str {
    match self {
      Self::QuotedText(v) => v,
      Self::QuotedTextWithEscape(v) => v,
      Self::Text(v) => v,
    }
  }
}

impl PartialEq<str> for CueStr<'_> {
  fn eq(&self, other: &str) -> bool {
    match self {
      CueStr::Text(v) => (*v).eq(other),
      CueStr::QuotedText(v) => {
        if v.len() > 1 {
          let inner = inner_text!(v);
          inner.eq(other)
        } else {
          debug_assert!(
            false,
            "Quoted CueStr length cannot be less than 2, if this debug_assert is triggered it means tokenizer is borked and tests aren't working."
          );
          false
        }
      }
      CueStr::QuotedTextWithEscape(v) => {
        if v.len() > 1 {
          let mut lhs = inner_text!(v).chars();
          let mut rhs = other.chars();

          loop {
            let mut lchar = lhs.next();
            let rchar = rhs.next();

            if lchar.is_some_and(|ch| ch == '\\') {
              lchar = lhs.next();
            }

            match (lchar, rchar) {
              (Some(l), Some(r)) if l == r => continue,
              (None, None) => break,
              _ => return false,
            }
          }

          true
        } else {
          debug_assert!(
            false,
            "Escaped quoted CueStr length cannot be less than 2, if this debug_assert is triggered it means tokenizer is borked and tests aren't working."
          );
          false
        }
      }
    }
  }
}

impl PartialEq<&str> for CueStr<'_> {
  #[inline]
  fn eq(&self, other: &&str) -> bool {
    self.eq(*other)
  }
}

impl<'a> TryFrom<&'a str> for CueStr<'a> {
  type Error = CueStrError;

  #[inline]
  fn try_from(value: &'a str) -> Result<Self, Self::Error> {
    Self::from_raw_str(value)
  }
}

impl<'a> TryFrom<&'a CStr> for CueStr<'a> {
  type Error = CueStrError;

  #[inline]
  fn try_from(value: &'a CStr) -> Result<Self, Self::Error> {
    match value.to_str() {
      Ok(v) => CueStr::from_raw_str(v),
      Err(_) => Err(CueStrErrorKind::InvalidUtf8.into()),
    }
  }
}

impl core::fmt::Display for CueStr<'_> {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    match self {
      Self::QuotedText(quoted_text) => f.write_str(inner_text!(quoted_text)),
      Self::Text(text) => f.write_str(text),
      Self::QuotedTextWithEscape(quoted_text) => {
        let text = inner_text!(quoted_text);
        let mut slice_start = 0;
        let mut iter = text.chars().enumerate();

        while let Some((idx, ch)) = iter.next() {
          if ch == '\\' {
            f.write_str(&text[slice_start..idx])?;
            slice_start = idx + 1;
            _ = iter.next();
          }
        }

        if slice_start < text.len() {
          f.write_str(&text[slice_start..])
        } else {
          Ok(())
        }
      }
    }
  }
}

#[cfg(feature = "alloc")]
mod alloc {
  use super::CueStr;
  use alloc::{borrow::Cow, string::ToString};

  impl<'a> CueStr<'a> {
    pub fn as_cow_str(&self) -> Cow<'a, str> {
      match self {
        CueStr::QuotedText(v) => Cow::Borrowed(inner_text!(v)),
        CueStr::QuotedTextWithEscape(_) => Cow::Owned(self.to_string()),
        CueStr::Text(v) => Cow::Borrowed(v),
      }
    }
  }

  impl<'a> Into<Cow<'a, str>> for CueStr<'a> {
    #[inline]
    fn into(self) -> Cow<'a, str> {
      (&self).into()
    }
  }

  impl<'a> Into<Cow<'a, str>> for &CueStr<'a> {
    #[inline]
    fn into(self) -> Cow<'a, str> {
      self.as_cow_str()
    }
  }
}
