use super::remark::RemarkIter;
use crate::{
  core::{
    command::Command,
    cue_str::CueStr,
    flags::TrackFlag,
    timestamp::CueTimestamp,
    track::{DataType, Track, TrackIndex, TrackNo},
  },
  discid::isrc::Isrc,
  error::{CueLibError, ParseError, ParseErrorKind},
  internal::lexer::CueLexer,
  probe::builder::TrackProbeBuilder,
};

#[derive(Clone)]
pub struct TrackListProbe<'a> {
  lexer: CueLexer<'a>,
  initial_track: Track,
}

#[derive(Clone)]
pub struct TrackIndexProbe<'a> {
  lexer: CueLexer<'a>,
}

#[derive(Clone)]
pub struct TrackProbe<'a> {
  /// Track number and basic track information
  pub(super) track: Track,

  /// Track-specific flags (such as preemphasis, copy permission, etc.)
  pub(super) flags: Option<TrackFlag>,

  /// International Standard Recording Code for the track
  pub(super) isrc: Option<Isrc>,

  /// Post-gap time stamp (POSTGAP command)
  pub(super) postgap: Option<CueTimestamp>,

  /// Pre-gap time stamp (PREGAP command)
  pub(super) pregap: Option<CueTimestamp>,

  /// Performer name for this specific track (PERFORMER command)
  pub(super) performer: Option<CueStr<'a>>,

  /// Songwriter name for this specific track (SONGWRITER command)
  pub(super) songwriter: Option<CueStr<'a>>,

  /// Track title (TITLE command)
  pub(super) title: Option<CueStr<'a>>,

  /// Sub-index probe for the track
  pub(super) sub_index_probe: TrackIndexProbe<'a>,

  /// Track start timestamp (INDEX 01)
  pub(super) start_index: CueTimestamp,

  /// Optional timestamp for pregap segment exist in the track file (INDEX 00)
  pub(super) pregap_index: Option<CueTimestamp>,

  /// Slice containing the complete track portion of the cuesheet
  pub(super) track_buffer: &'a str,
}

pub struct Tracks<'a> {
  lexer: CueLexer<'a>,
  track: Option<Track>,
}

pub struct TrackSubIndexes<'a> {
  lexer: CueLexer<'a>,
  prev_index: Option<TrackIndex>,
}

impl<'a> TrackProbe<'a> {
  #[inline]
  pub const fn track_info(&self) -> &Track {
    &self.track
  }

  #[inline]
  pub const fn track_data_type(&self) -> DataType {
    self.track.data_type
  }

  #[inline]
  pub const fn track_no(&self) -> TrackNo {
    self.track.track_no
  }

  #[inline]
  pub const fn isrc(&self) -> Option<Isrc> {
    self.isrc
  }

  #[inline]
  pub const fn flags(&self) -> Option<TrackFlag> {
    self.flags
  }

  #[inline]
  pub const fn postgap(&self) -> Option<CueTimestamp> {
    self.postgap
  }

  #[inline]
  pub const fn pregap(&self) -> Option<CueTimestamp> {
    self.pregap
  }

  #[inline]
  pub const fn performer(&self) -> Option<CueStr<'a>> {
    self.performer
  }

  #[inline]
  pub const fn songwriter(&self) -> Option<CueStr<'a>> {
    self.songwriter
  }

  #[inline]
  pub const fn title(&self) -> Option<CueStr<'a>> {
    self.title
  }

  #[inline]
  pub const fn sub_indexes(&self) -> TrackSubIndexes<'a> {
    self.sub_index_probe.iter()
  }

  #[inline]
  pub const fn start_index(&self) -> CueTimestamp {
    self.start_index
  }

  #[inline]
  pub const fn pregap_index(&self) -> Option<CueTimestamp> {
    self.pregap_index
  }

  #[inline]
  pub fn remarks(&self) -> RemarkIter<'a> {
    RemarkIter::new(self.track_buffer)
  }

  #[cfg(feature = "metadata")]
  #[inline]
  pub fn vorbis_comments(&self) -> crate::probe::vorbis_remark::VorbisRemarkIter<'a> {
    self.remarks().into()
  }
}

impl<'a> TrackListProbe<'a> {
  #[inline]
  pub(super) const fn iter(&self) -> Tracks<'a> {
    Tracks {
      lexer: self.lexer.snapshot(),
      track: Some(self.initial_track),
    }
  }

  #[inline]
  pub(super) const fn new(lexer: CueLexer<'a>, initial_track: Track) -> Self {
    Self {
      lexer,
      initial_track,
    }
  }
}

impl<'a> TrackIndexProbe<'a> {
  #[inline]
  const fn iter(&self) -> TrackSubIndexes<'a> {
    TrackSubIndexes {
      lexer: self.lexer.snapshot(),
      prev_index: None,
    }
  }
}

impl<'a> Tracks<'a> {
  pub fn next_track(&mut self) -> Result<Option<TrackProbe<'a>>, CueLibError> {
    if let Some(curr_track) = self.track {
      let index_probe = TrackIndexProbe {
        lexer: self.lexer.snapshot(),
      };
      let mut builder = TrackProbeBuilder::new(index_probe, curr_track);
      let track_buf_start = self.lexer.cursor_position();
      let mut track_buf_end = track_buf_start;

      'PARSER: loop {
        match self.lexer.next_command()? {
          Some(Command::Index { value }) => match value.index_no.into_inner() {
            0 => builder.set_pregap_index(value.timestamp),
            1 => builder.set_start_index(value.timestamp),
            _ => Ok(()),
          },
          Some(Command::Remark { .. }) => Ok(()),
          Some(Command::Flags { value }) => builder.set_flags(value),
          Some(Command::ISRC { value }) => builder.set_isrc(value),
          Some(Command::Performer { value }) => builder.set_performer(value),
          Some(Command::Postgap { value }) => builder.set_postgap(value),
          Some(Command::Pregap { value }) => builder.set_pregap(value),
          Some(Command::SongWriter { value }) => builder.set_songwriter(value),
          Some(Command::Title { value }) => builder.set_title(value),
          Some(Command::Track { value }) => {
            // Track no's must be sequential
            if curr_track.track_no.saturating_add(1) == value.track_no {
              self.track = Some(value);
              break 'PARSER;
            } else {
              Err(ParseErrorKind::InvalidTrackNo)
            }
          }
          None => {
            self.track = None;
            break 'PARSER;
          }
          Some(_) => Err(ParseErrorKind::InvalidCommandUsage),
        }
        .map_err(|kind| ParseError::new_with_line(kind, self.lexer.position().line))?;

        track_buf_end = self.lexer.cursor_position();
      }

      let track_buf = &self.lexer.as_raw_buffer()[track_buf_start..track_buf_end];
      let probe = builder
        .build(track_buf)
        .map_err(|kind| ParseError::new_with_position(kind, self.lexer.position()))?;

      return Ok(Some(probe));
    } else {
      Ok(None)
    }
  }
}

impl<'a> TrackSubIndexes<'a> {
  pub fn next_index(&mut self) -> Result<Option<TrackIndex>, CueLibError> {
    loop {
      match self.lexer.next_command()? {
        Some(Command::Index { value }) => {
          let is_valid = match self.prev_index {
            Some(prev) => {
              value.index_no == prev.index_no.saturating_add(1) && value.timestamp >= prev.timestamp
            }
            None => value.index_no.into_inner() <= 1,
          };

          if is_valid {
            self.prev_index = Some(value);

            match value.index_no.into_inner() {
              0 | 1 => continue,
              _ => return Ok(Some(value)),
            };
          } else {
            let parse_error = ParseError::new_with_line(
              ParseErrorKind::InvalidTrackIndex,
              self.lexer.position().line,
            );

            return Err(parse_error.into());
          }
        }
        Some(Command::Track { .. }) | None => {
          return Ok(None);
        }
        Some(_) => continue,
      }
    }
  }
}
