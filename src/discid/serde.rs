use super::{ean::Ean13, isrc::Isrc, upc::UpcA};
use serde::Serialize;

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
