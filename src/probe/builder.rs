use super::{
  cuesheet::CuesheetProbe,
  track::{TrackIndexProbe, TrackListProbe, TrackProbe},
};
use crate::{
  core::{
    album_file::AlbumFile, cue_str::CueStr, flags::TrackFlag, timestamp::CueTimestamp, track::Track,
  },
  discid::isrc::Isrc,
  error::ParseErrorKind,
};

pub struct CueProbeBuilder<'a> {
  catalog: Option<CueStr<'a>>,
  cdtextfile: Option<CueStr<'a>>,
  file: Option<AlbumFile<'a>>,
  performer: Option<CueStr<'a>>,
  songwriter: Option<CueStr<'a>>,
  title: Option<CueStr<'a>>,
  tracks_probe: Option<TrackListProbe<'a>>,
}

pub struct TrackProbeBuilder<'a> {
  flags: Option<TrackFlag>,
  isrc: Option<Isrc>,
  performer: Option<CueStr<'a>>,
  postgap: Option<CueTimestamp>,
  pregap: Option<CueTimestamp>,
  pregap_index: Option<CueTimestamp>,
  songwriter: Option<CueStr<'a>>,
  start_index: Option<CueTimestamp>,
  sub_index_probe: TrackIndexProbe<'a>,
  title: Option<CueStr<'a>>,
  track: Track,
}

impl<'a> CueProbeBuilder<'a> {
  #[inline]
  pub fn new() -> Self {
    Self {
      catalog: None,
      cdtextfile: None,
      file: None,
      performer: None,
      songwriter: None,
      title: None,
      tracks_probe: None,
    }
  }

  pub const fn set_catalog(&mut self, catalog: CueStr<'a>) -> Result<(), ParseErrorKind> {
    if self.catalog.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.catalog = Some(catalog);
    Ok(())
  }

  #[inline]
  pub const fn set_cdtextfile(&mut self, cdtextfile: CueStr<'a>) -> Result<(), ParseErrorKind> {
    if self.cdtextfile.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.cdtextfile = Some(cdtextfile);
    Ok(())
  }

  #[inline]
  pub const fn set_file(&mut self, file: AlbumFile<'a>) -> Result<(), ParseErrorKind> {
    if self.file.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.file = Some(file);
    Ok(())
  }

  #[inline]
  pub const fn set_performer(&mut self, performer: CueStr<'a>) -> Result<(), ParseErrorKind> {
    if self.performer.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.performer = Some(performer);
    Ok(())
  }

  #[inline]
  pub const fn set_songwriter(&mut self, songwriter: CueStr<'a>) -> Result<(), ParseErrorKind> {
    if self.songwriter.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.songwriter = Some(songwriter);
    Ok(())
  }

  #[inline]
  pub const fn set_title(&mut self, title: CueStr<'a>) -> Result<(), ParseErrorKind> {
    if self.title.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.title = Some(title);
    Ok(())
  }

  #[inline]
  pub const fn set_tracks_probe(
    &mut self,
    probe: TrackListProbe<'a>,
  ) -> Result<(), ParseErrorKind> {
    if self.tracks_probe.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.tracks_probe = Some(probe);
    Ok(())
  }

  pub fn build(self, album_buffer: &'a str) -> Result<CuesheetProbe<'a>, ParseErrorKind> {
    let tracks_probe = self
      .tracks_probe
      .ok_or(ParseErrorKind::MissingTrackCommand)?;

    let probe = CuesheetProbe {
      album_buffer,
      catalog: self.catalog,
      cdtextfile: self.cdtextfile,
      file: self.file,
      performer: self.performer,
      songwriter: self.songwriter,
      title: self.title,
      tracks_probe,
    };

    Ok(probe)
  }
}

impl<'a> TrackProbeBuilder<'a> {
  #[inline]
  pub fn new(index_probe: TrackIndexProbe<'a>, track: Track) -> Self {
    Self {
      track,
      sub_index_probe: index_probe,
      flags: None,
      isrc: None,
      postgap: None,
      pregap: None,
      performer: None,
      songwriter: None,
      title: None,
      pregap_index: None,
      start_index: None,
    }
  }

  #[inline]
  pub const fn set_flags(&mut self, flags: TrackFlag) -> Result<(), ParseErrorKind> {
    if self.flags.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.flags = Some(flags);
    Ok(())
  }

  #[inline]
  pub const fn set_isrc(&mut self, isrc: Isrc) -> Result<(), ParseErrorKind> {
    if self.isrc.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.isrc = Some(isrc);
    Ok(())
  }

  #[inline]
  pub const fn set_postgap(&mut self, postgap: CueTimestamp) -> Result<(), ParseErrorKind> {
    if self.postgap.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.postgap = Some(postgap);
    Ok(())
  }

  #[inline]
  pub const fn set_pregap(&mut self, pregap: CueTimestamp) -> Result<(), ParseErrorKind> {
    if self.pregap.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.pregap = Some(pregap);
    Ok(())
  }

  #[inline]
  pub const fn set_performer(&mut self, performer: CueStr<'a>) -> Result<(), ParseErrorKind> {
    if self.performer.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.performer = Some(performer);
    Ok(())
  }

  #[inline]
  pub const fn set_songwriter(&mut self, songwriter: CueStr<'a>) -> Result<(), ParseErrorKind> {
    if self.songwriter.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.songwriter = Some(songwriter);
    Ok(())
  }

  #[inline]
  pub const fn set_start_index(&mut self, start_index: CueTimestamp) -> Result<(), ParseErrorKind> {
    if self.start_index.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.start_index = Some(start_index);
    Ok(())
  }

  #[inline]
  pub const fn set_pregap_index(
    &mut self,
    pregap_index: CueTimestamp,
  ) -> Result<(), ParseErrorKind> {
    // Pregap index (INDEX 00) must be set before the start index (INDEX 01)
    if self.start_index.is_some() {
      return Err(ParseErrorKind::InvalidTrackIndex);
    }

    if self.pregap_index.is_some() {
      return Err(ParseErrorKind::InvalidTrackIndex);
    }

    self.pregap_index = Some(pregap_index);
    Ok(())
  }

  #[inline]
  pub const fn set_title(&mut self, title: CueStr<'a>) -> Result<(), ParseErrorKind> {
    if self.title.is_some() {
      return Err(ParseErrorKind::MultipleCommand);
    }

    self.title = Some(title);
    Ok(())
  }

  pub fn build(self, track_buffer: &'a str) -> Result<TrackProbe<'a>, ParseErrorKind> {
    let start_index = self.start_index.ok_or(ParseErrorKind::InvalidTrackIndex)?;

    let probe = TrackProbe {
      flags: self.flags,
      isrc: self.isrc,
      performer: self.performer,
      postgap: self.postgap,
      pregap: self.pregap,
      pregap_index: self.pregap_index,
      songwriter: self.songwriter,
      start_index,
      sub_index_probe: self.sub_index_probe,
      title: self.title,
      track: self.track,
      track_buffer,
    };

    Ok(probe)
  }
}
