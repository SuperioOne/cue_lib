use super::error::SplitError;
use cue_ffmpeg::{
  codec::{context::AvCodecContext, frame::AvFrame, packet::AvPacket},
  error::{AvError, AvLibError},
  ffmpeg::{
    AV_CODEC_FLAG_GLOBAL_HEADER, AVCodecID_AV_CODEC_ID_FLAC, AVCodecID_AV_CODEC_ID_MP3,
    AVCodecID_AV_CODEC_ID_MP3ADU, AVCodecID_AV_CODEC_ID_MP3ON4, AVFMT_GLOBALHEADER, AVStream,
  },
  format::{
    context::{AvInputContext, AvOutputContext},
    stream::{StreamType, copy_stream_properties},
  },
  util::{audio_fifo::AudioFifo, dictionary::AvDictionaryRef, timestamp::AvTimestamp},
};
use cue_lib::{core::cue_str::CueStr, parse::CueSheet};
use std::{ffi::CStr, io::ErrorKind, path::Path};

macro_rules! static_cstr {
  ($value:literal) => {
    unsafe { &CStr::from_bytes_with_nul_unchecked(concat!($value, "\0").as_bytes()) }
  };
}

const COVER_IMAGE_KEY: &'static CStr = static_cstr!("comment");
const COVER_IMAGE_VALUE: &'static CStr = static_cstr!("Cover (front)");
const UNTITLED_TRACK: CueStr<'static> = CueStr::Text("untitled");
const EXT_FLAC: &'static str = "flac";
const EXT_MP3: &'static str = "mp3";

struct SplitOutput {
  start_time: AvTimestamp,
  end_time: Option<AvTimestamp>,
  context: AvOutputContext,
  encoder: AvCodecContext,
  samples: AudioFifo,
}

pub struct SplitTranscoder {
  input: AvInputContext,
  outputs: Vec<SplitOutput>,
  audio_stream_index: i32,
  audio_decoder: AvCodecContext,
  cover_image_stream_index: Option<i32>,
  cover_image_packets: Vec<AvPacket>,
}

impl SplitTranscoder {
  pub fn init<I, O>(
    input_path: I,
    output_dir: O,
    cuesheet: &CueSheet<'_>,
  ) -> Result<Self, SplitError>
  where
    I: AsRef<Path>,
    O: AsRef<Path>,
  {
    if cuesheet.tracks.len() < 2 {
      return Err(SplitError::NothingToSplit);
    }

    let input = AvInputContext::open_path(input_path.as_ref())?;

    if output_dir.as_ref().exists() {
      if !output_dir.as_ref().is_dir() {
        return Err(SplitError::InvalidOutputDir(
          output_dir.as_ref().to_path_buf(),
        ));
      }
    } else {
      std::fs::create_dir_all(&output_dir)?;
    }

    let audio_stream = input
      .find_best_stream(StreamType::Audio)
      .ok_or(SplitError::NothingToSplit)
      .and_then(|v| {
        if v.codecpar.is_null() {
          Err(SplitError::UnknownAudioContainer)
        } else {
          Ok(v)
        }
      })?;

    let input_cover_img = find_cover_image_stream(&input);
    let audio_codec = unsafe { &*audio_stream.codecpar };
    let file_extension = match audio_codec.codec_id {
      AVCodecID_AV_CODEC_ID_MP3 | AVCodecID_AV_CODEC_ID_MP3ADU | AVCodecID_AV_CODEC_ID_MP3ON4 => {
        Some(EXT_MP3)
      }
      AVCodecID_AV_CODEC_ID_FLAC => Some(EXT_FLAC),
      _ => input_path
        .as_ref()
        .extension()
        .map(|v| v.to_str())
        .flatten(),
    }
    .ok_or(SplitError::UnknownAudioContainer)?;

    let mut decoder = AvCodecContext::new_decoder(audio_codec.codec_id);
    decoder.copy_params_from(audio_codec)?;
    decoder.frame_size = unsafe { *audio_stream.codecpar }.frame_size;
    decoder.pkt_timebase = audio_stream.time_base;
    decoder.open()?;

    let mut outputs = Vec::with_capacity(cuesheet.tracks.len());

    for track_info in cuesheet.tracks.iter() {
      let file_name = format!(
        "{no} {title}",
        no = track_info.track_no,
        title = track_info.title.unwrap_or(UNTITLED_TRACK),
      );

      let output_path = output_dir
        .as_ref()
        .join(file_name)
        .with_added_extension(file_extension);

      let mut output_context = AvOutputContext::open_path(output_path)?;
      let flags = output_context.flags;
      let output_audio = output_context.create_stream()?;

      let mut encoder = AvCodecContext::new_encoder(audio_codec.codec_id);
      encoder.copy_params_from(audio_codec)?;
      encoder.time_base.den = audio_codec.sample_rate;
      encoder.time_base.num = 1;
      encoder.frame_size = unsafe { *audio_stream.codecpar }.frame_size;

      if (flags & AVFMT_GLOBALHEADER as i32) == AVFMT_GLOBALHEADER as i32 {
        encoder.flags |= AV_CODEC_FLAG_GLOBAL_HEADER as i32;
      }

      encoder.open()?;
      encoder.copy_params_to(unsafe { output_audio.codecpar.as_mut() }.expect("dafuq"))?;

      if let Some(cover) = input_cover_img {
        let output_cover_img = output_context.create_stream()?;
        copy_stream_properties(cover, output_cover_img)?;
      }

      outputs.push(SplitOutput {
        start_time: track_info.time_info.start.as_duration().into(),
        end_time: track_info.time_info.end.map(|v| v.as_duration().into()),
        context: output_context,
        encoder,
        samples: AudioFifo::new(decoder.sample_fmt, decoder.ch_layout.nb_channels),
      });
    }

    Ok(Self {
      cover_image_packets: Vec::new(),
      audio_stream_index: audio_stream.index,
      cover_image_stream_index: input_cover_img.map(|v| v.index),
      audio_decoder: decoder,
      outputs,
      input,
    })
  }

  pub fn split(mut self) -> Result<(), SplitError> {
    let mut pkt = AvPacket::try_new()?;

    'DECODER: loop {
      pkt.reset();

      match self.input.read_frame(&mut pkt) {
        Ok(()) => {
          if pkt.stream_index == self.audio_stream_index {
            let frame_ts = AvTimestamp::new(pkt.pts, self.audio_decoder.pkt_timebase);

            for output in self.outputs.iter_mut() {
              if frame_ts >= output.start_time && output.end_time.is_none_or(|v| frame_ts <= v) {
                self.audio_decoder.send_packet(&pkt)?;
                let mut frame = AvFrame::try_new()?;

                match self.audio_decoder.receive_frame(&mut frame) {
                  Ok(()) => {
                    output.samples.push(frame.data.as_ptr(), frame.nb_samples)?;
                    break;
                  }
                  Err(AvError::IOError(err)) if err.kind() == ErrorKind::WouldBlock => {
                    continue 'DECODER;
                  }
                  Err(other) => return Err(other.into()),
                }
              }
            }
          } else if let Some(cover_stream_index) = self.cover_image_stream_index
            && pkt.stream_index == cover_stream_index
          {
            pkt.stream_index = cover_stream_index;
            self.cover_image_packets.push(pkt.clone());
          }
        }
        Err(AvError::AvLibError(AvLibError::Eof)) => {
          break 'DECODER;
        }
        Err(err) => {
          return Err(err.into());
        }
      }
    }

    for mut output in self.outputs.into_iter() {
      let mut pts: i64 = 0;
      let mut writer = output.context.start_writer()?;
      let mut packet = AvPacket::try_new()?;

      writer.write_header()?;

      for pkt in self.cover_image_packets.iter() {
        let mut owned_pkt = pkt.clone();
        writer.write_frame(&mut owned_pkt)?;
      }

      while !output.samples.is_empty() {
        let frame_size = output.samples.len().min(output.encoder.frame_size as usize) as i32;
        let mut frame = AvFrame::try_new()?;

        frame.nb_samples = frame_size;
        frame.copy_channel_layout(&output.encoder.ch_layout)?;
        frame.format = output.encoder.sample_fmt;
        frame.sample_rate = output.encoder.sample_rate;
        frame.alloc_buffer()?;

        let read = output.samples.pop(frame.data.as_ptr(), frame_size)?;

        frame.pts = pts;
        pts += read as i64;

        output.encoder.send_frame(&frame)?;

        packet.reset();
        match output.encoder.receive_packet(&mut packet) {
          Ok(()) => {
            writer.write_frame(&mut packet)?;
          }
          Err(AvError::AvLibError(AvLibError::Eof)) => break,
          Err(AvError::IOError(err)) if err.kind() == ErrorKind::WouldBlock => {
            continue;
          }
          Err(err) => return Err(err.into()),
        }
      }

      output.encoder.finish()?;

      loop {
        packet.reset();
        match output.encoder.receive_packet(&mut packet) {
          Ok(()) => {
            writer.write_frame(&mut packet)?;
          }
          Err(AvError::AvLibError(AvLibError::Eof)) => break,
          Err(AvError::IOError(err)) if err.kind() == ErrorKind::WouldBlock => {
            continue;
          }
          Err(err) => return Err(err.into()),
        }
      }

      writer.finish()?;
    }

    Ok(())
  }
}

fn find_cover_image_stream(input: &AvInputContext) -> Option<&AVStream> {
  if let Some(stream) = input.find_best_stream(StreamType::Video) {
    if let Some(metadata) =
      unsafe { stream.metadata.as_ref() }.map(|v| AvDictionaryRef::from_ref(v))
    {
      if let Some(value) = metadata.get(COVER_IMAGE_KEY) {
        if value == COVER_IMAGE_VALUE {
          return Some(stream);
        }
      }
    }
  }

  None
}
