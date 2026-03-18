use cue_ffmpeg_sys::{avcodec_version, avformat_version, avutil_version};

pub mod codec;
pub mod common;
pub mod error;
pub mod format;
pub mod util;

// FORMAT

// CODEC

pub mod ffmpeg {
  pub use cue_ffmpeg_sys::*;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VersionInfo {
  pub avutil: Version,
  pub avformat: Version,
  pub avcodec: Version,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version(u32);

pub fn linked_lib_version() -> VersionInfo {
  VersionInfo {
    avutil: Version(unsafe { avutil_version() }),
    avformat: Version(unsafe { avformat_version() }),
    avcodec: Version(unsafe { avcodec_version() }),
  }
}

impl Version {
  #[inline]
  pub const fn minor(&self) -> u8 {
    (self.0 >> 8 & 0xFF) as u8
  }

  #[inline]
  pub const fn major(&self) -> u8 {
    (self.0 >> 16 & 0xFF) as u8
  }

  #[inline]
  pub const fn micro(&self) -> u8 {
    (self.0 & 0xFF) as u8
  }

  #[inline]
  pub const fn as_u32(&self) -> u32 {
    self.0
  }
}

impl std::fmt::Display for Version {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_fmt(format_args!(
      "{}.{}.{}",
      self.major(),
      self.minor(),
      self.micro()
    ))
  }
}
