use super::{MetadataTag, util::impl_metadata_mapping};
use crate::{core::cue_str::CueStr, metadata::error::InvalidMetadataTagName};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, PartialEq)]
pub struct Id3Metadata<'a> {
  pub tag: Id3Tag,
  pub value: CueStr<'a>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Id3Tag {
  inner: MetadataTag,
}

impl_metadata_mapping!(
  Id3Tag,
    "COMM:description"                   => Comment,
    "MVIN"                               => MovementNumber,
    "MVIN"                               => MovementTotal,
    "MVNM"                               => Movement,
    "POPM"                               => Rating,
    "TALB"                               => Album,
    "TBPM"                               => Bpm,
    "TCMP"                               => Compilation,
    "TCOM"                               => Composer,
    "TCON"                               => Genre,
    "TCOP"                               => Copyright,
    "TDOR"                               => OriginalDate,
    "TDRC"                               => Date,
    "TENC"                               => EncodedBy,
    "TEXT"                               => Lyricist,
    "TIPL:DJ-mix"                        => DjMixer,
    "TIPL:arranger"                      => Arranger,
    "TIPL:engineer"                      => Engineer,
    "TIPL:mix"                           => Mixer,
    "TIPL:producer"                      => Producer,
    "TIT1"                               => Grouping,
    "TIT2"                               => Title,
    "TIT3"                               => Subtitle,
    "TKEY"                               => Key,
    "TLAN"                               => Language,
    "TMCL:instrument"                    => Performer,
    "TMED"                               => Media,
    "TMOO"                               => Mood,
    "TOAL"                               => OriginalAlbum,
    "TOFN"                               => OriginalFileName,
    "TOPE"                               => OriginalArtist,
    "TPE1"                               => Artist,
    "TPE2"                               => AlbumArtist,
    "TPE3"                               => Conductor,
    "TPE4"                               => Remixer,
    "TPOS"                               => DiscNumber,
    "TPUB"                               => Label,
    "TRCK"                               => TrackNumber,
    "TSO2"                               => AlbumArtistSort,
    "TSOA"                               => AlbumSort,
    "TSOC"                               => ComposerSort,
    "TSOP"                               => ArtistSort,
    "TSOT"                               => TitleSort,
    "TSRC"                               => Isrc,
    "TSSE"                               => EncoderSettings,
    "TSST"                               => DiscSubtitle,
    "TXXX:ASIN"                          => Asin,
    "TXXX:Acoustid Fingerprint"          => AcoustidFingerprint,
    "TXXX:Acoustid Id"                   => AcoustidId,
    "TXXX:Artists"                       => Artists,
    "TXXX:BARCODE"                       => Barcode,
    "TXXX:CATALOGNUMBER"                 => CatalogNumber,
    "TXXX:DIRECTOR"                      => Director,
    "TXXX:LICENSE"                       => License,
    "TXXX:REPLAYGAIN_ALBUM_GAIN"         => ReplayGainAlbumGain,
    "TXXX:REPLAYGAIN_ALBUM_PEAK"         => ReplayGainAlbumPeak,
    "TXXX:REPLAYGAIN_ALBUM_RANGE"        => ReplayGainAlbumRange,
    "TXXX:REPLAYGAIN_REFERENCE_LOUDNESS" => ReplayGainReferenceLoudness,
    "TXXX:REPLAYGAIN_TRACK_GAIN"         => ReplayGainTrackGain ,
    "TXXX:REPLAYGAIN_TRACK_PEAK"         => ReplayGainTrackPeak,
    "TXXX:REPLAYGAIN_TRACK_RANGE"        => ReplayGainTrackRange,
    "TXXX:SCRIPT"                        => Script,
    "TXXX:SHOWMOVEMENT"                  => ShowMovement,
    "TXXX:WORK TIT1"                     => Work,
    "TXXX:Writer"                        => Writer,
    "USLT:description"                   => Lyrics,
    "WOAR"                               => Website
);

#[cfg(test)]
mod test {
  use crate::metadata::id3::LOOKUP_TABLE;

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
