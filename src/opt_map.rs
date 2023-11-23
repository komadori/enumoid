use crate::base::EnumArrayHelper;
use crate::base::EnumSetHelper;
use crate::base::EnumSize;
use crate::set::EnumSet;
use crate::sub_base::BitsetWordTrait;
use crate::sub_base::RawSizeWord;
use crate::EnumIndex;
use std::hash::Hash;
use std::mem;

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
    if self.valid.get_by_index(index) {
      Some(unsafe {
        T::partial_slice(&self.data)[index.into_usize()].assume_init_ref()
      })
    } else {
      None
    }
  }

  /// Returns a reference to the value associated with a given key,
  /// or `None` if the key has no value in the map.
  pub fn get(&self, key: T) -> Option<&V> {
    self.get_by_index(key.into())
  }

  /// Returns a mutable reference to the value associated with a given index,
  /// or `None` if the index has no value in the map.
  pub fn get_by_index_mut(&mut self, index: EnumIndex<T>) -> Option<&mut V> {
    if self.valid.get_by_index(index) {
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
  pub fn get_mut(&mut self, key: T) -> Option<&mut V> {
    self.get_by_index_mut(key.into())
  }

  /// Sets the value associated with a given index.
  pub fn set_by_index(&mut self, index: EnumIndex<T>, value: Option<V>) {
    let cell = &mut T::partial_slice_mut(&mut self.data)[index.into_usize()];
    if self.valid.get_by_index(index) {
      unsafe { cell.assume_init_drop() };
    }
    self.valid.set_by_index(index, value.is_some());
    if let Some(v) = value {
      cell.write(v);
    }
  }

  /// Sets the value associated with a given key.
  pub fn set(&mut self, key: T, value: Option<V>) {
    self.set_by_index(key.into(), value)
  }

  /// Clears all the elements from the map.
  pub fn clear(&mut self) {
    let data = T::partial_slice_mut(&mut self.data);
    for key in T::iter() {
      let index = key.into();
      if self.valid.get_by_index(index) {
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
    Some(unsafe { EnumSize::<T>::from_word_unchecked(size) })
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
