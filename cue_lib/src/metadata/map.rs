use super::{VorbisComment, VorbisTagName};
use crate::core::cue_str::CueStr;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};

#[derive(Debug)]
pub struct MetadataMap<'a> {
  inner: BTreeMap<VorbisTagName, Vec<CueStr<'a>>>,
}

pub struct Iter<'a> {
  inner: alloc::collections::btree_map::Iter<'a, VorbisTagName, Vec<CueStr<'a>>>,
}

pub struct IterMut<'a> {
  inner: alloc::collections::btree_map::IterMut<'a, VorbisTagName, Vec<CueStr<'a>>>,
}

impl<'a> MetadataMap<'a> {
  #[inline]
  pub const fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
    }
  }

  pub fn from_iter<I>(metadata_iter: I) -> Self
  where
    I: Iterator<Item = VorbisComment<'a>>,
  {
    let mut metadata_map = BTreeMap::new();

    for metadata in metadata_iter {
      let list: &mut Vec<CueStr<'a>> = metadata_map.entry(metadata.tag).or_default();
      list.push(metadata.value);
    }

    Self {
      inner: metadata_map,
    }
  }

  #[inline]
  pub fn get(&self, key: &VorbisTagName) -> Option<&Vec<CueStr<'a>>> {
    self.inner.get(key)
  }

  #[inline]
  pub fn get_mut(&mut self, key: &VorbisTagName) -> Option<&mut Vec<CueStr<'a>>> {
    self.inner.get_mut(key)
  }

  #[inline]
  pub fn insert(&mut self, key: VorbisTagName, value: Vec<CueStr<'a>>) -> Option<Vec<CueStr<'a>>> {
    self.inner.insert(key, value)
  }

  #[inline]
  pub fn remove(&mut self, key: &VorbisTagName) -> Option<Vec<CueStr<'a>>> {
    self.inner.remove(key)
  }

  #[inline]
  pub fn contains_key(&self, key: &VorbisTagName) -> bool {
    self.inner.contains_key(key)
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.inner.len()
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  #[inline]
  pub fn iter(&'a self) -> Iter<'a> {
    Iter {
      inner: self.inner.iter(),
    }
  }

  #[inline]
  pub fn iter_mut(&'a mut self) -> IterMut<'a> {
    IterMut {
      inner: self.inner.iter_mut(),
    }
  }

  #[inline]
  pub fn clear(&mut self) {
    self.inner.clear();
  }
}

impl<'a> Iterator for Iter<'a> {
  type Item = (&'a VorbisTagName, &'a Vec<CueStr<'a>>);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}

impl<'a> Iterator for IterMut<'a> {
  type Item = (&'a VorbisTagName, &'a mut Vec<CueStr<'a>>);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}
