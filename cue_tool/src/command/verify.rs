use super::Command;
use cue_lib::probe::CueSheetProbe;

pub struct CmdVerify<'a> {
  cuesheet: &'a str,
}

impl<'a> CmdVerify<'a> {
  #[inline]
  pub const fn new(cuesheet: &'a str) -> Self {
    Self { cuesheet }
  }
}

impl<'a> Command for &'a CmdVerify<'a> {
  type Error = cue_lib::error::CueLibError;

  #[inline]
  fn run(self) -> Result<(), cue_lib::error::CueLibError> {
    CueSheetProbe::verify(self.cuesheet)
  }
}
