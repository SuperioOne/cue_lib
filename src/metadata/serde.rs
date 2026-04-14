use super::{Metadata, vorbis::VorbisTag};
use serde::{Serialize, ser::SerializeStruct as _};

impl Serialize for VorbisTag {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.as_str())
  }
}

impl Serialize for crate::metadata::id3::Id3Tag {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.as_ref())
  }
}

impl Serialize for crate::metadata::avlib::AvLibTag {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.as_ref())
  }
}

#[cfg(feature = "alloc")]
impl<'a, T> Serialize for super::map::MetadataMap<'a, T>
where
  T: Metadata,
{
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
