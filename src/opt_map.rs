use crate::base::EnumArrayHelper;
use crate::base::Size;
use crate::raw::RawIndex;
use crate::set::EnumSet;
use std::hash::Hash;
use std::mem;

/// A partial map from enumoid `T` to values `V`.
pub struct EnumOptionMap<T: EnumArrayHelper<V>, V> {
  valid: EnumSet<T>,
  pub(crate) data: T::PartialArray,
}

impl<T: EnumArrayHelper<V>, V> EnumOptionMap<T, V> {
  /// Creates a new empty map.
  pub fn new() -> Self {
    EnumOptionMap {
      valid: EnumSet::<T>::new(),
      data: T::new_partial(),
    }
  }

  #[inline]
  pub(crate) fn get_internal(&self, i: T::Word) -> Option<&V> {
    if self.valid.get_internal(i) {
      Some(unsafe { T::partial_slice(&self.data)[i.as_()].assume_init_ref() })
    } else {
      None
    }
  }

  /// Returns a reference to the value associated with a given key,
  /// or `None` if the key has no value in the map.
  pub fn get(&self, key: T) -> Option<&V> {
    self.get_internal(T::into_word(key))
  }

  /// Returns a mutable reference to the value associated with a given key,
  /// or `None` if the key has no value in the map.
  pub fn get_mut(&mut self, key: T) -> Option<&mut V> {
    let i = T::into_word(key);
    if self.valid.get_internal(i) {
      Some(unsafe {
        T::partial_slice_mut(&mut self.data)[i.as_()].assume_init_mut()
      })
    } else {
      None
    }
  }

  /// Sets the value associated with a given key.
  pub fn set(&mut self, key: T, value: Option<V>) {
    let i = T::into_word(key);
    let cell = &mut T::partial_slice_mut(&mut self.data)[i.as_()];
    if self.valid.get_internal(i) {
      unsafe { cell.assume_init_drop() };
    }
    self.valid.set_internal(i, value.is_some());
    if let Some(v) = value {
      cell.write(v);
    }
  }

  /// Clears all the elements from the map.
  pub fn clear(&mut self) {
    let data = T::partial_slice_mut(&mut self.data);
    for key in T::iter() {
      let word = key.into_word();
      if self.valid.get_internal(word) {
        let cell = &mut data[word.as_()];
        unsafe { cell.assume_init_drop() };
      }
    }
    self.valid.clear();
  }

  /// Returns true if the map is empty.
  pub fn is_empty(&self) -> bool {
    !self.valid.any()
  }

  /// Returns true if the map is fully populated.
  pub fn is_full(&self) -> bool {
    self.valid.all()
  }

  /// Returns the size of a vector needed to represent the map,
  /// or `None` if the map is not representable by a vector.
  ///
  /// A map is representable by vector if all the populated values
  /// are contiguous with the first key, or if the map is empty.
  pub fn is_vec(&self) -> Option<Size<T>> {
    let mut seen_none = false;
    let mut size = T::Word::ZERO;
    for (k, v) in self.valid.iter() {
      if v {
        if seen_none {
          return None;
        }
        size = T::into_word(k).inc();
      } else {
        seen_none = true;
      }
    }
    Some(unsafe { Size::<T>::from_word_unchecked(size) })
  }

  /// Returns true if the map contains the key.
  pub fn contains(&self, value: T) -> bool {
    self.valid.get(value)
  }

  pub(crate) fn into_partial(mut self) -> T::PartialArray {
    self.valid.clear();
    mem::replace(&mut self.data, T::new_partial())
  }
}

impl<T: EnumArrayHelper<V>, V> Default for EnumOptionMap<T, V> {
  fn default() -> Self {
    EnumOptionMap::<T, V>::new()
  }
}

impl<T: EnumArrayHelper<V>, V> Drop for EnumOptionMap<T, V> {
  fn drop(&mut self) {
    self.clear()
  }
}

impl<T: EnumArrayHelper<V>, V: PartialEq> PartialEq for EnumOptionMap<T, V> {
  fn eq(&self, other: &Self) -> bool {
    for key in T::iter() {
      let i = key.into_word();
      if self.get_internal(i) != other.get_internal(i) {
        return false;
      }
    }
    true
  }
}

impl<T: EnumArrayHelper<V>, V: Eq> Eq for EnumOptionMap<T, V> {}

impl<T: EnumArrayHelper<V>, V: Hash> Hash for EnumOptionMap<T, V> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for key in T::iter() {
      self.get(key).hash(state);
    }
  }
}
