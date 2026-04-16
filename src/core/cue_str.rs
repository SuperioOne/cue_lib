use crate::core::error::{CueStrError, CueStrErrorKind};
use core::ffi::CStr;

/// Cue sheet specific string
///
/// This structure does not own the data, it simply a [str] reference wrapper with some additional
/// information about escape characters and quotation marks.
///
/// ## Logic operations
/// [CueStr] can be directly compared with other [str] references. It automatically handles
/// escape characters and quotations marks without any extra allocation.
///
/// ```
/// use cue_lib::core::CueStr;
/// use std::borrow::Cow;
///
/// // Cue sheet form      -> "Hello \"world\""
/// // Human readable form -> Hello "world"
/// let cue_str = CueStr::try_from_raw_str(r#""Hello \"world\"""#).unwrap();
/// assert_eq!(r#"Hello "world""#, cue_str);
/// ```
/// ## Using as str
///
/// [CueStr] does not provide [`AsRef<str>`](AsRef) because it can contain escape sequences and
/// this might require additional memory allocation, which must be handled explicitly by the user.
///
/// - Use [`as_cow_str`](CueStr::as_cow_str) to convert [CueStr] into `Cow<'_, str>`, which avoids
/// additional allocation when [CueStr] does not contain escape characters.
/// - Use `to_string` to convert [CueStr] into `String`, duplicating string in memory
/// regardless of whether it contains escape characters or not.
/// - Match [CueStr] (it's an enum) and sanitize the inner [str] with custom logic.
///
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CueStr<'a> {
  /// Text surrounded by quotation marks
  /// ## example:
  /// - `"Multiple words"`
  /// - `"WrappedWithQuotationMarks"`
  QuotedText(&'a str),
  /// Text surrounded by quotation marks and escape sequences
  /// ## example:
  /// - `"Never \"gonna\" \\ give you up"`
  QuotedTextWithEscape(&'a str),
  /// A regular text without whitespaces, escape sequences, and quotation marks
  /// ## example:
  /// - `NeverGonnaLetYouDown`
  Text(&'a str),
}

/// Strips first and last characters
macro_rules! inner_text {
  ($input:expr) => {
    &$input[1..($input.len() - 1)]
  };
}

impl<'a> CueStr<'a> {
  /// Tries to convert [str] into [CueStr]. Returns [CueStrError] if [str] is not a valid cue
  /// sheet string.
  ///
  /// ## Remark
  /// [`TryFrom<&str>`](TryFrom) trait implementation also internally calls this function.
  pub fn try_from_raw_str(s: &'a str) -> Result<Self, CueStrError> {
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

  ///
  pub const fn into_inner(self) -> &'a str {
    match self {
      CueStr::QuotedText(v) => v,
      CueStr::QuotedTextWithEscape(v) => v,
      CueStr::Text(v) => v,
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

impl<'a> PartialEq<CueStr<'a>> for str {
  #[inline]
  fn eq(&self, other: &CueStr<'a>) -> bool {
    other.eq(self)
  }
}

impl<'a> PartialEq<CueStr<'a>> for &str {
  #[inline]
  fn eq(&self, other: &CueStr<'a>) -> bool {
    other.eq(self)
  }
}

impl<'a> TryFrom<&'a str> for CueStr<'a> {
  type Error = CueStrError;

  #[inline]
  fn try_from(value: &'a str) -> Result<Self, Self::Error> {
    Self::try_from_raw_str(value)
  }
}

impl<'a> TryFrom<&'a CStr> for CueStr<'a> {
  type Error = CueStrError;

  #[inline]
  fn try_from(value: &'a CStr) -> Result<Self, Self::Error> {
    match value.to_str() {
      Ok(v) => CueStr::try_from_raw_str(v),
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
    /// <div class="warning">
    ///
    /// Requires **alloc** feature
    ///
    /// </div>
    ///
    /// Converts [CueStr] into [`Cow<'_, str>`](Cow), which strips quotation marks and escapes charcaters if necessary.
    ///
    /// ## Remark
    ///
    /// [CueStr] also implements [Into] trait for [Cow], which calls [as_cow_str](CueStr::as_cow_str) under the
    /// hood.
    ///
    /// ```
    /// use cue_lib::core::CueStr;
    /// use std::borrow::Cow;
    ///
    /// let cue_str = CueStr::try_from_raw_str("\"Hello \\\"world\\\"\"").unwrap();
    /// let text: Cow<'_, str> =  cue_str.into();
    ///
    /// assert_eq!(text, "Hello \"world\"");
    /// ```

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
