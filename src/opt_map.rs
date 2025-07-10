use crate::base::EnumArrayHelper;
use crate::base::EnumSetHelper;
use crate::base::EnumSize;
use crate::set::EnumSet;
use crate::set::EnumSetIndexIter;
use crate::sub_base::BitsetWordTrait;
use crate::EnumIndex;
use core::slice;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::mem;
use std::mem::MaybeUninit;

/// A partial map from enumoid `T` to values `V`.
///
/// The optional type parameter `BitsetWord` is passed on to an embedded `EnumSet` which is used
/// to store the validity bitmap.
pub struct EnumOptionMap<
  T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
  V,
  BitsetWord: BitsetWordTrait = u8,
> {
  valid: EnumSet<T, BitsetWord>,
  pub(crate) data: T::PartialArray,
}

impl<
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
    V,
    BitsetWord: BitsetWordTrait,
  > EnumOptionMap<T, V, BitsetWord>
{
  /// Creates a new empty map.
  pub fn new() -> Self {
    EnumOptionMap {
      valid: EnumSet::<T, BitsetWord>::new(),
      data: T::new_partial(),
    }
  }

  /// Returns a reference to the value associated with a given index,
  /// or `None` if the index has no value in the map.
  #[inline]
  pub fn get_by_index(&self, index: EnumIndex<T>) -> Option<&V> {
    if self.valid.contains_index(index) {
      Some(unsafe {
        T::partial_slice(&self.data)[index.into_usize()].assume_init_ref()
      })
    } else {
      None
    }
  }

  /// Returns a reference to the value associated with a given key,
  /// or `None` if the key has no value in the map.
  #[inline]
  pub fn get(&self, key: T) -> Option<&V> {
    self.get_by_index(key.into())
  }

  /// Returns a mutable reference to the value associated with a given index,
  /// or `None` if the index has no value in the map.
  #[inline]
  pub fn get_by_index_mut(&mut self, index: EnumIndex<T>) -> Option<&mut V> {
    if self.valid.contains_index(index) {
      Some(unsafe {
        T::partial_slice_mut(&mut self.data)[index.into_usize()]
          .assume_init_mut()
      })
    } else {
      None
    }
  }

  /// Returns a mutable reference to the value associated with a given key,
  /// or `None` if the key has no value in the map.
  #[inline]
  pub fn get_mut(&mut self, key: T) -> Option<&mut V> {
    self.get_by_index_mut(key.into())
  }

  /// Sets the value associated with a given index and returns the old value if one was present.
  #[inline]
  pub fn set_by_index(
    &mut self,
    index: EnumIndex<T>,
    value: Option<V>,
  ) -> Option<V> {
    let cell = &mut T::partial_slice_mut(&mut self.data)[index.into_usize()];
    let old = if self.valid.contains_index(index) {
      Some(unsafe { cell.assume_init_read() })
    } else {
      None
    };
    self.valid.set_by_index(index, value.is_some());
    if let Some(v) = value {
      cell.write(v);
    }
    old
  }

  /// Sets the value associated with a given key and returns the old value if one was present.
  #[inline]
  pub fn set(&mut self, key: T, value: Option<V>) -> Option<V> {
    self.set_by_index(key.into(), value)
  }

  /// Adds a value at the given index to the map and returns the old value if one was present.
  #[inline]
  pub fn insert_by_index(
    &mut self,
    index: EnumIndex<T>,
    value: V,
  ) -> Option<V> {
    self.set_by_index(index, Some(value))
  }

  /// Adds a value with the given key to the map and returns the old value if one was present.
  #[inline]
  pub fn insert(&mut self, key: T, value: V) -> Option<V> {
    self.insert_by_index(key.into(), value)
  }

  /// Removes any value at the given index from the map and returns it if one was present.
  #[inline]
  pub fn remove_by_index(&mut self, index: EnumIndex<T>) -> Option<V> {
    self.set_by_index(index, None)
  }

  /// Removes any value with the given key from the map and returns it if one was present.
  #[inline]
  pub fn remove(&mut self, key: T) -> Option<V> {
    self.remove_by_index(key.into())
  }

  /// Swaps two elements in the map by index.
  #[inline]
  pub fn swap_by_index(&mut self, a: EnumIndex<T>, b: EnumIndex<T>) {
    let valid_a = self.valid.contains_index(a);
    let valid_b = self.valid.contains_index(b);
    let slice = T::partial_slice_mut(&mut self.data);
    if valid_a && valid_b {
      slice.swap(a.into_usize(), b.into_usize());
    } else if valid_a || valid_b {
      self.valid.set_by_index(b, !valid_a);
      self.valid.set_by_index(a, !valid_b);
      let (src, dst) = if valid_a { (a, b) } else { (b, a) };
      unsafe {
        slice[dst.into_usize()]
          .write(slice[src.into_usize()].assume_init_read());
      }
    }
  }

  /// Swaps two elements in the map.
  #[inline]
  pub fn swap(&mut self, a: T, b: T) {
    self.swap_by_index(a.into(), b.into());
  }

  /// Clears all the elements from the map.
  pub fn clear(&mut self) {
    let data = T::partial_slice_mut(&mut self.data);
    for key in T::iter() {
      let index = key.into();
      if self.valid.contains_index(index) {
        let cell = &mut data[index.into_usize()];
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
  pub fn is_vec(&self) -> Option<EnumSize<T>> {
    let mut size = EnumSize::<T>::EMPTY;
    for i in self.valid.iter_index() {
      size = size.increase()?;
      if size.into_last_index() != Some(i) {
        return None;
      }
    }
    Some(size)
  }

  /// Returns true if the map contains the index.
  #[inline]
  pub fn contains_index(&self, index: EnumIndex<T>) -> bool {
    self.valid.contains_index(index)
  }

  /// Returns true if the map contains the key.
  #[inline]
  pub fn contains(&self, value: T) -> bool {
    self.valid.contains(value)
  }

  /// Returns an iterator over the keys and values.
  #[inline]
  pub fn iter(&self) -> EnumOptionMapIter<T, V, BitsetWord> {
    EnumOptionMapIter {
      iter: self.valid.iter_index(),
      data: &self.data,
    }
  }

  /// Returns a mutable iterator over the keys and values.
  #[inline]
  pub fn iter_mut(&mut self) -> EnumOptionMapIterMut<T, V, BitsetWord> {
    EnumOptionMapIterMut {
      iter: self.valid.iter_index(),
      data: T::partial_slice_mut(&mut self.data).iter_mut(),
      prev: 0,
    }
  }

  /// Returns the number of populated keys in the map.
  #[inline]
  pub fn count(&self) -> usize {
    self.valid.count()
  }

  /// Returns a reference to the set of populated keys.
  #[inline]
  pub fn keys(&self) -> &EnumSet<T, BitsetWord> {
    &self.valid
  }

  pub(crate) fn into_partial(mut self) -> T::PartialArray {
    self.valid.clear();
    mem::replace(&mut self.data, T::new_partial())
  }
}

impl<
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord> + Debug,
    V: Debug,
    BitsetWord: BitsetWordTrait,
  > Debug for EnumOptionMap<T, V, BitsetWord>
{
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.debug_map().entries(self.iter()).finish()
  }
}

impl<
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
    V,
    BitsetWord: BitsetWordTrait,
  > Default for EnumOptionMap<T, V, BitsetWord>
{
  fn default() -> Self {
    EnumOptionMap::<T, V, BitsetWord>::new()
  }
}

impl<
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
    V,
    BitsetWord: BitsetWordTrait,
  > Drop for EnumOptionMap<T, V, BitsetWord>
{
  fn drop(&mut self) {
    self.clear()
  }
}

impl<
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
    V: PartialEq,
    BitsetWord: BitsetWordTrait,
  > PartialEq for EnumOptionMap<T, V, BitsetWord>
{
  fn eq(&self, other: &Self) -> bool {
    for key in T::iter() {
      let index = key.into();
      if self.get_by_index(index) != other.get_by_index(index) {
        return false;
      }
    }
    true
  }
}

impl<
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
    V: Eq,
    BitsetWord: BitsetWordTrait,
  > Eq for EnumOptionMap<T, V, BitsetWord>
{
}

impl<
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
    V: Hash,
    BitsetWord: BitsetWordTrait,
  > Hash for EnumOptionMap<T, V, BitsetWord>
{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    for key in T::iter() {
      self.get(key).hash(state);
    }
  }
}

pub struct EnumOptionMapIter<
  'a,
  T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
  V: 'a,
  BitsetWord: BitsetWordTrait,
> {
  iter: EnumSetIndexIter<'a, T, BitsetWord>,
  data: &'a T::PartialArray,
}

impl<
    'a,
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
    V,
    BitsetWord: BitsetWordTrait,
  > Iterator for EnumOptionMapIter<'a, T, V, BitsetWord>
{
  type Item = (T, &'a V);

  fn next(&mut self) -> Option<Self::Item> {
    let index = self.iter.next()?;
    let value = unsafe {
      T::partial_slice(self.data)[index.into_usize()].assume_init_ref()
    };
    Some((index.into_value(), value))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

pub struct EnumOptionMapIterMut<
  'a,
  T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
  V: 'a,
  BitsetWord: BitsetWordTrait,
> {
  iter: EnumSetIndexIter<'a, T, BitsetWord>,
  data: slice::IterMut<'a, MaybeUninit<V>>,
  prev: usize,
}

impl<
    'a,
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
    V,
    BitsetWord: BitsetWordTrait,
  > Iterator for EnumOptionMapIterMut<'a, T, V, BitsetWord>
{
  type Item = (T, &'a mut V);

  fn next(&mut self) -> Option<Self::Item> {
    let index = self.iter.next()?;
    let value = self.data.nth(index.into_usize() - self.prev).unwrap();
    self.prev = index.into_usize() + 1;
    Some((index.into_value(), unsafe { value.assume_init_mut() }))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}
