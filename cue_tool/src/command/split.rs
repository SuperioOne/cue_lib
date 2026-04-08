use self::{error::SplitError, transcode::SplitTranscoder};
use super::Command;
use cue_ffmpeg::{
  avlib_version,
  ffmpeg::{LIBAVCODEC_VERSION_MAJOR, LIBAVFORMAT_VERSION_MAJOR},
};
use cue_lib::parse::{CueSheet, CueSheetParser};
use std::path::{Path, PathBuf};

pub mod error;
mod metadata;
mod transcode;

pub struct CmdSplit<'a> {
  cuesheet: &'a str,
  output_dir: Option<PathBuf>,
  input_path: Option<PathBuf>,
  vorbis_remarks: bool,
}

impl<'a> CmdSplit<'a> {
  pub const fn new(cuesheet: &'a str) -> Self {
    Self {
      cuesheet,
      output_dir: None,
      input_path: None,
      vorbis_remarks: false,
    }
  }

  #[inline]
  pub fn set_input_path(mut self, value: Option<PathBuf>) -> Self {
    self.input_path = value;
    self
  }

  #[inline]
  pub const fn set_vorbis_remarks(mut self, value: bool) -> Self {
    self.vorbis_remarks = value;
    self
  }

  #[inline]
  pub fn set_output_dir(mut self, value: Option<PathBuf>) -> Self {
    self.output_dir = value;
    self
  }

  #[inline]
  fn resolve_file_path(&self, cuesheet: &CueSheet<'_>) -> Result<PathBuf, SplitError> {
    match self.input_path.as_ref() {
      Some(path) => {
        if path.is_dir() {
          if let Some(file_cmd) = cuesheet.file {
            let file_name = file_cmd.name.to_string();
            Ok(path.join(Path::new(&file_name)))
          } else {
            Err(SplitError::InvalidFilePath(path.clone()))
          }
        } else if path.is_file() {
          Ok(path.clone())
        } else {
          Err(SplitError::InvalidFilePath(path.clone()))
        }
      }
      None => {
        if let Some(file_cmd) = cuesheet.file {
          let file_name = file_cmd.name.to_string();
          let file_path = Path::new(&file_name);

          if file_path.is_file() {
            Ok(file_path.to_path_buf())
          } else {
            Err(SplitError::InvalidFilePath(file_path.to_path_buf()))
          }
        } else {
          Err(SplitError::InvalidFilePath(PathBuf::new()))
        }
      }
    }
  }
}

impl<'a> Command for CmdSplit<'a> {
  type Error = SplitError;

  fn run(self) -> Result<(), Self::Error> {
    let av_version = avlib_version();

    if av_version.avformat.major() != LIBAVFORMAT_VERSION_MAJOR as u8
      || av_version.avcodec.major() != LIBAVCODEC_VERSION_MAJOR as u8
    {
      return Err(SplitError::UnsupportedAvLibVersion);
    }

    let cuesheet = CueSheetParser::new()
      .allow_vorbis_remarks(self.vorbis_remarks)
      .parse(self.cuesheet)?;

    let input_path = self.resolve_file_path(&cuesheet)?;

    let output_dir = if let Some(output_dir) = self.output_dir {
      output_dir
    } else {
      PathBuf::from(".")
    };

    SplitTranscoder::init(input_path, output_dir, &cuesheet)?.split()
  }
}
