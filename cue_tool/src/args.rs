use clap::{Parser, Subcommand, ValueEnum};
use std::{ffi::OsString, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  /// Cuesheet file path
  #[arg(short, long)]
  pub input: Option<PathBuf>,

  /// Verbosity level
  #[arg(long)]
  pub verbose: Option<VerboseLevel>,

  #[command(subcommand)]
  pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
  /// Verifies input cuesheet syntax
  Test,
  /// Parses cuesheet and serializes data as structured JSON string
  ConvertJson {
    #[arg(short, long)]
    output_file: Option<PathBuf>,
    /// Enables Vorbis metadata comments from remarks
    #[arg(short, long)]
    metadata: bool,
    /// Formats JSON output
    #[arg(short, long)]
    pretty_print: bool,
  },
  /// jq like basic filter to print data from cuesheet
  Query {
    input: OsString,
    /// Enables Vorbis metadata comments from remarks
    #[arg(short, long)]
    metadata: bool,
  },
  /// Splits target file into multiple tracks based on cuesheet
  Split {
    /// Root directory for the input file or, FILE path
    #[arg(long)]
    input_path: Option<PathBuf>,
    /// Output directory for the split tracks
    #[arg(short, long)]
    output_dir: Option<PathBuf>,
    /// Enables Vorbis metadata comments from remarks
    #[arg(short, long)]
    metadata: bool,
  },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum VerboseLevel {
  Default,
  Full,
  Quiet,
}

impl Args {
  #[inline]
  pub fn init() -> Self {
    Self::parse()
  }
}

impl Default for VerboseLevel {
  #[inline]
  fn default() -> Self {
    Self::Default
  }
}
