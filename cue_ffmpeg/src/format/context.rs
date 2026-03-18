use crate::{
  common::{ts_as_option, unsafe_av_result},
  error::{AvError, AvLibError},
  format::stream::{StreamIter, StreamMutIter, StreamType, copy_stream_properties},
  util::{
    dictionary::{AvDictionaryMut, AvDictionaryRef},
    timestamp::AvTimestamp,
  },
};
use cue_ffmpeg_sys::{
  AVFormatContext, AVSEEK_FLAG_BACKWARD, AVStream, av_dict_copy, av_find_best_stream,
  av_seek_frame, avformat_new_stream,
};
use std::{
  ops::{Deref, DerefMut},
  ptr::{null, null_mut},
  time::Duration,
};

mod input;
mod output;

pub use input::*;
pub use output::*;

pub struct AvContext {
  inner: *mut AVFormatContext,
}

impl AvContext {
  pub const fn duration(&self) -> Option<Duration> {
    match ts_as_option!(unsafe { (*self.inner).duration }) {
      Some(v) => Some(Duration::from_micros(v as u64)),
      None => None,
    }
  }

  pub fn seek_forward(&mut self, stream_index: u32, timestamp: Duration) -> Result<(), AvError> {
    self.internal_seek(stream_index, timestamp, 0)
  }

  pub fn seek_backward(&mut self, stream_index: u32, timestamp: Duration) -> Result<(), AvError> {
    self.internal_seek(stream_index, timestamp, AVSEEK_FLAG_BACKWARD as i32)
  }

  fn internal_seek(
    &mut self,
    stream_index: u32,
    timestamp: Duration,
    flag: i32,
  ) -> Result<(), AvError> {
    if stream_index < self.nb_streams {
      let av_timestamp = AvTimestamp::from(timestamp);

      unsafe_av_result!(av_seek_frame(
        self.inner,
        stream_index as i32,
        av_timestamp.as_i64(),
        flag
      ))
    } else {
      Err(AvLibError::StreamNotFound.into())
    }
  }

  pub fn get_stream(&self, idx: usize) -> Option<&AVStream> {
    if idx >= self.nb_streams as usize {
      None
    } else {
      let stream = unsafe { self.streams.add(idx).read() };
      unsafe { stream.as_ref() }
    }
  }

  pub fn get_mut_stream(&self, idx: usize) -> Option<&mut AVStream> {
    if idx >= self.nb_streams as usize {
      None
    } else {
      let stream = unsafe { self.streams.add(idx).read() };
      unsafe { stream.as_mut() }
    }
  }

  pub fn find_best_stream(&self, stream_type: StreamType) -> Option<&AVStream> {
    let index =
      unsafe { av_find_best_stream(self.inner, stream_type.as_i32(), -1, -1, null_mut(), 0) };

    if index >= 0 {
      self.get_stream(index as usize)
    } else {
      None
    }
  }

  pub fn create_stream(&mut self) -> Result<&mut AVStream, AvError> {
    let stream = unsafe { avformat_new_stream(self.inner, null()) };

    if let Some(value) = unsafe { stream.as_mut() } {
      Ok(value)
    } else {
      Err(AvError::AllocationError)
    }
  }

  pub fn copy_streams(&self, dst: &mut Self) -> Result<(), AvError> {
    for in_stream in self.stream_iter() {
      let out_stream = dst.create_stream()?;
      copy_stream_properties(in_stream, out_stream)?;
    }

    Ok(())
  }

  pub fn stream_iter(&self) -> StreamIter<'_> {
    StreamIter::from_context(&self)
  }

  pub fn stream_mut_iter(&mut self) -> StreamMutIter<'_> {
    StreamMutIter::from_context(self)
  }

  pub const fn metadata(&self) -> Option<AvDictionaryRef<'_>> {
    match unsafe { (*self.inner).metadata.as_ref() } {
      Some(v) => Some(AvDictionaryRef::from_ref(v)),
      None => None,
    }
  }

  pub fn metadata_mut(&mut self) -> Option<AvDictionaryMut<'_>> {
    match unsafe { (*self.inner).metadata.as_mut() } {
      Some(v) => Some(AvDictionaryMut::from_mut(v)),
      None => None,
    }
  }

  pub fn copy_metadata_to(&self, dest: &mut AvContext) -> Result<(), AvError> {
    if self.metadata.is_null() {
      Err(AvError::UninitializedDictionary)
    } else {
      unsafe_av_result!(av_dict_copy(&mut dest.metadata, self.metadata, 0))
    }
  }

  #[inline]
  pub const fn as_ptr(&self) -> *const AVFormatContext {
    self.inner
  }
}

impl Deref for AvContext {
  type Target = AVFormatContext;

  #[inline]
  fn deref(&self) -> &Self::Target {
    unsafe { &*self.inner }
  }
}

impl DerefMut for AvContext {
  #[inline]
  fn deref_mut(&mut self) -> &mut Self::Target {
    unsafe { &mut *self.inner }
  }
}

impl AsRef<AVFormatContext> for AvContext {
  #[inline]
  fn as_ref(&self) -> &AVFormatContext {
    unsafe { &*self.inner }
  }
}

impl AsMut<AVFormatContext> for AvContext {
  #[inline]
  fn as_mut(&mut self) -> &mut AVFormatContext {
    unsafe { &mut *self.inner }
  }
}
