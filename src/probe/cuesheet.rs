use super::{
  builder::CueProbeBuilder,
  remark::RemarkIter,
  track::{TrackListProbe, Tracks},
};
use crate::{
  core::{AlbumFile, Command, CueStr},
  error::{CueLibError, ParseError, ParseErrorKind},
  internal::{lexer::CueLexer, tokenizer::Tokenizer},
};

/// Cue sheet prober
pub struct CuesheetProbe<'a> {
  /// Catalog number for the release (CATALOG command)
  pub(super) catalog: Option<CueStr<'a>>,

  /// CD-TEXT file name (CDTEXTFILE command)
  pub(super) cdtextfile: Option<CueStr<'a>>,

  /// Main audio file associated with the album (FILE command)
  pub(super) file: Option<AlbumFile<'a>>,

  /// Performer name for the entire album (PERFORMER command)
  pub(super) performer: Option<CueStr<'a>>,

  /// Songwriter name for the entire album (SONGWRITER command)
  pub(super) songwriter: Option<CueStr<'a>>,

  /// Album title (TITLE command)
  pub(super) title: Option<CueStr<'a>>,

  /// Collection of track-level probe data
  pub(super) tracks_probe: TrackListProbe<'a>,

  /// Slice containing the complete album portion of the cuesheet
  pub(super) album_buffer: &'a str,
}

impl<'a> CuesheetProbe<'a> {
  /// Creates new cue sheet probe
  pub fn new(cuesheet: &'a str) -> Result<Self, CueLibError> {
    let tokenizer = Tokenizer::new(cuesheet);
    let mut lexer = CueLexer::new(tokenizer);
    let mut builder = CueProbeBuilder::new();
    let mut album_buffer_end = 0;

    'PARSER: while let Some(command) = lexer.next_command()? {
      match command {
        Command::Catalog { value } => builder.set_catalog(value),
        Command::CdTextFile { value } => builder.set_cdtextfile(value),
        Command::File { value } => builder.set_file(value),
        Command::Title { value } => builder.set_title(value),
        Command::Performer { value } => builder.set_performer(value),
        Command::SongWriter { value } => builder.set_songwriter(value),
        Command::Remark { .. } => Ok(()),
        Command::Track { value } => {
          // exit condition
          builder
            .set_tracks_probe(TrackListProbe::new(lexer.snapshot(), value))
            .map_err(|kind| ParseError::new_with_position(kind, lexer.position()))?;

          break 'PARSER;
        }
        _ => Err(ParseErrorKind::InvalidCommandUsage),
      }
      .map_err(|kind| ParseError::new_with_line(kind, lexer.position().line))?;

      album_buffer_end = lexer.cursor_position();
    }

    let probe = builder
      .build(&cuesheet[0..album_buffer_end])
      .map_err(|kind| ParseError::new_with_position(kind, lexer.position()))?;

    Ok(probe)
  }

  /// Iterates over probe to verify if cuesheet is valid.
  pub fn verify(cuesheet: &str) -> Result<(), CueLibError> {
    let probe = CuesheetProbe::new(cuesheet)?;
    let mut tracks = probe.tracks();

    while let Some(track) = tracks.next_track()? {
      let mut indexes = track.sub_indexes();

      while let Some(_) = indexes.next_index()? {
        continue;
      }
    }

    Ok(())
  }

  /// Returns a reference to the album title if present.
  #[inline]
  pub const fn album_title(&self) -> Option<CueStr<'a>> {
    self.title
  }

  /// Returns a reference to the performer name if present.
  #[inline]
  pub const fn performer(&self) -> Option<CueStr<'a>> {
    self.performer
  }

  /// Returns a reference to the catalog number if present.
  #[inline]
  pub const fn catalog(&self) -> Option<CueStr<'a>> {
    self.catalog
  }

  /// Returns a reference to the songwriter name if present.
  #[inline]
  pub const fn songwriter(&self) -> Option<CueStr<'a>> {
    self.songwriter
  }

  /// Returns a reference to the CD-TEXT file name if present.
  #[inline]
  pub const fn cdtextfile(&self) -> Option<CueStr<'a>> {
    self.cdtextfile
  }

  /// Returns a reference to the main audio file information if present.
  #[inline]
  pub const fn file_info(&self) -> Option<AlbumFile<'a>> {
    self.file
  }

  /// Returns an iterator over the tracks in the cuesheet.
  #[inline]
  pub const fn tracks(&self) -> Tracks<'a> {
    self.tracks_probe.iter()
  }

  /// Returns an iterator over the remarks on the album portion of the cuesheet.
  #[inline]
  pub const fn remarks(&self) -> RemarkIter<'a> {
    RemarkIter::new(self.album_buffer)
  }

  /// Returns an iterator over the vorbis metadata remarks in the album portion of the cuesheet.
  ///
  /// <div class="warning">
  ///
  /// Requires **metadata** feature
  ///
  /// </div>
  #[cfg(feature = "metadata")]
  #[inline]
  pub fn vorbis_comments(&self) -> crate::probe::vorbis_remark::VorbisRemarkIter<'a> {
    self.remarks().into()
  }
}
