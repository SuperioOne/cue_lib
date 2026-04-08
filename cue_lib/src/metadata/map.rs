use super::MetadataTag;
use crate::core::cue_str::CueStr;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use core::borrow::Borrow;

#[derive(Debug)]
pub struct MetadataMap<'a, T>
where
  T: Borrow<MetadataTag> + Ord + Clone,
{
  inner: BTreeMap<T, Vec<CueStr<'a>>>,
}

pub struct Iter<'a, T>
where
  T: Borrow<MetadataTag>,
{
  inner: alloc::collections::btree_map::Iter<'a, T, Vec<CueStr<'a>>>,
}

pub struct IterMut<'a, T>
where
  T: Borrow<MetadataTag>,
{
  inner: alloc::collections::btree_map::IterMut<'a, T, Vec<CueStr<'a>>>,
}

impl<'a, T> MetadataMap<'a, T>
where
  T: Borrow<MetadataTag> + Ord + Clone,
{
  #[inline]
  pub const fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
    }
  }

  pub fn from_iter<I>(metadata_iter: I) -> Self
  where
    I: Iterator<Item = (T, CueStr<'a>)> + 'a,
  {
    let mut map = Self::new();

    for metadata in metadata_iter {
      _ = map.insert(metadata.0, metadata.1)
    }

    map
  }

  pub fn get<K>(&self, key: K) -> Option<&[CueStr<'a>]>
  where
    K: Borrow<MetadataTag>,
  {
    self.inner.get(key.borrow()).map(|v| v.as_slice())
  }

  pub fn get_mut<K>(&mut self, key: K) -> Option<&mut Vec<CueStr<'a>>>
  where
    K: Borrow<MetadataTag>,
  {
    self.inner.get_mut(key.borrow())
  }

  pub fn override_key<K>(&mut self, key: K, value: Vec<CueStr<'a>>) -> Option<Vec<CueStr<'a>>>
  where
    K: Borrow<T>,
  {
    self.inner.insert(key.borrow().clone(), value)
  }

  pub fn insert<K>(&mut self, key: K, value: CueStr<'a>) -> bool
  where
    K: Borrow<T>,
  {
    let values = self
      .inner
      .entry(key.borrow().clone())
      .or_insert(Vec::with_capacity(1));

    for existing in values.iter() {
      if *existing == value {
        return false;
      }
    }

    values.push(value);
    true
  }

  pub fn remove<K>(&mut self, key: K) -> Option<Vec<CueStr<'a>>>
  where
    K: Borrow<MetadataTag>,
  {
    self.inner.remove(key.borrow())
  }

  pub fn contains_key<K>(&self, key: K) -> bool
  where
    K: Borrow<MetadataTag>,
  {
    self.inner.contains_key(key.borrow())
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
  pub fn iter(&'a self) -> Iter<'a, T> {
    Iter {
      inner: self.inner.iter(),
    }
  }

  #[inline]
  pub fn iter_mut(&'a mut self) -> IterMut<'a, T> {
    IterMut {
      inner: self.inner.iter_mut(),
    }
  }

  #[inline]
  pub fn clear(&mut self) {
    self.inner.clear();
  }
}

impl<'a, T> Iterator for Iter<'a, T>
where
  T: Borrow<MetadataTag>,
{
  type Item = (&'a T, &'a Vec<CueStr<'a>>);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}

impl<'a, T> Iterator for IterMut<'a, T>
where
  T: Borrow<MetadataTag>,
{
  type Item = (&'a T, &'a mut Vec<CueStr<'a>>);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.inner.next()
  }
}
