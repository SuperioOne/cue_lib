use super::MetadataTag;
use crate::core::CueStr;
use alloc::{collections::btree_map::BTreeMap, vec::Vec};
use core::borrow::Borrow;

/// A map that associates metadata tags with multiple unique values
///
/// This structure is designed for storing ID3 and Vorbis comment metadata,
/// where each tag can have multiple associated values. It uses a [`BTreeMap`]
/// internally and ensures that duplicate values (case-sensitive) are automatically
/// deduplicated when inserting new entries.
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

impl<'a, T> MetadataMap<'a, T>
where
  T: Borrow<MetadataTag> + Ord + Clone,
{
  /// Creates a new, empty [`MetadataMap`]
  #[inline]
  pub const fn new() -> Self {
    Self {
      inner: BTreeMap::new(),
    }
  }

  /// Returns a reference to values associated with the given key
  ///
  /// If the key is not present, returns [None]
  pub fn get<K>(&self, key: K) -> Option<&[CueStr<'a>]>
  where
    K: Borrow<MetadataTag>,
  {
    self.inner.get(key.borrow()).map(|v| v.as_slice())
  }

  /// Overrides values for the given key, replacing any existing ones
  ///
  /// Returns the previous vector of values if the key existed, or [None] if it did not
  pub fn override_key<K>(&mut self, key: K, value: Vec<CueStr<'a>>) -> Option<Vec<CueStr<'a>>>
  where
    K: Borrow<T>,
  {
    self.inner.insert(key.borrow().clone(), value)
  }

  /// Inserts a value into the given key
  ///
  /// Returns `true` if the value was inserted, `false` if it already existed
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

  /// Removes metadata tag
  pub fn remove<K>(&mut self, key: K) -> Option<Vec<CueStr<'a>>>
  where
    K: Borrow<MetadataTag>,
  {
    self.inner.remove(key.borrow())
  }

  /// Checks whether the given key is present in the map
  pub fn contains_key<K>(&self, key: K) -> bool
  where
    K: Borrow<MetadataTag>,
  {
    self.inner.contains_key(key.borrow())
  }

  /// Returns the number of key-value pairs in the map
  #[inline]
  pub fn len(&self) -> usize {
    self.inner.len()
  }

  /// Returns `true` if the map is empty
  #[inline]
  pub fn is_empty(&self) -> bool {
    self.inner.is_empty()
  }

  /// Returns an iterator over the entries of the map
  #[inline]
  pub fn iter(&'a self) -> Iter<'a, T> {
    Iter {
      inner: self.inner.iter(),
    }
  }

  /// Clears the map, removing all entries
  #[inline]
  pub fn clear(&mut self) {
    self.inner.clear();
  }
}

impl<'a, T> FromIterator<(T, CueStr<'a>)> for MetadataMap<'a, T>
where
  T: Borrow<MetadataTag> + Ord + Clone + 'a,
{
  fn from_iter<I: IntoIterator<Item = (T, CueStr<'a>)>>(iter: I) -> Self {
    let mut map = Self::new();

    for metadata in iter {
      _ = map.insert(metadata.0, metadata.1)
    }

    map
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
