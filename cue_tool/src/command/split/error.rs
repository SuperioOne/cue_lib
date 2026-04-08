use crate::{args::VerboseLevel, cli_error::ErrorFormat};
use cue_ffmpeg::error::AvError;
use cue_lib::error::CueLibError;
use std::path::PathBuf;

pub enum SplitError {
  AvError(AvError),
  CueLibError(CueLibError),
  IOError(std::io::Error),
  InvalidFilePath(PathBuf),
  InvalidOutputDir(PathBuf),
  UnsupportedAvLibVersion,
  NothingToSplit,
  MultipleAudioStream,
  UnknownAudioContainer,
}

impl ErrorFormat for SplitError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
    input_buffer: &str,
    verbose_level: crate::args::VerboseLevel,
  ) -> std::fmt::Result {
    if verbose_level == VerboseLevel::Quiet {
      Ok(())
    } else {
      match self {
        Self::CueLibError(error) => ErrorFormat::fmt(error, f, input_buffer, verbose_level),
        Self::IOError(error) => std::fmt::Display::fmt(&error, f),
        Self::UnsupportedAvLibVersion => f.write_str("linked avlib version on system is not supported"),
        Self::NothingToSplit => f.write_str("cuesheet has only one or none track, nothing to split."),
        Self::AvError(error) => std::fmt::Display::fmt(&error, f),
        Self::UnknownAudioContainer => f.write_str("unable to detect audio codec"),
        Self::InvalidFilePath(path) => f.write_fmt(format_args!("invalid file path: {:?}", path)),
        Self::InvalidOutputDir(path) => f.write_fmt(format_args!("invalid output directory: {:?}", path)),
        Self::MultipleAudioStream => f.write_str("media container has multiple audio stream, this tool is not designed to handle such a container type")
      }
    }
  }
}

impl From<CueLibError> for SplitError {
  #[inline]
  fn from(value: CueLibError) -> Self {
    Self::CueLibError(value)
  }
}

impl From<AvError> for SplitError {
  #[inline]
  fn from(value: AvError) -> Self {
    Self::AvError(value)
  }
}

impl From<std::io::Error> for SplitError {
  #[inline]
  fn from(value: std::io::Error) -> Self {
    Self::IOError(value)
  }
}
