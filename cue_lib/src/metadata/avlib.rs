use super::{MetadataTag, util::impl_metadata_mapping};
use crate::{core::cue_str::CueStr, metadata::error::InvalidMetadataTagName};

#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[derive(Debug, PartialEq)]
pub struct AvLibMetadata<'a> {
  pub tag: AvLibTag,
  pub value: CueStr<'a>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AvLibTag {
  inner: MetadataTag,
}

impl_metadata_mapping!(
  AvLibTag,
       "album"         => Album ,
       "album-sort"    => AlbumSort,
       "album_artist"  => AlbumArtist,
       "artist"        => Artist,
       "artist-sort"   => ArtistSort,
       "compilation"   => Compilation,
       "composer"      => Composer,
       "copyright"     => Copyright,
       "date"          => Date,
       "disc"          => DiscNumber,
       "encoded_by"    => EncodedBy,
       "encoder"       => EncoderSettings,
       "genre"         => Genre,
       "grouping"      => Grouping,
       "language"      => Language,
       "lyrics"        => Lyrics,
       "performer"     => Performer,
       "publisher"     => Label,
       "title"         => Title,
       "title-sort"    => TitleSort,
       "track"         => TrackNumber
);

impl TryFrom<MetadataTag> for AvLibTag {
  type Error = InvalidMetadataTagName;

  fn try_from(value: MetadataTag) -> Result<Self, Self::Error> {
    match value {
      MetadataTag::Album
      | MetadataTag::AlbumSort
      | MetadataTag::AlbumArtist
      | MetadataTag::Artist
      | MetadataTag::ArtistSort
      | MetadataTag::Compilation
      | MetadataTag::Composer
      | MetadataTag::Copyright
      | MetadataTag::Date
      | MetadataTag::DiscNumber
      | MetadataTag::EncodedBy
      | MetadataTag::EncoderSettings
      | MetadataTag::Genre
      | MetadataTag::Grouping
      | MetadataTag::Language
      | MetadataTag::Lyrics
      | MetadataTag::Performer
      | MetadataTag::Label
      | MetadataTag::Title
      | MetadataTag::TitleSort
      | MetadataTag::TrackNumber => Ok(AvLibTag { inner: value }),
      _ => Err(InvalidMetadataTagName),
    }
  }
}

#[cfg(test)]
mod test {
  use crate::metadata::avlib::LOOKUP_TABLE;

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
