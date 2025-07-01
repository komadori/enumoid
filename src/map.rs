use crate::base::EnumArrayHelper;
use crate::base::EnumSetHelper;
use crate::iter::EnumSliceIter;
use crate::iter::EnumSliceIterMut;
use crate::opt_map::EnumOptionMap;
use crate::sub_base::RawSizeWord;
use crate::EnumIndex;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;
use std::mem;
use std::ops::{Index, IndexMut};

/// A total map from enumoid `T` to values `V`.
pub struct EnumMap<T: EnumArrayHelper<V>, V> {
  data: T::TotalArray,
}

impl<T: EnumArrayHelper<V>, V: Default> EnumMap<T, V> {
  /// Creates a new map populated with default values.
  pub fn new() -> Self {
    Self::new_with(|_| Default::default())
  }
}

impl<T: EnumArrayHelper<V>, V> EnumMap<T, V> {
  /// Creates a new map populated by a callback function.
  pub fn new_with<F>(mut f: F) -> Self
  where
    F: FnMut(T) -> V,
  {
    let mut arr = T::new_partial();
    for (key, cell) in T::iter().zip(T::partial_slice_mut(&mut arr).iter_mut())
    {
      cell.write(f(key));
    }
    EnumMap {
      data: unsafe { T::partial_to_total(arr) },
    }
  }

  /// Returns a slice containing all the values in the map.
  #[inline]
  pub fn as_slice(&self) -> &[V] {
    T::total_slice(&self.data)
  }

  /// Returns a mutable slice containing all the values in the map.
  #[inline]
  pub fn as_slice_mut(&mut self) -> &mut [V] {
    T::total_slice_mut(&mut self.data)
  }

  /// Returns a reference to the value associated with a given index.
  #[inline]
  pub fn get_by_index(&self, index: EnumIndex<T>) -> &V {
    &self[index]
  }

  /// Returns a reference to the value associated with a given key.
  #[inline]
  pub fn get(&self, key: T) -> &V {
    &self[key]
  }

  /// Returns a mutable reference to the value associated with a given index.
  #[inline]
  pub fn get_by_index_mut(&mut self, index: EnumIndex<T>) -> &mut V {
    &mut self[index]
  }

  /// Returns a mutable reference to the value associated with a given key.
  #[inline]
  pub fn get_mut(&mut self, key: T) -> &mut V {
    &mut self[key]
  }

  /// Sets the value associated with a given index and returns the old value.
  #[inline]
  pub fn set_by_index(&mut self, index: EnumIndex<T>, value: V) -> V {
    mem::replace(self.get_by_index_mut(index), value)
  }

  /// Sets the value associated with a given key and returns the old value.
  #[inline]
  pub fn set(&mut self, key: T, value: V) -> V {
    self.set_by_index(key.into(), value)
  }

  /// Swaps two elements in the map.
  #[inline]
  pub fn swap(&mut self, a: T, b: T) {
    self
      .as_slice_mut()
      .swap(T::into_word(a).as_(), T::into_word(b).as_())
  }

  /// Returns an iterator over the keys and elements.
  #[inline]
  pub fn iter(&self) -> EnumSliceIter<T, V> {
    EnumSliceIter {
      _phantom: Default::default(),
      word: T::Word::ZERO,
      iter: self.as_slice().iter(),
    }
  }

  /// Returns a mutable iterator over the keys and elements.
  #[inline]
  pub fn iter_mut(&mut self) -> EnumSliceIterMut<T, V> {
    EnumSliceIterMut {
      _phantom: Default::default(),
      word: T::Word::ZERO,
      iter: self.as_slice_mut().iter_mut(),
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

impl<T: EnumArrayHelper<V>, V: PartialEq> PartialEq for EnumMap<T, V> {
  fn eq(&self, other: &Self) -> bool {
    self.as_slice() == other.as_slice()
  }
}

impl<T: EnumArrayHelper<V>, V: Eq> Eq for EnumMap<T, V> {}

impl<T: EnumArrayHelper<V>, V: Hash> Hash for EnumMap<T, V> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.as_slice().hash(state);
  }
}

impl<T: EnumArrayHelper<V>, V> Index<EnumIndex<T>> for EnumMap<T, V> {
  type Output = V;

  #[inline]
  fn index(&self, index: EnumIndex<T>) -> &V {
    unsafe { self.as_slice().get_unchecked(index.into_usize()) }
  }
}

impl<T: EnumArrayHelper<V>, V> Index<T> for EnumMap<T, V> {
  type Output = V;

  #[inline]
  fn index(&self, key: T) -> &V {
    &self[EnumIndex::from_value(key)]
  }
}

impl<T: EnumArrayHelper<V>, V> IndexMut<EnumIndex<T>> for EnumMap<T, V> {
  #[inline]
  fn index_mut(&mut self, index: EnumIndex<T>) -> &mut V {
    unsafe { self.as_slice_mut().get_unchecked_mut(index.into_usize()) }
  }
}

impl<T: EnumArrayHelper<V>, V> IndexMut<T> for EnumMap<T, V> {
  #[inline]
  fn index_mut(&mut self, key: T) -> &mut V {
    &mut self[EnumIndex::from_value(key)]
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

impl<T: EnumArrayHelper<V> + EnumSetHelper<u8>, V> TryFrom<EnumOptionMap<T, V>>
  for EnumMap<T, V>
{
  type Error = ();
  fn try_from(from: EnumOptionMap<T, V>) -> Result<Self, Self::Error> {
    if from.is_full() {
      let data = from.into_partial();
      Ok(EnumMap {
        data: unsafe { T::partial_to_total(data) },
      })
    } else {
      Err(())
    }
  }
}
