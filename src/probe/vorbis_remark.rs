use super::remark::RemarkIter;
use crate::{
  core::CueStr,
  metadata::vorbis::{VorbisComment, VorbisTag},
};

pub struct VorbisRemarkIter<'a> {
  inner: RemarkIter<'a>,
}

impl<'a> Iterator for VorbisRemarkIter<'a> {
  type Item = (VorbisTag, CueStr<'a>);

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(remark) = self.inner.next() {
      match VorbisComment::try_from_line(remark) {
        Ok(VorbisComment { tag, value }) => return Some((tag, value)),
        Err(_) => continue,
      }
    }

    None
  }
}

impl<'a> From<RemarkIter<'a>> for VorbisRemarkIter<'a> {
  #[inline]
  fn from(value: RemarkIter<'a>) -> Self {
    Self { inner: value }
  }
}
