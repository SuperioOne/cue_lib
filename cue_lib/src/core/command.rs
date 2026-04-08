use super::{
  album_file::AlbumFile,
  cue_str::CueStr,
  flags::TrackFlag,
  timestamp::CueTimestamp,
  track::{Track, TrackIndex},
};
use crate::discid::isrc::Isrc;

pub enum Command<'a> {
  /// Disc's media catalog number (MCN)
  Catalog { value: CueStr<'a> },

  /// Specifies the name of the file that contains the encoded CD-Text information for the disc
  CdTextFile { value: CueStr<'a> },

  /// The data or audio filename
  File { value: AlbumFile<'a> },

  /// Special subcode flags
  Flags { value: TrackFlag },

  /// Track or subtrack index
  Index { value: TrackIndex },

  /// International Standard Recording Code
  ISRC { value: Isrc },

  /// Track/Album performer
  Performer { value: CueStr<'a> },

  /// Specifies length of the track post-gap
  Postgap { value: CueTimestamp },

  /// Specifies length of the track pre-gap
  Pregap { value: CueTimestamp },

  /// Comment line, but it can also be used as additional metadata
  Remark { value: &'a str },

  /// Track/Album song writer
  SongWriter { value: CueStr<'a> },

  /// Track/Album title
  Title { value: CueStr<'a> },

  /// Starts new track
  Track { value: Track },
}
