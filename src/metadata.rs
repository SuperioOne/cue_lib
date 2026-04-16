#[cfg(feature = "serde")]
mod serde;
mod util;

#[cfg(feature = "alloc")]
pub mod map;

/// FFmpeg's internal metadata mappings
pub mod avlib;
/// Metadata related errors
pub mod error;
/// ID3 metadata mappings
pub mod id3;
/// Vorbis comment tags
pub mod vorbis;

use core::borrow::Borrow;

/// A common trait to get metadata tag name
pub trait Metadata: Borrow<MetadataTag> + Borrow<str> + Ord + Clone + TryFrom<MetadataTag> {
  fn as_str(&self) -> &'static str;
}

/// General purpose metadata tag type
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
pub enum MetadataTag {
  AcoustidFingerprint,
  AcoustidId,
  Album,
  AlbumArtist,
  AlbumArtistSort,
  AlbumSort,
  Arranger,
  Artist,
  Artists,
  ArtistSort,
  Asin,
  Barcode,
  Bpm,
  CatalogNumber,
  Comment,
  Compilation,
  Composer,
  ComposerSort,
  Conductor,
  Copyright,
  Date,
  Director,
  DiscNumber,
  DiscSubtitle,
  DjMixer,
  EncodedBy,
  EncoderSettings,
  Engineer,
  Genre,
  Grouping,
  Isrc,
  Key,
  Label,
  Language,
  License,
  Lyricist,
  Lyrics,
  Media,
  Mixer,
  Mood,
  Movement,
  MovementNumber,
  MovementTotal,
  OriginalAlbum,
  OriginalArtist,
  OriginalDate,
  OriginalFileName,
  OriginalYear,
  Performer,
  Producer,
  Rating,
  ReleaseCountry,
  ReleaseStatus,
  ReleaseType,
  Remixer,
  ReplayGainAlbumGain,
  ReplayGainAlbumPeak,
  ReplayGainAlbumRange,
  ReplayGainReferenceLoudness,
  ReplayGainTrackGain,
  ReplayGainTrackPeak,
  ReplayGainTrackRange,
  Script,
  ShowMovement,
  Subtitle,
  Title,
  TitleSort,
  TotalDiscs,
  TotalTracks,
  TrackNumber,
  Website,
  Work,
  Writer,
}

impl<T> PartialEq<T> for MetadataTag
where
  T: Metadata,
{
  #[inline]
  fn eq(&self, other: &T) -> bool {
    let tag: &MetadataTag = other.borrow();
    self.eq(tag)
  }
}
