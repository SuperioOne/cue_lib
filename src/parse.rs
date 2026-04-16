#[cfg(feature = "metadata")]
use crate::metadata::{map::MetadataMap, vorbis::VorbisTag};
use crate::{
  core::{AlbumFile, CueStr, CueTimestamp, DataType, TrackFlag, TrackIndex, TrackNo},
  discid::isrc::Isrc,
  error::CueLibError,
  probe::{
    CuesheetProbe,
    track::{TrackSubIndexes, Tracks},
  },
};
use alloc::vec::Vec;

/// A cuesheet representing a CD-like structure with metadata and tracks.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct Cuesheet<'a> {
  /// The catalog number associated with the cuesheet.
  pub catalog: Option<CueStr<'a>>,
  /// The CD-TEXT file name, if available.
  pub cdtextfile: Option<CueStr<'a>>,
  /// Information about the audio file, such as its path and type.
  pub file: Option<AlbumFile<'a>>,
  /// The performer (artist) of the entire album.
  pub performer: Option<CueStr<'a>>,
  /// The songwriter of the entire album.
  pub songwriter: Option<CueStr<'a>>,
  /// The title of the album.
  pub album_title: Option<CueStr<'a>>,
  /// A list of tracks in the cuesheet.
  pub tracks: Vec<TrackInfo<'a>>,

  #[cfg(feature = "metadata")]
  /// Metadata map for Vorbis comments
  pub remark_metadata: MetadataMap<'a, VorbisTag>,
}

// Represents a single track within a cuesheet.
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug)]
pub struct TrackInfo<'a> {
  /// The data type of the track (e.g., audio or mode2)
  pub data_type: DataType,
  /// Flags associated with the track
  pub flags: Option<TrackFlag>,
  /// The ISRC for the track
  pub isrc: Option<Isrc>,
  /// The performer (artist) of this track
  pub performer: Option<CueStr<'a>>,
  /// The postgap time, if present
  pub postgap: Option<CueTimestamp>,
  /// The pregap time, if present
  pub pregap: Option<CueTimestamp>,
  /// The songwriter of this track
  pub songwriter: Option<CueStr<'a>>,
  /// List of sub-indexes for this track
  pub sub_indexes: Vec<TrackIndex>,
  /// Time-related information for this track
  pub time_info: TimeInfo,
  /// The title of this track
  pub title: Option<CueStr<'a>>,
  /// The track number
  pub no: TrackNo,

  /// Metadata map for Vorbis comments, if enabled
  #[cfg(feature = "metadata")]
  pub remark_metadata: MetadataMap<'a, VorbisTag>,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Default, Debug)]
/// Time-related information for a track
pub struct TimeInfo {
  /// The start time of the track
  pub start: CueTimestamp,
  /// The end time of the track
  pub end: Option<CueTimestamp>,
  /// The start time of the pregap, if present
  pub pregap_start: Option<CueTimestamp>,
  /// The duration of the track
  pub duration: Option<CueTimestamp>,
}

/// A parser for cuesheet data
#[derive(Default)]
pub struct CuesheetParser {
  #[cfg(feature = "metadata")]
  allow_vorbis_remarks: bool,
}

impl CuesheetParser {
  /// Creates a new empty cuesheet parser
  #[inline]
  pub const fn new() -> Self {
    Self {
      allow_vorbis_remarks: false,
    }
  }

  /// Configures whether to allow Vorbis remarks during parsing
  #[cfg(feature = "metadata")]
  #[inline]
  pub const fn allow_vorbis_remarks(mut self, value: bool) -> Self {
    self.allow_vorbis_remarks = value;
    self
  }

  /// Parses a cuesheet from the given input string
  pub fn parse<'a>(self, input: &'a str) -> Result<Cuesheet<'a>, CueLibError> {
    let probe = CuesheetProbe::new(input)?;
    let mut cuesheet = Cuesheet {
      catalog: probe.catalog(),
      cdtextfile: probe.cdtextfile(),
      file: probe.file_info(),
      performer: probe.performer(),
      songwriter: probe.songwriter(),
      album_title: probe.album_title(),
      tracks: Vec::new(),

      #[cfg(feature = "metadata")]
      remark_metadata: MetadataMap::new(),
    };

    #[cfg(feature = "metadata")]
    {
      if self.allow_vorbis_remarks {
        cuesheet.remark_metadata = MetadataMap::from_iter(probe.vorbis_comments());
      }
    };

    self.process_tracks(cuesheet, probe.tracks())
  }

  #[inline]
  fn process_tracks<'a>(
    &self,
    mut cuesheet: Cuesheet<'a>,
    mut track_probe: Tracks<'a>,
  ) -> Result<Cuesheet<'a>, CueLibError> {
    while let Some(track) = track_probe.next_track()? {
      let mut track_info = TrackInfo {
        data_type: track.track_data_type(),
        flags: track.flags(),
        isrc: track.isrc(),
        performer: track.performer(),
        postgap: track.postgap(),
        pregap: track.pregap(),
        songwriter: track.songwriter(),
        sub_indexes: Vec::new(),
        time_info: TimeInfo {
          start: track.start_index(),
          pregap_start: track.pregap_index(),
          end: None,
          duration: None,
        },
        title: track.title(),
        no: track.track_no(),

        #[cfg(feature = "metadata")]
        remark_metadata: crate::metadata::map::MetadataMap::new(),
      };

      Self::process_sub_indexes(&mut track_info, track.sub_indexes())?;

      #[cfg(feature = "metadata")]
      {
        if self.allow_vorbis_remarks {
          track_info.remark_metadata = MetadataMap::from_iter(track.vorbis_comments());
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
    while let Some(index) = indexes.next_index()? {
      track.sub_indexes.push(index);
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

        let duration = end.as_millis() - track.time_info.start.as_millis();
        track.time_info.end = Some(end);
        track.time_info.duration = Some(CueTimestamp::from_millis(duration));
      }
    }
  }
}
