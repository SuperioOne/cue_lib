#![allow(nonstandard_style)]
use std::os::raw::c_int;

macro_rules! mktag {
  ($a:literal, $b:literal, $c:literal, $d:literal) => {
    (($a as i32) | (($b as i32) << 8) | (($c as i32) << 16) | (($d as i32) << 24))
  };
}

macro_rules! impl_av_errors {
  ($(
    $(#[$doc:meta])*
    ($variant_name:tt = $value:expr)
  ),+) => {
    #[repr(i32)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum AvLibError {
      $(
        $(#[$doc])*
        $variant_name = $value
      ),+
    }

    $(
        $(#[$doc])*
        const $variant_name: i32 = $value;
    )+

  };
}

impl_av_errors!(
  (BsfNotFound = mktag!(0xF8, 'B', 'S', 'F')),
  /// Internal bug, also see AVERROR_BUG2
  (Bug = mktag!('B', 'U', 'G', '!')),
  /// Buffer too small
  (BufferTooSmall = mktag!('B', 'U', 'F', 'S')),
  /// Decoder not found
  (DecoderNotFound = mktag!(0xF8, 'D', 'E', 'C')),
  /// Demuxer not found
  (DemuxerNotFound = mktag!(0xF8, 'D', 'E', 'M')),
  /// Encoder not found
  (EncoderNotFound = mktag!(0xF8, 'E', 'N', 'C')),
  /// End of file
  (Eof = mktag!('E', 'O', 'F', ' ')),
  /// Immediate exit was requested; the called function should not be restarted
  (Exit = mktag!('E', 'X', 'I', 'T')),
  /// Generic error in an external library
  (External = mktag!('E', 'X', 'T', ' ')),
  /// Filter not found
  (FilterNotFound = mktag!(0xF8, 'F', 'I', 'L')),
  /// Invalid data found when processing input
  (InvalidData = mktag!('I', 'N', 'D', 'A')),
  /// Muxer not found
  (MuxerNotFound = mktag!(0xF8, 'M', 'U', 'X')),
  /// Option not found
  (OptionNotFound = mktag!(0xF8, 'O', 'P', 'T')),
  /// Not yet implemented in FFmpeg, patches welcome
  (PatchWelcome = mktag!('P', 'A', 'W', 'E')),
  /// Protocol not found
  (ProtocolNotFound = mktag!(0xF8, 'P', 'R', 'O')),
  /// Stream not found
  (StreamNotFound = mktag!(0xF8, 'S', 'T', 'R')),
  (Bug2 = mktag!('B', 'U', 'G', ' ')),
  /// Unknown error, typically from an external library
  (Unknown = mktag!('U', 'N', 'K', 'N')),
  /// Requested feature is flagged experimental. Set strict_std_compliance if you really want to use it.
  (Experimental = 0x2bb2afa8),
  /// Input changed between calls. Reconfiguration is required. (can be OR-ed with AVERROR_OUTPUT_CHANGED)
  (InputChanged = 0x636e6701),
  /// Output changed between calls. Reconfiguration is required. (can be OR-ed with AVERROR_INPUT_CHANGED)
  (OutputChanged = 0x636e6702),
  (HttpBadRequest = mktag!(0xF8, '4', '0', '0')),
  (HttpUnauthorized = mktag!(0xF8, '4', '0', '1')),
  (HttpForbidden = mktag!(0xF8, '4', '0', '3')),
  (HttpNotFound = mktag!(0xF8, '4', '0', '4')),
  (HttpTooManyRequests = mktag!(0xF8, '4', '2', '9')),
  (HttpOther4xx = mktag!(0xF8, '4', 'X', 'X')),
  (HttpServerError = mktag!(0xF8, '5', 'X', 'X'))
);

#[derive(Debug)]
pub enum AvError {
  UninitializedDictionary,
  InvalidFrameSize,
  AllocationError,
  DumpError,
  IOError(std::io::Error),
  AvLibError(AvLibError),
}

impl From<std::io::Error> for AvError {
  #[inline]
  fn from(value: std::io::Error) -> Self {
    Self::IOError(value)
  }
}

impl From<std::io::ErrorKind> for AvError {
  #[inline]
  fn from(value: std::io::ErrorKind) -> Self {
    Self::IOError(std::io::Error::from(value))
  }
}

impl From<AvLibError> for AvError {
  #[inline]
  fn from(value: AvLibError) -> Self {
    Self::AvLibError(value)
  }
}

impl AvError {
  pub fn from_raw_err_code(code: c_int) -> Result<(), Self> {
    if code > -1 {
      Ok(())
    } else {
      // NOTE: Yes, code sign is inverted on purpose. See POSIX errors.
      match -code {
        BsfNotFound => Err(AvLibError::BsfNotFound.into()),
        Bug => Err(AvLibError::Bug.into()),
        BufferTooSmall => Err(AvLibError::BufferTooSmall.into()),
        DecoderNotFound => Err(AvLibError::DecoderNotFound.into()),
        DemuxerNotFound => Err(AvLibError::DemuxerNotFound.into()),
        EncoderNotFound => Err(AvLibError::EncoderNotFound.into()),
        Eof => Err(AvLibError::Eof.into()),
        Exit => Err(AvLibError::Exit.into()),
        External => Err(AvLibError::External.into()),
        FilterNotFound => Err(AvLibError::FilterNotFound.into()),
        InvalidData => Err(AvLibError::InvalidData.into()),
        MuxerNotFound => Err(AvLibError::MuxerNotFound.into()),
        OptionNotFound => Err(AvLibError::OptionNotFound.into()),
        PatchWelcome => Err(AvLibError::PatchWelcome.into()),
        ProtocolNotFound => Err(AvLibError::ProtocolNotFound.into()),
        StreamNotFound => Err(AvLibError::StreamNotFound.into()),
        Bug2 => Err(AvLibError::Bug2.into()),
        Unknown => Err(AvLibError::Unknown.into()),
        Experimental => Err(AvLibError::Experimental.into()),
        OutputChanged => Err(AvLibError::OutputChanged.into()),
        InputChanged => Err(AvLibError::InputChanged.into()),
        HttpBadRequest => Err(AvLibError::HttpBadRequest.into()),
        HttpUnauthorized => Err(AvLibError::HttpUnauthorized.into()),
        HttpForbidden => Err(AvLibError::HttpForbidden.into()),
        HttpNotFound => Err(AvLibError::HttpNotFound.into()),
        HttpTooManyRequests => Err(AvLibError::HttpTooManyRequests.into()),
        HttpOther4xx => Err(AvLibError::HttpOther4xx.into()),
        HttpServerError => Err(AvLibError::HttpServerError.into()),
        value => Err(AvError::IOError(std::io::Error::from_raw_os_error(value))),
      }
    }
  }
}
