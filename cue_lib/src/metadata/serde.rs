use serde::{Serialize, ser::SerializeStruct as _};

impl Serialize for crate::metadata::VorbisTagName {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_str(self.as_str())
  }
}

#[cfg(feature = "alloc")]
impl<'a> Serialize for super::MetadataMap<'a> {
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
