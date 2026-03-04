use self::{
  args::Args,
  cli_error::cli_stderr,
  command::{Command, convert::ConvertCommand, verify::CommandVerify},
};
use std::{io::Read as _, path::Path, process::ExitCode};

pub mod args;
pub mod cli_error;
pub mod command;
pub mod output_writer;

#[inline]
fn read_cuesheet<T>(path: Option<T>) -> Result<String, std::io::Error>
where
  T: AsRef<Path>,
{
  match path {
    Some(path) => std::fs::read_to_string(path),
    None => {
      let mut buffer = String::new();
      let mut stdin = std::io::stdin();

      match stdin.read_to_string(&mut buffer) {
        Ok(_) => Ok(buffer),
        Err(err) => Err(err),
      }
    }
  }
}

fn main() -> ExitCode {
  let args = Args::init();
  let verbosity = args.verbose.unwrap_or_default();
  let cuesheet = match read_cuesheet(args.input.as_ref()) {
    Ok(buffer) => buffer,
    Err(err) => {
      cli_stderr!(err, input = "", verbosity = verbosity);
      return ExitCode::FAILURE;
    }
  };

  macro_rules! run {
    ($cmd:expr) => {
      match $cmd.run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
          cli_stderr!(err, input = cuesheet.as_str(), verbosity = verbosity);
          ExitCode::FAILURE
        }
      }
    };
  }

  match args.command {
    args::Commands::Verify => {
      let cmd = CommandVerify::new(cuesheet.as_str());
      run!(cmd)
    }
    args::Commands::ConvertJson {
      output_file,
      metadata,
      pretty_print,
    } => {
      let cmd = ConvertCommand::new(cuesheet.as_str())
        .set_vorbis_remarks(metadata)
        .set_output_file(output_file)
        .set_pretty_print(pretty_print);

      run!(cmd)
    }
    args::Commands::Query { input } => {
      todo!()
    }
  }
}
