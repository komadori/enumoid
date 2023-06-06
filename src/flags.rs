use crate::Enumoid;
use num_traits::AsPrimitive;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Index;

/// A set of enumoid `T`'s members.
#[derive(Copy, Clone)]
pub struct EnumFlags<T: Enumoid> {
  data: T::FlagsArray,
}

impl<T: Enumoid> EnumFlags<T> {
  /// Creates a new empty set.
  pub fn new() -> Self {
    EnumFlags {
      data: T::DEFAULT_FLAGS,
    }
  }

  #[inline]
  pub(crate) fn set_internal(&mut self, i: T::Word, x: bool) {
    unsafe {
      hint_assert!(
        i < T::SIZE_WORD,
        "Index out of bounds: {:?} >= {:?}",
        i,
        T::SIZE
      );
    }
    let j = (i / T::FLAGS_BITS_WORD).as_();
    let mask = 1 << (i % T::FLAGS_BITS_WORD).as_();
    let set = if x { mask } else { 0 };
    let slice = T::slice_flags_mut(&mut self.data);
    let bits = unsafe { slice.get_unchecked_mut(j) };
    *bits = *bits & !mask | set;
  }

  /// Sets whether a member is in the set.
  pub fn set(&mut self, e: T, x: bool) {
    self.set_internal(T::into_word(e), x)
  }

  /// Clears all the members from the set.
  pub fn clear(&mut self) {
    self.data = T::DEFAULT_FLAGS;
  }

  #[inline]
  pub(crate) fn get_internal(&self, i: T::Word) -> bool {
    unsafe {
      hint_assert!(
        i < T::SIZE_WORD,
        "Index out of bounds: {:?} >= {:?}",
        i,
        T::SIZE
      );
    }
    let j = (i / T::FLAGS_BITS_WORD).as_();
    let slice = T::slice_flags(&self.data);
    let bits = unsafe { slice.get_unchecked(j) };
    (bits >> (i % T::FLAGS_BITS_WORD).as_()) & 1 == 1
  }

  /// Returns true if a specific member is in the set.
  pub fn get(&self, e: T) -> bool {
    self.get_internal(T::into_word(e))
  }

  /// Returns an iterator over the members of the set.
  pub fn iter(&self) -> EnumFlagsIter<T> {
    EnumFlagsIter {
      flags: self,
      iter: T::word_range(T::ZERO_WORD, T::SIZE_WORD),
    }
  }

  /// Returns the number of members in the set.
  pub fn count(&self) -> usize {
    let slice = T::slice_flags(&self.data);
    slice
      .iter()
      .fold(0, |acc, &val| acc + val.count_ones() as usize)
  }

  /// Returns true if there are any members in the set.
  pub fn any(&self) -> bool {
    let slice = T::slice_flags(&self.data);
    slice.iter().any(|&val| val != 0)
  }

  /// Returns true if all possible members are in the set.
  pub fn all(&self) -> bool {
    let slice = T::slice_flags(&self.data);
    let last = !0 >> (T::FLAGS_BITS - T::SIZE % T::FLAGS_BITS);
    slice[..T::SIZE / T::FLAGS_BITS]
      .iter()
      .all(|&val| val == !0)
      && (T::SIZE % T::FLAGS_BITS == 0
        || slice[T::SIZE / T::FLAGS_BITS] == last)
  }
}

impl<T: Enumoid + Debug> Debug for EnumFlags<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.debug_map().entries(self.iter()).finish()
  }
}

impl<T: Enumoid> Default for EnumFlags<T> {
  fn default() -> Self {
    EnumFlags::<T>::new()
  }
}

impl<T: Enumoid> PartialEq for EnumFlags<T> {
  fn eq(&self, other: &Self) -> bool {
    T::slice_flags(&self.data) == T::slice_flags(&other.data)
  }
}

impl<T: Enumoid> Eq for EnumFlags<T> {}

impl<T: Enumoid> Hash for EnumFlags<T> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    T::slice_flags(&self.data).hash(state);
  }
}

const TRUE: &bool = &true;
const FALSE: &bool = &false;

impl<T: Enumoid> Index<T> for EnumFlags<T> {
  type Output = bool;

  #[inline]
  fn index(&self, i: T) -> &bool {
    if self.get(i) {
      TRUE
    } else {
      FALSE
    }
  }
}

pub struct EnumFlagsIter<'a, T: Enumoid> {
  flags: &'a EnumFlags<T>,
  iter: T::WordRange,
}

impl<'a, T: Enumoid> Iterator for EnumFlagsIter<'a, T> {
  type Item = (T, bool);

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    let word = self.iter.next()?;
    let key = unsafe { T::from_word_unchecked(word) };
    Some((key, self.flags.get_internal(word)))
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

impl<'a, T: Enumoid> ExactSizeIterator for EnumFlagsIter<'a, T> {}
