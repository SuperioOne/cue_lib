use super::Command;
use cue_lib::parse::CueSheetParser;

pub struct CmdQuery<'a> {
  cuesheet: &'a str,
  query: &'a str,
  vorbis_remarks: bool,
}

impl<'a> CmdQuery<'a> {
  #[inline]
  pub const fn new(cuesheet: &'a str, query: &'a str) -> Self {
    Self {
      cuesheet,
      query,
      vorbis_remarks: false,
    }
  }

  #[inline]
  pub const fn set_vorbis_remarks(mut self, value: bool) -> Self {
    self.vorbis_remarks = value;
    self
  }
}

impl<'a> Command for &'a CmdQuery<'a> {
  type Error = cue_lib::error::CueLibError;

  #[inline]
  fn run(self) -> Result<(), cue_lib::error::CueLibError> {
    let cuesheet = CueSheetParser::new()
      .allow_vorbis_remarks(self.vorbis_remarks)
      .parse(self.cuesheet)?;

    println!("{}", self.query);
    Ok(())
  }
}
