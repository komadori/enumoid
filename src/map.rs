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

/// A total map from enumoid `T` to values `V`.
pub struct EnumMap<T: EnumArrayHelper<V>, V> {
  data: T::TotalArray,
}

impl<T: EnumArrayHelper<V>, V: Default> EnumMap<T, V> {
  pub fn new() -> Self {
    Self::new_with(|_| Default::default())
  }
}

impl<T: EnumArrayHelper<V>, V> EnumMap<T, V> {
  pub fn new_with<F>(mut f: F) -> Self
  where
    F: FnMut(T) -> V,
  {
    let mut arr = T::new_partial();
    for (i, cell) in T::partial_slice_mut(&mut arr).iter_mut().enumerate() {
      *cell = mem::MaybeUninit::new(f(T::from_usize(i)));
    }
    EnumMap {
      data: unsafe { T::partial_to_total(arr) },
    }
  }

  pub fn as_slice(&self) -> &[V] {
    T::total_slice(&self.data)
  }

  pub fn as_slice_mut(&mut self) -> &mut [V] {
    T::total_slice_mut(&mut self.data)
  }

  pub fn get(&self, key: T) -> &V {
    &self.as_slice()[T::into_usize(key)]
  }

  pub fn swap(&mut self, a: T, b: T) {
    self.as_slice_mut().swap(T::into_usize(a), T::into_usize(b))
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

impl<T: EnumArrayHelper<V>, V: Clone> Clone for EnumMap<T, V> {
  fn clone(&self) -> Self {
    Self::new_with(|k| self.get(k).clone())
  }
}

impl<T: EnumArrayHelper<V>, V: Copy> Copy for EnumMap<T, V> where
  T::TotalArray: Copy
{
}

impl<T: EnumArrayHelper<V> + Debug, V: Debug> Debug for EnumMap<T, V> {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.debug_map().entries(self.iter()).finish()
  }
}

impl<T: EnumArrayHelper<V>, V: Default> Default for EnumMap<T, V> {
  fn default() -> Self {
    Self::new()
  }
}

impl<T: EnumArrayHelper<V>, V> Index<T> for EnumMap<T, V> {
  type Output = V;

  fn index(&self, i: T) -> &V {
    &self.as_slice()[T::into_usize(i)]
  }
}

impl<T: EnumArrayHelper<V>, V> IndexMut<T> for EnumMap<T, V> {
  fn index_mut(&mut self, i: T) -> &mut V {
    &mut self.as_slice_mut()[T::into_usize(i)]
  }
}

impl<'a, T: EnumArrayHelper<V>, V> iter::IntoIterator for &'a EnumMap<T, V> {
  type Item = (T, &'a V);
  type IntoIter = EnumSliceIter<'a, T, V>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

impl<'a, T: EnumArrayHelper<V>, V> iter::IntoIterator
  for &'a mut EnumMap<T, V>
{
  type Item = (T, &'a mut V);
  type IntoIter = EnumSliceIterMut<'a, T, V>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
  }
}

impl<'a, T: EnumFlagsHelper + EnumArrayHelper<V>, V>
  TryFrom<EnumOptionMap<T, V>> for EnumMap<T, V>
{
  type Error = ();
  fn try_from(from: EnumOptionMap<T, V>) -> Result<Self, Self::Error> {
    if from.is_full() {
      Ok(EnumMap {
        data: unsafe { T::partial_to_total(from.data) },
      })
    } else {
      Err(())
    }
  }
}
