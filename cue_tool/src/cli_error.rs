use crate::args::VerboseLevel;
use std::{
  fmt::{Debug, Display},
  io::{IsTerminal, stderr},
};

#[derive(Debug)]
pub struct CliError<'a, E> {
  verbose_level: VerboseLevel,
  input_buffer: &'a str,
  error: E,
}

impl<'a, E> CliError<'a, E>
where
  E: ErrorFormat,
{
  #[inline]
  pub const fn new(error: E, input_buffer: &'a str) -> Self {
    Self {
      error,
      verbose_level: VerboseLevel::Default,
      input_buffer,
    }
  }

  #[inline]
  pub const fn new_with_verbose(
    error: E,
    input_buffer: &'a str,
    verbose_level: VerboseLevel,
  ) -> Self {
    Self {
      error,
      verbose_level,
      input_buffer,
    }
  }
}

impl<'a, E> core::fmt::Display for CliError<'a, E>
where
  E: ErrorFormat,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    self.error.fmt(f, self.input_buffer, self.verbose_level)
  }
}

pub trait ErrorFormat {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
    input_buffer: &str,
    verbose_level: VerboseLevel,
  ) -> std::fmt::Result;
}

impl ErrorFormat for std::io::Error {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
    _: &str,
    verbose_level: VerboseLevel,
  ) -> std::fmt::Result {
    match verbose_level {
      VerboseLevel::Quiet => Ok(()),
      _ => std::fmt::Display::fmt(&self, f),
    }
  }
}

struct AnsiCodes {
  error: &'static str,
  reset: &'static str,
  warning: &'static str,
  info: &'static str,
}

impl Default for AnsiCodes {
  fn default() -> Self {
    Self {
      error: Default::default(),
      reset: Default::default(),
      warning: Default::default(),
      info: Default::default(),
    }
  }
}

impl ErrorFormat for cue_lib::error::CueLibError {
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
    input_buffer: &str,
    verbose_level: VerboseLevel,
  ) -> std::fmt::Result {
    match self.kind() {
      cue_lib::error::CueLibErrorKind::ParseError(parse_error) => match verbose_level {
        VerboseLevel::Quiet => Ok(()),
        VerboseLevel::Default => std::fmt::Display::fmt(&self, f),
        VerboseLevel::Full => {
          let ansi = if stderr().is_terminal() {
            AnsiCodes {
              error: "\x1B[1;91m",
              info: "\x1B[1;96m",
              warning: "\x1B[1;93m",
              reset: "\x1B[0m",
            }
          } else {
            AnsiCodes::default()
          };

          let line_idx = parse_error.line();
          let line_no = line_idx + 1;
          let start = line_idx.saturating_sub(10);

          for (rel_idx, line) in input_buffer.lines().skip(start).take(20).enumerate() {
            if rel_idx + start == line_idx {
              let column_idx = if parse_error.column() > 0 {
                parse_error.column()
              } else {
                line.chars().take_while(|v| v.is_whitespace()).count()
              };
              let column = column_idx + 1;

              eprintln!(
                "{ansi_err}{line}{reset}",
                ansi_err = ansi.error,
                reset = ansi.reset
              );

              if column_idx > 0 {
                eprint!("{:>column_idx$}", ' ')
              }

              eprintln!(
                "{ansi_warn}^{dash:->24}{self} at {line_no}:{column}{reset}",
                dash = ' ',
                ansi_warn = ansi.warning,
                reset = ansi.reset
              );
            } else {
              eprintln!("{line}");
            }
          }

          if let Some((_, line)) = input_buffer
            .lines()
            .enumerate()
            .find(|(idx, _)| *idx == line_idx)
          {
            eprintln!(
              "\n{ansi_info}Line {line_no}: UTF-8 Character Breakdown{reset}",
              ansi_info = ansi.info,
              reset = ansi.reset,
            );
            eprintln!("\n  Column | UTF-8 Character");
            eprintln!("  -------|----------------");

            for (idx, char) in line.chars().enumerate() {
              eprintln!(
                "  {column:<7}| {ansi_info}{char:?}{reset}",
                column = idx + 1,
                ansi_info = ansi.info,
                reset = ansi.reset,
              );
            }
          }

          Ok(())
        }
      },
    }
  }
}

impl<'a, E> core::error::Error for CliError<'a, E> where E: ErrorFormat + Display + Debug {}

macro_rules! cli_stderr {
  ($error:expr, input = $input:expr) => {{
    let error = $crate::cli_error::CliError::new($error, $input);
    eprintln!("{}", $error)
  }};

  ($error:expr, input = $input:expr, verbosity = $verbosity:expr) => {{
    let error = $crate::cli_error::CliError::new_with_verbose($error, $input, $verbosity);
    eprintln!("{}", error);
  }};

  (message = $error:expr, verbosity = $verbosity:expr) => {{
    match $verbosity {
      $crate::args::VerboseLevel::Quiet => {}
      _ => eprintln!("{}", $error),
    }
  }};
}

pub(crate) use cli_stderr;
