use crate::base::EnumArrayHelper;
use crate::base::EnumFlagsHelper;
use crate::iter::EnumSliceIter;
use crate::iter::EnumSliceIterMut;
use crate::opt_map::EnumOptionMap;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::iter;
use std::mem;
use std::ops::{Index, IndexMut};
use std::ptr;

/// A vector of values `V` indexed by enumoid `T`.
pub struct EnumVec<T: EnumArrayHelper<V>, V> {
  pub(crate) len: T::CompactSize,
  pub(crate) data: T::PartialArray,
}

impl<T: EnumArrayHelper<V>, V> EnumVec<T, V> {
  pub fn new() -> Self {
    EnumVec {
      len: T::compact_size(0),
      data: T::new_partial(),
    }
  }

  pub fn new_with<F>(last_key: Option<T>, mut f: F) -> Self
  where
    F: FnMut(T) -> V,
  {
    let len = match last_key {
      Some(key) => T::into_usize(key) + 1,
      None => 0,
    };
    let mut vec = Self::new();
    for i in 0..len {
      vec.push(f(T::from_usize(i)))
    }
    vec
  }

  pub fn as_slice(&self) -> &[V] {
    let inited = &T::partial_slice(&self.data)[0..T::uncompact_size(self.len)];
    unsafe { &*(inited as *const [std::mem::MaybeUninit<V>] as *const [V]) }
  }

  pub fn as_slice_mut(&mut self) -> &mut [V] {
    let inited =
      &mut T::partial_slice_mut(&mut self.data)[0..T::uncompact_size(self.len)];
    unsafe { &mut *(inited as *mut [std::mem::MaybeUninit<V>] as *mut [V]) }
  }

  pub fn get(&self, key: T) -> Option<&V> {
    let i = T::into_usize(key);
    if i < T::uncompact_size(self.len) {
      Some(&self.as_slice()[i])
    } else {
      None
    }
  }

  pub fn len(&self) -> usize {
    T::uncompact_size(self.len)
  }

  pub fn is_empty(&self) -> bool {
    T::uncompact_size(self.len) == 0
  }

  pub fn last_key(&self) -> Option<T> {
    let len = T::uncompact_size(self.len);
    if len == 0 {
      None
    } else {
      Some(T::from_usize(len - 1))
    }
  }

  pub fn swap(&mut self, a: T, b: T) {
    self.as_slice_mut().swap(T::into_usize(a), T::into_usize(b))
  }

  pub fn clear(&mut self) {
    for x in self.iter_mut() {
      mem::drop(x)
    }
    self.len = T::compact_size(0);
  }

  pub fn push(&mut self, value: V) {
    let len = T::uncompact_size(self.len);
    T::partial_slice_mut(&mut self.data)[len] =
      mem::MaybeUninit::<V>::new(value);
    self.len = T::compact_size(len + 1);
  }

  #[inline]
  pub fn iter(&self) -> EnumSliceIter<T, V> {
    EnumSliceIter {
      _phantom: Default::default(),
      iter: self.as_slice().iter().enumerate(),
    }
  }

  #[inline]
  pub fn iter_mut(&mut self) -> EnumSliceIterMut<T, V> {
    EnumSliceIterMut {
      _phantom: Default::default(),
      iter: self.as_slice_mut().iter_mut().enumerate(),
    }
  }
}

impl<T: EnumArrayHelper<V> + Debug, V: Debug> Debug for EnumVec<T, V> {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.debug_map().entries(self.iter()).finish()
  }
}

impl<T: EnumArrayHelper<V>, V> Default for EnumVec<T, V> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: EnumArrayHelper<V>, V> Drop for EnumVec<T, V> {
  fn drop(&mut self) {
    for cell in T::partial_slice_mut(&mut self.data)
      [0..T::uncompact_size(self.len)]
      .iter_mut()
    {
      unsafe { ptr::drop_in_place(cell.as_mut_ptr()) };
    }
  }
}

impl<T: EnumArrayHelper<V>, V: Clone> Clone for EnumVec<T, V> {
  fn clone(&self) -> Self {
    let mut clone = Self::new();
    for (_, value) in self.iter() {
      clone.push(value.clone())
    }
    clone
  }
}

impl<T: EnumArrayHelper<V>, V> Index<T> for EnumVec<T, V> {
  type Output = V;

  fn index(&self, i: T) -> &V {
    &self.as_slice()[T::into_usize(i)]
  }
}

impl<T: EnumArrayHelper<V>, V> IndexMut<T> for EnumVec<T, V> {
  fn index_mut(&mut self, i: T) -> &mut V {
    &mut self.as_slice_mut()[T::into_usize(i)]
  }
}

impl<T: EnumArrayHelper<V>, V> iter::FromIterator<V> for EnumVec<T, V> {
  fn from_iter<I: iter::IntoIterator<Item = V>>(iter: I) -> Self {
    let mut c = EnumVec::<T, V>::new();
    for i in iter {
      c.push(i);
    }
    c
  }
}

impl<'a, T: EnumArrayHelper<V>, V> iter::IntoIterator for &'a EnumVec<T, V> {
  type Item = (T, &'a V);
  type IntoIter = EnumSliceIter<'a, T, V>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T: EnumFlagsHelper + EnumArrayHelper<V>, V>
  TryFrom<EnumOptionMap<T, V>> for EnumVec<T, V>
{
  type Error = ();
  fn try_from(from: EnumOptionMap<T, V>) -> Result<Self, Self::Error> {
    match from.is_vec() {
      Some(size) => Ok(EnumVec {
        len: T::compact_size(size),
        data: from.data,
      }),
      None => Err(()),
    }
  }
}
