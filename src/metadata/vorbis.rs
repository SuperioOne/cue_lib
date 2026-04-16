use super::{
  MetadataTag,
  util::{impl_metadata_mapping, split_metadata_line},
};
use crate::{core::CueStr, metadata::error::InvalidMetadataTagName};
use core::str::FromStr;

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, PartialEq)]
pub struct VorbisComment<'a> {
  pub tag: VorbisTag,
  pub value: CueStr<'a>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VorbisTag {
  inner: MetadataTag,
}

impl_metadata_mapping!(
  VorbisTag,
    "ACOUSTID_FINGERPRINT"          => AcoustidFingerprint,
    "ACOUSTID_ID"                   => AcoustidId,
    "ALBUM"                         => Album,
    "ALBUMARTIST"                   => AlbumArtist,
    "ALBUMARTISTSORT"               => AlbumArtistSort,
    "ALBUMSORT"                     => AlbumSort,
    "ARRANGER"                      => Arranger,
    "ARTIST"                        => Artist,
    "ARTISTS"                       => Artists,
    "ARTISTSORT"                    => ArtistSort,
    "ASIN"                          => Asin,
    "BARCODE"                       => Barcode,
    "BPM"                           => Bpm,
    "CATALOGNUMBER"                 => CatalogNumber,
    "COMMENT"                       => Comment,
    "COMPILATION"                   => Compilation,
    "COMPOSER"                      => Composer,
    "COMPOSERSORT"                  => ComposerSort,
    "CONDUCTOR"                     => Conductor,
    "COPYRIGHT"                     => Copyright,
    "DATE"                          => Date,
    "DIRECTOR"                      => Director,
    "DISCNUMBER"                    => DiscNumber,
    "DISCSUBTITLE"                  => DiscSubtitle,
    "DISCTOTAL"                     => TotalDiscs,
    "DJMIXER"                       => DjMixer,
    "ENCODEDBY"                     => EncodedBy,
    "ENCODERSETTINGS"               => EncoderSettings,
    "ENGINEER"                      => Engineer,
    "GENRE"                         => Genre,
    "GROUPING"                      => Grouping,
    "ISRC"                          => Isrc,
    "KEY"                           => Key,
    "LABEL"                         => Label,
    "LANGUAGE"                      => Language,
    "LICENSE"                       => License,
    "LYRICIST"                      => Lyricist,
    "LYRICS"                        => Lyrics,
    "MEDIA"                         => Media,
    "MIXER"                         => Mixer,
    "MOOD"                          => Mood,
    "MOVEMENT"                      => MovementNumber,
    "MOVEMENTNAME"                  => Movement,
    "MOVEMENTTOTAL"                 => MovementTotal,
    "ORIGINALDATE"                  => OriginalDate,
    "ORIGINALFILENAME"              => OriginalFileName,
    "ORIGINALYEAR"                  => OriginalYear,
    "PERFORMER"                     => Performer,
    "PRODUCER"                      => Producer,
    "RATING"                        => Rating,
    "RELEASECOUNTRY"                => ReleaseCountry,
    "RELEASESTATUS"                 => ReleaseStatus,
    "RELEASETYPE"                   => ReleaseType,
    "REMIXER"                       => Remixer,
    "REPLAYGAIN_ALBUM_GAIN"         => ReplayGainAlbumGain,
    "REPLAYGAIN_ALBUM_PEAK"         => ReplayGainAlbumPeak,
    "REPLAYGAIN_ALBUM_RANGE"        => ReplayGainAlbumRange,
    "REPLAYGAIN_REFERENCE_LOUDNESS" => ReplayGainReferenceLoudness,
    "REPLAYGAIN_TRACK_GAIN"         => ReplayGainTrackGain,
    "REPLAYGAIN_TRACK_PEAK"         => ReplayGainTrackPeak,
    "REPLAYGAIN_TRACK_RANGE"        => ReplayGainTrackRange,
    "SCRIPT"                        => Script,
    "SHOWMOVEMENT"                  => ShowMovement,
    "SUBTITLE"                      => Subtitle,
    "TITLE"                         => Title,
    "TITLESORT"                     => TitleSort,
    "TRACKNUMBER"                   => TrackNumber,
    "TRACKTOTAL"                    => TotalTracks,
    "WEBSITE"                       => Website,
    "WORK"                          => Work,
    "WRITER"                        => Writer
);

impl<'a> VorbisComment<'a> {
  pub fn try_from_line(line: &'a str) -> Result<Self, InvalidMetadataTagName> {
    let (key, value) = split_metadata_line(line)?;
    let tag = match key {
      CueStr::Text(v) => VorbisTag::from_str(v),
      _ => Err(InvalidMetadataTagName),
    }?;

    Ok(VorbisComment { value, tag })
  }
}

#[cfg(test)]
mod test {
  use crate::metadata::vorbis::LOOKUP_TABLE;

  #[test]
  fn is_inner_table_sorted() {
    let is_sorted = LOOKUP_TABLE.is_sorted_by(|a, b| match a.0.cmp(b.0) {
      core::cmp::Ordering::Greater => false,
      _ => true,
    });

    assert!(
      is_sorted,
      "make sure the impl_metadata_mapping tags are sorted on the code"
    );
  }
}
