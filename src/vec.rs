use crate::base::EnumArrayHelper;
use crate::base::Size;
use crate::iter::EnumSliceIter;
use crate::iter::EnumSliceIterMut;
use crate::opt_map::EnumOptionMap;
use crate::raw::RawIndex;
use std::convert::TryFrom;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;
use std::mem;
use std::ops::{Index, IndexMut};

/// A vector of values `V` indexed by enumoid `T`.
pub struct EnumVec<T: EnumArrayHelper<V>, V> {
  pub(crate) len: T::Word,
  pub(crate) data: T::PartialArray,
}

impl<T: EnumArrayHelper<V>, V> EnumVec<T, V> {
  /// Creates a new vector with no elements.
  pub fn new() -> Self {
    EnumVec {
      len: T::Word::ZERO,
      data: T::new_partial(),
    }
  }

  /// Creates a new vector with a specified number of elements generated by a
  /// callback function.
  pub fn new_with<F>(size: Size<T>, mut f: F) -> Self
  where
    F: FnMut(T) -> V,
  {
    let mut vec = Self::new();
    for key in size.iter() {
      vec.push(f(key));
    }
    vec
  }

  /// Returns a slice containing all the values in the vector.
  pub fn as_slice(&self) -> &[V] {
    unsafe {
      hint_assert!(
        self.len <= T::SIZE_WORD,
        "Length out of bounds: {:?} > {:?}",
        self.len,
        T::SIZE
      );
      let inited =
        T::partial_slice(&self.data).get_unchecked(0..self.len.as_());
      &*(inited as *const [std::mem::MaybeUninit<V>] as *const [V])
    }
  }

  /// Returns a mutable slice containing all the values in the vector.
  pub fn as_slice_mut(&mut self) -> &mut [V] {
    unsafe {
      hint_assert!(
        self.len <= T::SIZE_WORD,
        "Length out of bounds: {:?} > {:?}",
        self.len,
        T::SIZE
      );
      let inited = T::partial_slice_mut(&mut self.data)
        .get_unchecked_mut(0..self.len.as_());
      &mut *(inited as *mut [std::mem::MaybeUninit<V>] as *mut [V])
    }
  }

  /// Returns a reference to the value associated with a given key,
  /// or `None` if the key is beyond the end of the vector.
  pub fn get(&self, key: T) -> Option<&V> {
    let i = T::into_word(key);
    self.as_slice().get(i.as_())
  }

  /// Returns a mutable reference to the value associated with a given key,
  /// or `None` if the key is beyond the end of the vector.
  pub fn get_mut(&mut self, key: T) -> Option<&mut V> {
    let i = T::into_word(key);
    self.as_slice_mut().get_mut(i.as_())
  }

  /// Returns true if the vector is empty.
  pub fn is_empty(&self) -> bool {
    self.len == T::Word::ZERO
  }

  /// Returns true if the vector is fully populated.
  pub fn is_full(&self) -> bool {
    self.len == T::SIZE_WORD
  }

  /// Returns the size of the vector.
  pub fn size(&self) -> Size<T> {
    unsafe { Size::<T>::from_word_unchecked(self.len) }
  }

  /// Swaps two elements in the vector.
  ///
  /// # Panics
  /// Panics if `a` or `b` are beyond the end of the vector.
  pub fn swap(&mut self, a: T, b: T) {
    self
      .as_slice_mut()
      .swap(T::into_word(a).as_(), T::into_word(b).as_())
  }

  /// Removes an element and returns it, replacing it with the last element.
  ///
  /// # Panics
  /// Panics if `key` is beyond the end of the vector.
  pub fn swap_remove(&mut self, key: T) -> V {
    let index = T::into_word(key).as_();
    assert!(index < self.len.as_());
    let slice = T::partial_slice_mut(&mut self.data);
    self.len = self.len.dec();
    unsafe {
      let value = slice[index].assume_init_read();
      slice[index].write(slice[self.len.as_()].assume_init_read());
      value
    }
  }

  /// Clears all the elements from the vector.
  pub fn clear(&mut self) {
    for cell in
      T::partial_slice_mut(&mut self.data)[0..self.len.as_()].iter_mut()
    {
      unsafe { cell.assume_init_drop() };
    }
    self.len = T::Word::ZERO;
  }

  /// Adds an element to the end of the vector.
  ///
  /// # Panics
  /// Panics if the vector is already full.
  pub fn push(&mut self, value: V) {
    let len = self.len.as_();
    T::partial_slice_mut(&mut self.data)[len] =
      mem::MaybeUninit::<V>::new(value);
    self.len = self.len.inc();
  }

  /// Removes an element from the end of the vector and returns it,
  /// or `None` if the vector is empty.
  pub fn pop(&mut self) -> Option<V> {
    if self.len == T::Word::ZERO {
      None
    } else {
      let i = self.len.as_() - 1;
      let cell = &T::partial_slice_mut(&mut self.data)[i];
      self.len = self.len.dec();
      Some(unsafe { cell.assume_init_read() })
    }
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
    self.clear()
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

impl<T: EnumArrayHelper<V>, V: PartialEq> PartialEq for EnumVec<T, V> {
  fn eq(&self, other: &Self) -> bool {
    self.as_slice() == other.as_slice()
  }
}

impl<T: EnumArrayHelper<V>, V: Eq> Eq for EnumVec<T, V> {}

impl<T: EnumArrayHelper<V>, V: Hash> Hash for EnumVec<T, V> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    self.as_slice().hash(state);
  }
}

impl<T: EnumArrayHelper<V>, V> Index<T> for EnumVec<T, V> {
  type Output = V;

  fn index(&self, i: T) -> &V {
    &self.as_slice()[T::into_word(i).as_()]
  }
}

impl<T: EnumArrayHelper<V>, V> IndexMut<T> for EnumVec<T, V> {
  fn index_mut(&mut self, i: T) -> &mut V {
    &mut self.as_slice_mut()[T::into_word(i).as_()]
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

impl<T: EnumArrayHelper<V>, V> TryFrom<EnumOptionMap<T, V>> for EnumVec<T, V> {
  type Error = ();
  fn try_from(from: EnumOptionMap<T, V>) -> Result<Self, Self::Error> {
    match from.is_vec() {
      Some(size) => Ok(EnumVec {
        len: size.to_word(),
        data: from.into_partial(),
      }),
      None => Err(()),
    }
  }
}
