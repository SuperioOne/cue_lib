use crate::core::{
  cue_str::CueStr,
  error::{CueStrError, CueStrErrorKind},
};

/// Zero Width No-break Space aka Byte Order Mark
const ZWNBP: char = '\u{feff}';

#[derive(Default, Debug, Clone, Copy)]
pub struct Position {
  pub line: usize,
  pub column: usize,
}

#[derive(Debug)]
pub enum Token<'a> {
  Text { value: CueStr<'a> },
  LF,
}

impl<'a> From<CueStr<'a>> for Token<'a> {
  fn from(value: CueStr<'a>) -> Self {
    Self::Text { value }
  }
}

#[derive(Clone)]
pub struct Tokenizer<'a> {
  buffer: &'a str,
  position: Position,
  cursor_index: usize,
}

impl<'a> Tokenizer<'a> {
  pub const fn new(buffer: &'a str) -> Self {
    Self {
      buffer,
      position: Position { line: 0, column: 0 },
      cursor_index: 0,
    }
  }

  #[inline]
  pub const fn position(&self) -> &Position {
    &self.position
  }

  #[inline]
  pub const fn cursor_position(&self) -> usize {
    self.cursor_index
  }

  /// This function simply clones the underlying buffer reference and position.
  /// This allows branching and rewinding the tokenizer.
  ///
  /// Exact same behavior can be achieved via [.clone()](Self::clone). If you need a const-time
  /// version use this fn.
  pub const fn snapshot(&self) -> Self {
    Self {
      buffer: self.buffer,
      position: Position {
        line: self.position.line,
        column: self.position.column,
      },
      cursor_index: self.cursor_index,
    }
  }

  #[inline]
  pub const fn as_raw_buffer(&self) -> &'a str {
    &self.buffer
  }

  pub fn next_token(&mut self) -> Result<Option<Token<'a>>, CueStrError> {
    self.eat_whitespace();
    let start = self.cursor_index;

    if start < self.buffer.len() {
      let remaining = &self.buffer[start..];

      let token: Token<'a> = match remaining
        .as_bytes()
        .first()
        .expect("unreachable 'None' case, length is already checked")
      {
        b'\n' => self.line_feed(),
        b'"' => self.quoted_str()?.into(),
        _ => self.regular_str()?.into(),
      };

      self.eat_whitespace();
      Ok(Some(token))
    } else {
      Ok(None)
    }
  }

  fn eat_whitespace(&mut self) {
    let start = self.cursor_index;
    let remaining = &self.buffer[start..];
    let mut chars = remaining.chars();

    loop {
      match chars.next() {
        Some(value) => {
          if value != '\n' && (value.is_whitespace() || value == ZWNBP) {
            self.cursor_index += value.len_utf8();
            self.position.column += 1;
          } else {
            break;
          }
        }
        None => break,
      }
    }
  }

  #[inline]
  fn line_feed(&mut self) -> Token<'a> {
    self.cursor_index += '\n'.len_utf8();
    self.position.line += 1;
    self.position.column = 0;

    Token::LF
  }

  #[inline]
  fn quoted_str(&mut self) -> Result<CueStr<'a>, CueStrError> {
    let start = self.cursor_index;
    let remaining = &self.buffer[start..];
    let mut has_escape = false;
    let mut chars = remaining.chars();

    macro_rules! next_char {
      () => {{
        let next = chars.next();

        if let Some(v) = next {
          self.cursor_index += v.len_utf8();
          self.position.column += 1;
        }

        next
      }};
    }

    // skips starting double quote
    _ = next_char!();

    loop {
      match next_char!() {
        Some('"') => {
          let end = self.cursor_index;
          let cue_str = if has_escape {
            CueStr::QuotedTextWithEscape(&self.buffer[start..end])
          } else {
            CueStr::QuotedText(&self.buffer[start..end])
          };

          return Ok(cue_str);
        }
        Some('\\') => {
          has_escape = true;
          match next_char!() {
            Some('"' | '\\') => {}
            Some(_) => return Err(CueStrError::new(CueStrErrorKind::UnescapedSpecialChar)),
            _ => return Err(CueStrError::new(CueStrErrorKind::MissingEndingQuote)),
          }
        }
        Some('\n') => return Err(CueStrError::new(CueStrErrorKind::MissingEndingQuote)),
        Some(_) => continue,
        None => break,
      }
    }

    Err(CueStrError::new(CueStrErrorKind::MissingEndingQuote))
  }

  #[inline]
  fn regular_str(&mut self) -> Result<CueStr<'a>, CueStrError> {
    let start = self.cursor_index;
    let remaining = &self.buffer[start..];
    let mut chars = remaining.chars();

    loop {
      match chars.next() {
        Some(v) if !v.is_whitespace() => {
          self.cursor_index += v.len_utf8();
          self.position.column += 1;
        }
        _ => {
          let end = self.cursor_index;
          let cue_str = CueStr::Text(&self.buffer[start..end]);

          return Ok(cue_str);
        }
      }
    }
  }
}
