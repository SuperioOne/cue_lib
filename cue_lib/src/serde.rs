use crate::{
  core::{
    album_file::{AlbumFile, KnownFileType},
    cue_str::CueStr,
    flags::TrackFlag,
    timestamp::CueTimeStamp,
    track::{DataType, IndexNo, Track, TrackIndex, TrackNo},
  },
  discid::{ean::Ean13, isrc::Isrc, upc::UpcA},
  metadata::{MetadataMap, VorbisComment, VorbisTagName},
};
use alloc::borrow::Cow;
use serde::{Serialize, ser::SerializeStruct};

impl<'a> Serialize for CueStr<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let value: Cow<'a, str> = self.into();
    serializer.serialize_str(value.as_ref())
  }
}

impl Serialize for Isrc {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let ascii_bytes = self.as_ascii_bytes();
    let isrc_str = unsafe { core::str::from_utf8_unchecked(&ascii_bytes) };
    serializer.serialize_str(isrc_str)
  }
}

impl Serialize for Ean13 {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let ascii_bytes = self.as_ascii_bytes();
    let ean_str = unsafe { core::str::from_utf8_unchecked(&ascii_bytes) };
    serializer.serialize_str(ean_str)
  }
}

impl Serialize for UpcA {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let ascii_bytes = self.as_ascii_bytes();
    let upc_str = unsafe { core::str::from_utf8_unchecked(&ascii_bytes) };
    serializer.serialize_str(upc_str)
  }
}

impl Serialize for TrackNo {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_u8(self.into_inner())
  }
}

impl Serialize for IndexNo {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_u8(self.into_inner())
  }
}

impl Serialize for CueTimeStamp {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_u128(self.as_millis())
  }
}

impl Serialize for KnownFileType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.as_str())
  }
}

impl Serialize for TrackFlag {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.collect_seq(self.iter())
  }
}

impl Serialize for DataType {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.as_str())
  }
}

impl Serialize for VorbisTagName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.as_str())
  }
}

impl Serialize for TrackIndex {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut obj = serializer.serialize_struct("TrackIndex", 2)?;
    obj.serialize_field("index_no", &self.index_no)?;
    obj.serialize_field("timestamp", &self.timestamp)?;
    obj.end()
  }
}

impl Serialize for Track {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut obj = serializer.serialize_struct("Track", 2)?;
    obj.serialize_field("track_no", &self.track_no)?;
    obj.serialize_field("data_type", &self.data_type)?;
    obj.end()
  }
}

impl<'a> Serialize for VorbisComment<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut obj = serializer.serialize_struct("VorbisComment", 2)?;
    obj.serialize_field("value", &self.value)?;
    obj.serialize_field("tag", &self.tag)?;
    obj.end()
  }
}

impl<'a> Serialize for AlbumFile<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut obj = serializer.serialize_struct("AlbumFile", 2)?;
    obj.serialize_field("file_type", &self.file_type)?;
    obj.serialize_field("name", &self.name)?;
    obj.end()
  }
}

impl<'a> Serialize for MetadataMap<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    let mut obj = serializer.serialize_struct("MetadataMap", self.len())?;

    for (key, values) in self.iter() {
      obj.serialize_field(key.as_str(), &values)?;
    }

    obj.end()
  }
}
