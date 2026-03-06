use crate::{
  core::{
    album_file::AlbumFile,
    cue_str::CueStr,
    flags::TrackFlag,
    timestamp::CueTimeStamp,
    track::{DataType, TrackNo},
  },
  discid::isrc::Isrc,
  error::CueLibError,
  internal::bitflag::define_bitflag,
  probe::{
    CueSheetProbe,
    track::{TrackSubIndexes, Tracks},
  },
};
use alloc::vec::Vec;

define_bitflag!(ParseOptionFlag u8, values = [
  (EMPTY, 0),
  #[cfg(feature = "metadata")]
  (ALLOW_VORBIS_REMARKS, 1)
]);

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct CueSheet<'a> {
  pub catalog: Option<CueStr<'a>>,
  pub cdtextfile: Option<CueStr<'a>>,
  pub file: Option<AlbumFile<'a>>,
  pub performer: Option<CueStr<'a>>,
  pub songwriter: Option<CueStr<'a>>,
  pub title: Option<CueStr<'a>>,
  pub tracks: Vec<TrackInfo<'a>>,

  #[cfg(feature = "metadata")]
  pub remark_metadata: crate::metadata::MetadataMap<'a>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct TrackInfo<'a> {
  pub data_type: DataType,
  pub flags: Option<TrackFlag>,
  pub isrc: Option<Isrc>,
  pub performer: Option<CueStr<'a>>,
  pub postgap: Option<CueTimeStamp>,
  pub pregap: Option<CueTimeStamp>,
  pub songwriter: Option<CueStr<'a>>,
  pub sub_indexes: Option<Vec<CueTimeStamp>>,
  pub time_info: TimeInfo,
  pub title: Option<CueStr<'a>>,
  pub track_no: TrackNo,

  #[cfg(feature = "metadata")]
  pub remark_metadata: crate::metadata::MetadataMap<'a>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Default, Debug)]
pub struct TimeInfo {
  start: u128,
  end: Option<u128>,
  pregap_start: Option<u128>,
  duration: Option<u128>,
}

#[derive(Default)]
pub struct CueSheetParser {
  flags: ParseOptionFlag,
}

impl CueSheetParser {
  #[inline]
  pub const fn new() -> Self {
    Self {
      flags: ParseOptionFlag::EMPTY,
    }
  }

  #[cfg(feature = "metadata")]
  #[inline]
  pub const fn allow_vorbis_remarks(mut self, value: bool) -> Self {
    if value {
      self.flags.set_assign(ParseOptionFlag::ALLOW_VORBIS_REMARKS);
    } else {
      self
        .flags
        .unset_assign(ParseOptionFlag::ALLOW_VORBIS_REMARKS);
    }

    self
  }

  pub fn parse<'a>(self, input: &'a str) -> Result<CueSheet<'a>, CueLibError> {
    let probe = CueSheetProbe::new(input)?;
    let mut cuesheet = CueSheet {
      catalog: probe.catalog(),
      cdtextfile: probe.cdtextfile(),
      file: probe.file_info(),
      performer: probe.performer(),
      songwriter: probe.songwriter(),
      title: probe.album_title(),
      tracks: Vec::new(),

      #[cfg(feature = "metadata")]
      remark_metadata: crate::metadata::MetadataMap::new(),
    };

    #[cfg(feature = "metadata")]
    {
      if self.flags.has(ParseOptionFlag::ALLOW_VORBIS_REMARKS) {
        cuesheet.remark_metadata = crate::metadata::MetadataMap::from_iter(probe.vorbis_comments());
      }
    };

    self.process_tracks(cuesheet, probe.tracks())
  }

  #[inline]
  fn process_tracks<'a>(
    &self,
    mut cuesheet: CueSheet<'a>,
    mut track_probe: Tracks<'a>,
  ) -> Result<CueSheet<'a>, CueLibError> {
    while let Some(track) = track_probe.next_track()? {
      let mut track_info = TrackInfo {
        data_type: track.track_data_type(),
        flags: track.flags(),
        isrc: track.isrc(),
        performer: track.performer(),
        postgap: track.postgap(),
        pregap: track.pregap(),
        songwriter: track.songwriter(),
        sub_indexes: None,
        time_info: TimeInfo {
          start: track.start_index().as_millis(),
          pregap_start: track.pregap_index().map(|v| v.as_millis()),
          end: None,
          duration: None,
        },
        title: track.title(),
        track_no: track.track_no(),

        #[cfg(feature = "metadata")]
        remark_metadata: crate::metadata::MetadataMap::new(),
      };

      Self::process_sub_indexes(&mut track_info, track.sub_indexes())?;

      #[cfg(feature = "metadata")]
      {
        if self.flags.has(ParseOptionFlag::ALLOW_VORBIS_REMARKS) {
          track_info.remark_metadata =
            crate::metadata::MetadataMap::from_iter(track.vorbis_comments());
        }
      }

      cuesheet.tracks.push(track_info);
    }

    Self::calc_track_times(&mut cuesheet.tracks);

    Ok(cuesheet)
  }

  fn process_sub_indexes<'a>(
    track: &mut TrackInfo<'a>,
    mut indexes: TrackSubIndexes<'a>,
  ) -> Result<(), CueLibError> {
    let mut sub_indexes = Vec::new();
    while let Some(index) = indexes.next_index()? {
      sub_indexes.push(index.timestamp);
    }

    if !sub_indexes.is_empty() {
      track.sub_indexes = Some(sub_indexes);
    }

    Ok(())
  }

  #[inline]
  fn calc_track_times(tracks: &mut Vec<TrackInfo>) {
    let mut track_iter = tracks.iter_mut().peekable();

    while let Some(track) = track_iter.next() {
      if let Some(next_track) = track_iter.peek() {
        let end = if let Some(pregap) = next_track.time_info.pregap_start {
          pregap
        } else {
          next_track.time_info.start
        };

        track.time_info.end = Some(end);
        track.time_info.duration = Some(end - track.time_info.start);
      }
    }
  }
}
