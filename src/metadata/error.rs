#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct InvalidMetadataTagName;

impl core::fmt::Display for InvalidMetadataTagName {
  fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
    f.write_str("invalid metadata tag name")
  }
}

impl core::error::Error for InvalidMetadataTagName {}
