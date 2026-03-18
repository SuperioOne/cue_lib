use cue_ffmpeg::{format::context::AvContext, util::dictionary::Iter};
use cue_lib::{
  core::cue_str::CueStr,
  metadata::{MetadataTag, avlib::AvLibTag, id3::Id3Tag, map::MetadataMap, vorbis::VorbisTag},
  parse::{CueSheet, TrackInfo},
};
use std::{
  ops::{Deref, DerefMut},
  str::FromStr as _,
};

pub struct MetadataContainer<'a> {
  inner: MetadataMap<'a, MetadataTag>,
}

impl<'a> Deref for MetadataContainer<'a> {
  type Target = MetadataMap<'a, MetadataTag>;

  #[inline]
  fn deref(&self) -> &Self::Target {
    &self.inner
  }
}

impl<'a> DerefMut for MetadataContainer<'a> {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.inner
  }
}

impl<'a> From<MetadataMap<'a, MetadataTag>> for MetadataContainer<'a> {
  #[inline]
  fn from(value: MetadataMap<'a, MetadataTag>) -> Self {
    Self { inner: value }
  }
}

macro_rules! insert_optional {
  ($self:expr, $tag:expr, $optional_val:expr) => {{
    if let Some(value) = $optional_val {
      _ = $self.insert($tag, value);
    }
  }};
}

impl<'a> MetadataContainer<'a> {
  #[inline]
  pub const fn new() -> Self {
    Self {
      inner: MetadataMap::new(),
    }
  }

  pub fn insert_from_av(&mut self, iter: Iter<'a>) {
    for (tag, value) in iter {
      match (tag.to_str(), CueStr::try_from(value)) {
        (Ok(tag), Ok(value)) => {
          let tag: Option<MetadataTag> = {
            if let Ok(av_tag) = AvLibTag::from_str(tag) {
              Some(av_tag.into())
            } else if let Ok(id3_tag) = Id3Tag::from_str(tag) {
              Some(id3_tag.into())
            } else if let Ok(vorbis) = VorbisTag::from_str(tag) {
              Some(vorbis.into())
            } else {
              None
            }
          };

          if let Some(tag) = tag {
            self.insert(tag, value);
          } else {
            continue;
          }
        }
        _ => continue,
      };
    }
  }

  pub fn insert_from_album(&mut self, cuesheet: &'a CueSheet) {
    insert_optional!(
      self,
      MetadataTag::OriginalFilename,
      cuesheet.file.map(|v| v.name)
    );

    insert_optional!(self, MetadataTag::Album, cuesheet.title);
    insert_optional!(self, MetadataTag::AlbumArtist, cuesheet.performer);
    insert_optional!(self, MetadataTag::CatalogNumber, cuesheet.catalog);
    insert_optional!(self, MetadataTag::Writer, cuesheet.songwriter);

    for (tag, values) in cuesheet.remark_metadata.iter() {
      for value in values {
        _ = self.insert(tag, *value);
      }
    }
  }

  pub fn insert_from_track(&mut self, cuesheet: &'a TrackInfo<'a>) {
    insert_optional!(self, MetadataTag::Title, cuesheet.title);
    insert_optional!(self, MetadataTag::Artist, cuesheet.performer);
    insert_optional!(self, MetadataTag::Writer, cuesheet.songwriter);

    for (tag, values) in cuesheet.remark_metadata.iter() {
      for value in values {
        _ = self.insert(tag, *value);
      }
    }
  }
}
