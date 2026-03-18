use cue_ffmpeg_sys::{
  AV_DICT_IGNORE_SUFFIX, AV_DICT_MULTIKEY, AVDictionary, AVDictionaryEntry, av_dict_count,
  av_dict_get, av_dict_iterate, av_dict_set,
};
use std::{
  borrow::Borrow,
  ffi::{CStr, CString},
  ptr::{self, null},
  str::FromStr,
};

use crate::{common::unsafe_av_result, error::AvError};

pub struct AvDictionaryRef<'a> {
  inner: &'a AVDictionary,
}

pub struct AvDictionaryMut<'a> {
  inner: &'a mut AVDictionary,
}

impl<'a> AvDictionaryRef<'a> {
  #[inline]
  pub const fn from_ref(dictionary: &'a AVDictionary) -> Self {
    Self { inner: dictionary }
  }

  pub fn len(&self) -> usize {
    unsafe { av_dict_count(self.inner) as usize }
  }

  pub fn get<K>(&self, key: K) -> Option<&'a CStr>
  where
    K: Borrow<CStr>,
  {
    let mut tag: *const AVDictionaryEntry = null();

    tag = unsafe {
      av_dict_get(
        self.inner,
        key.borrow().as_ptr(),
        tag,
        AV_DICT_IGNORE_SUFFIX as i32,
      )
    };

    unsafe {
      if tag.is_null() || (*tag).value.is_null() {
        None
      } else {
        Some(CStr::from_ptr((*tag).value))
      }
    }
  }

  pub fn has<K>(&self, key: K) -> bool
  where
    K: Borrow<CStr>,
  {
    let mut tag: *const AVDictionaryEntry = null();

    tag = unsafe {
      av_dict_get(
        self.inner,
        key.borrow().as_ptr(),
        tag,
        AV_DICT_IGNORE_SUFFIX as i32,
      )
    };

    unsafe {
      if tag.is_null() || (*tag).value.is_null() {
        false
      } else {
        true
      }
    }
  }

  pub fn iter(&'a self) -> Iter<'a> {
    Iter {
      dictionary: self.inner,
      current: null(),
    }
  }
}

impl<'a> AvDictionaryMut<'a> {
  #[inline]
  pub(crate) const fn from_mut(dictionary: &'a mut AVDictionary) -> Self {
    Self { inner: dictionary }
  }

  pub fn len(&self) -> usize {
    unsafe { av_dict_count(self.inner) as usize }
  }

  pub fn override_entry<K, V>(&mut self, key: K, value: V) -> Result<(), AvError>
  where
    K: Borrow<str>,
    V: Borrow<str>,
  {
    self.internal_set(key, value, 0)
  }

  pub fn set<K, V>(&mut self, key: K, value: V) -> Result<(), AvError>
  where
    K: Borrow<str>,
    V: Borrow<str>,
  {
    self.internal_set(key, value, (AV_DICT_MULTIKEY) as i32)
  }

  fn internal_set<K, V>(&mut self, key: K, value: V, flag: i32) -> Result<(), AvError>
  where
    K: Borrow<str>,
    V: Borrow<str>,
  {
    let key = CString::from_str(key.borrow()).expect("unexpected null terminator on rust str");
    let value = CString::from_str(value.borrow()).expect("unexpected null terminator on rust str");

    unsafe_av_result!(av_dict_set(
      &mut ptr::from_mut(self.inner),
      key.as_ptr(),
      value.as_ptr(),
      flag
    ))
  }

  pub fn get<K>(&self, key: K) -> Option<&'a CStr>
  where
    K: Borrow<CStr>,
  {
    let mut tag: *const AVDictionaryEntry = null();

    tag = unsafe {
      av_dict_get(
        self.inner,
        key.borrow().as_ptr(),
        tag,
        AV_DICT_IGNORE_SUFFIX as i32,
      )
    };

    unsafe {
      if tag.is_null() || (*tag).value.is_null() {
        None
      } else {
        Some(CStr::from_ptr((*tag).value))
      }
    }
  }

  pub fn has<K>(&self, key: K) -> bool
  where
    K: Borrow<CStr>,
  {
    let mut tag: *const AVDictionaryEntry = null();

    tag = unsafe {
      av_dict_get(
        self.inner,
        key.borrow().as_ptr(),
        tag,
        AV_DICT_IGNORE_SUFFIX as i32,
      )
    };

    unsafe {
      if tag.is_null() || (*tag).value.is_null() {
        false
      } else {
        true
      }
    }
  }

  pub fn iter(&'a self) -> Iter<'a> {
    Iter {
      dictionary: self.inner,
      current: null(),
    }
  }
}

pub struct Iter<'a> {
  dictionary: &'a AVDictionary,
  current: *const AVDictionaryEntry,
}

impl<'a> Iterator for Iter<'a> {
  type Item = (&'a CStr, &'a CStr);

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      self.current = unsafe { av_dict_iterate(self.dictionary, self.current) };
      let tag = self.current;

      unsafe {
        if tag.is_null() {
          return None;
        } else {
          if (*tag).key.is_null() || (*tag).value.is_null() {
            // Kinda impossible case, skips current entry if tag's key is null
            continue;
          } else {
            let key = CStr::from_ptr((*tag).key);
            let value = CStr::from_ptr((*tag).value);

            return Some((key, value));
          }
        }
      }
    }
  }
}
