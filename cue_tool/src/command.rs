use crate::cli_error::ErrorFormat;

pub mod convert;
pub mod query;
pub mod split;
pub mod test;

pub trait Command
where
  Self::Error: ErrorFormat,
{
  type Error;

  fn run(self) -> Result<(), Self::Error>;
}
