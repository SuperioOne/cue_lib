#[cfg(feature = "serde")]
mod serde;
mod util;

#[cfg(feature = "alloc")]
pub mod map;

pub mod avlib;
pub mod error;
pub mod id3;
pub mod vorbis;

use core::borrow::Borrow;

pub trait Metadata: Borrow<MetadataTag> + Borrow<str> + Ord + Clone + TryFrom<MetadataTag> {
  fn as_str(&self) -> &'static str;
}

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
