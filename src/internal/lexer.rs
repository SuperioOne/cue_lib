use super::{
  enum_str::impl_enum_str,
  tokenizer::{Position, Tokenizer},
};
use crate::{
  core::{
    album_file::{AlbumFile, KnownFileType},
    command::Command,
    cue_str::CueStr,
    error::CueStrError,
    flags::TrackFlag,
    timestamp::CueTimestamp,
    track::{DataType, IndexNo, Track, TrackIndex, TrackNo},
  },
  discid::isrc::Isrc,
  error::{ParseError, ParseErrorKind},
  internal::tokenizer::Token,
};
use core::str::FromStr;

#[derive(Clone)]
pub struct CueLexer<'a> {
  tokenizer: Tokenizer<'a>,
  position: Position,
}

struct UnknownCommand;

impl_enum_str!(
  CommandName,
  parse_error = UnknownCommand,
  values = [
    (Catalog, "CATALOG"),
    (CdTextFile, "CDTEXTFILE"),
    (File, "FILE"),
    (Flags, "FLAGS"),
    (Index, "INDEX"),
    (Isrc, "ISRC"),
    (Performer, "PERFORMER"),
    (Postgap, "POSTGAP"),
    (Pregap, "PREGAP"),
    (Remark, "REM"),
    (Songwriter, "SONGWRITER"),
    (Title, "TITLE"),
    (Track, "TRACK")
  ]
);

impl<'a> CueLexer<'a> {
  pub const fn new(tokenizer: Tokenizer<'a>) -> Self {
    let position = *tokenizer.position();
    Self {
      tokenizer,
      position,
    }
  }

  /// Creates a snapshot of tokenizer and creates a new lexer instance from it.
  pub const fn snapshot(&self) -> Self {
    Self {
      tokenizer: self.tokenizer.snapshot(),
      position: self.position,
    }
  }

  #[inline]
  pub const fn position(&self) -> &Position {
    &self.position
  }

  #[inline]
  pub const fn cursor_position(&self) -> usize {
    self.tokenizer.cursor_position()
  }

  #[inline]
  pub const fn as_raw_buffer(&self) -> &'a str {
    self.tokenizer.as_raw_buffer()
  }

  pub fn next_command(&mut self) -> Result<Option<Command<'a>>, ParseError> {
    loop {
      match self.next_token() {
        Ok(Some(command_token)) => match command_token {
          Token::Text {
            value: CueStr::Text(cmd_text),
          } => {
            let command_name = match CommandName::from_str(cmd_text) {
              Ok(name) => Ok(name),
              Err(_) => Err(ParseError::new_with_position(
                ParseErrorKind::UnknownCommand,
                self.position(),
              )),
            }?;

            return match command_name {
              CommandName::Catalog => self.read_catalog(),
              CommandName::CdTextFile => self.read_cdtextfile(),
              CommandName::File => self.read_file(),
              CommandName::Flags => self.read_flags(),
              CommandName::Index => self.read_index(),
              CommandName::Isrc => self.read_isrc(),
              CommandName::Performer => self.read_performer(),
              CommandName::Postgap => self.read_postgap(),
              CommandName::Pregap => self.read_pregap(),
              CommandName::Remark => self.read_remark(),
              CommandName::Songwriter => self.read_songwriter(),
              CommandName::Title => self.read_title(),
              CommandName::Track => self.read_track(),
            }
            .map(|v| Some(v));
          }
          Token::LF => continue,
          _ => {
            return Err(ParseError::new_with_position(
              ParseErrorKind::InvalidCuesheetFormat,
              self.position(),
            ));
          }
        },
        Ok(None) => return Ok(None),
        Err(err) => return Err(ParseError::new_with_position(err.into(), self.position())),
      };
    }
  }

  fn next_token(&mut self) -> Result<Option<Token<'a>>, CueStrError> {
    self.position = *self.tokenizer.position();
    self.tokenizer.next_token()
  }

  fn expect_cue_str(&mut self) -> Result<CueStr<'a>, ParseError> {
    match self.next_token() {
      Ok(Some(Token::Text { value })) => Ok(value),
      Ok(_) => Err(ParseError::new_with_position(
        ParseErrorKind::InvalidCuesheetFormat,
        self.position(),
      )),
      Err(err) => Err(ParseError::new_with_position(err.into(), self.position())),
    }
  }

  fn expect_str(&mut self) -> Result<&'a str, ParseError> {
    match self.next_token() {
      Ok(Some(Token::Text {
        value: CueStr::Text(text),
      })) => Ok(text),
      Ok(_) => Err(ParseError::new_with_position(
        ParseErrorKind::InvalidCuesheetFormat,
        self.position(),
      )),
      Err(err) => Err(ParseError::new_with_position(err.into(), self.position())),
    }
  }

  fn expect_line_end(&mut self) -> Result<(), ParseError> {
    match self.next_token() {
      Ok(Some(Token::LF) | None) => Ok(()),
      _ => Err(ParseError::new_with_position(
        ParseErrorKind::InvalidCuesheetFormat,
        self.position(),
      )),
    }
  }

  fn read_catalog(&mut self) -> Result<Command<'a>, ParseError> {
    let value = self.expect_cue_str()?;
    self.expect_line_end()?;

    Ok(Command::Catalog { value })
  }

  fn read_cdtextfile(&mut self) -> Result<Command<'a>, ParseError> {
    let value = self.expect_cue_str()?;
    self.expect_line_end()?;

    Ok(Command::CdTextFile { value })
  }

  fn read_file(&mut self) -> Result<Command<'a>, ParseError> {
    let name = self.expect_cue_str()?;
    let type_str = self.expect_str()?;
    let file_type = KnownFileType::from_str(type_str)
      .map_err(|err| ParseError::new_with_position(err.into(), self.position()))?;

    self.expect_line_end()?;

    let value = AlbumFile { name, file_type };
    Ok(Command::File { value })
  }

  fn read_flags(&mut self) -> Result<Command<'a>, ParseError> {
    let mut value = TrackFlag::default();

    loop {
      match self.next_token() {
        Ok(Some(Token::Text {
          value: CueStr::Text(flag_str),
        })) => {
          value |= TrackFlag::from_str(flag_str)
            .map_err(|err| ParseError::new_with_position(err.into(), self.position()))?;
        }
        Ok(Some(Token::Text { .. })) => {
          return Err(ParseError::new_with_position(
            ParseErrorKind::InvalidCuesheetFormat,
            self.position(),
          ));
        }
        Ok(Some(Token::LF) | None) => break,
        Err(err) => return Err(ParseError::new_with_position(err.into(), self.position())),
      }
    }

    // NOTE: handles the case of FLAG command with full of whitespaces
    if value == TrackFlag::default() {
      Err(ParseError::new_with_position(
        ParseErrorKind::InvalidCuesheetFormat,
        self.position(),
      ))
    } else {
      Ok(Command::Flags { value })
    }
  }

  fn read_index(&mut self) -> Result<Command<'a>, ParseError> {
    let index_no = {
      let value = self.expect_str()?;
      IndexNo::from_str(value)
        .map_err(|err| ParseError::new_with_position(err.into(), self.position()))?
    };

    let timestamp = {
      let value = self.expect_str()?;
      CueTimestamp::from_str(value)
        .map_err(|err| ParseError::new_with_position(err.into(), self.position()))?
    };

    self.expect_line_end()?;

    let value = TrackIndex {
      no: index_no,
      timestamp,
    };

    Ok(Command::Index { value })
  }

  fn read_isrc(&mut self) -> Result<Command<'a>, ParseError> {
    let value = {
      let value = self.expect_str()?;
      Isrc::from_str(value)
        .map_err(|err| ParseError::new_with_position(err.into(), self.position()))?
    };

    self.expect_line_end()?;

    Ok(Command::ISRC { value })
  }

  fn read_performer(&mut self) -> Result<Command<'a>, ParseError> {
    let value = self.expect_cue_str()?;
    self.expect_line_end()?;

    Ok(Command::Performer { value })
  }

  fn read_postgap(&mut self) -> Result<Command<'a>, ParseError> {
    let timestamp = {
      let value = self.expect_str()?;
      CueTimestamp::from_str(value)
        .map_err(|err| ParseError::new_with_position(err.into(), self.position()))?
    };

    self.expect_line_end()?;

    Ok(Command::Postgap { value: timestamp })
  }

  fn read_pregap(&mut self) -> Result<Command<'a>, ParseError> {
    let timestamp = {
      let value = self.expect_str()?;
      CueTimestamp::from_str(value)
        .map_err(|err| ParseError::new_with_position(err.into(), self.position()))?
    };

    self.expect_line_end()?;

    Ok(Command::Pregap { value: timestamp })
  }

  fn read_remark(&mut self) -> Result<Command<'a>, ParseError> {
    let start = self.tokenizer.cursor_position();
    let end = loop {
      match self.next_token() {
        Ok(Some(Token::LF) | None) => {
          break self.tokenizer.cursor_position() - '\n'.len_utf8();
        }
        _ => continue,
      }
    };

    let value = &self.tokenizer.as_raw_buffer()[start..end].trim();

    Ok(Command::Remark { value })
  }

  fn read_songwriter(&mut self) -> Result<Command<'a>, ParseError> {
    let value = self.expect_cue_str()?;
    self.expect_line_end()?;

    Ok(Command::SongWriter { value })
  }

  fn read_title(&mut self) -> Result<Command<'a>, ParseError> {
    let value = self.expect_cue_str()?;
    self.expect_line_end()?;

    Ok(Command::Title { value })
  }

  fn read_track(&mut self) -> Result<Command<'a>, ParseError> {
    let track_no = {
      let value = self.expect_str()?;
      TrackNo::from_str(value)
        .map_err(|err| ParseError::new_with_position(err.into(), self.position()))?
    };

    let data_type = {
      let value = self.expect_str()?;
      DataType::from_str(value)
        .map_err(|err| ParseError::new_with_position(err.into(), self.position()))?
    };

    self.expect_line_end()?;

    let value = Track {
      track_no,
      data_type,
    };

    Ok(Command::Track { value })
  }
}

impl<'a> From<Tokenizer<'a>> for CueLexer<'a> {
  #[inline]
  fn from(value: Tokenizer<'a>) -> Self {
    Self::new(value)
  }
}
