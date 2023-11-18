use crate::sub_base::RawBitsetWord;
use crate::sub_base::RawSizeWord;
use crate::Enumoid;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Index;

/// A set of enumoid `T`'s members.
#[derive(Copy, Clone)]
pub struct EnumSet<T: Enumoid> {
  data: T::BitsetArray,
}

impl<T: Enumoid> EnumSet<T> {
  /// Creates a new empty set.
  pub fn new() -> Self {
    EnumSet {
      data: T::DEFAULT_BITSET,
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
    let j = i.as_() / T::BITSET_WORD_BITS;
    let mask = T::BitsetWord::ONE << (i.as_() % T::BITSET_WORD_BITS);
    let set = if x { mask } else { T::BitsetWord::ZERO };
    let slice = T::slice_bitset_mut(&mut self.data);
    let bits = unsafe { slice.get_unchecked_mut(j) };
    *bits = *bits & !mask | set;
  }

  /// Sets whether a member is in the set.
  pub fn set(&mut self, e: T, x: bool) {
    self.set_internal(T::into_word(e), x)
  }

  /// Clears all the members from the set.
  pub fn clear(&mut self) {
    self.data = T::DEFAULT_BITSET;
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
    let j = i.as_() / T::BITSET_WORD_BITS;
    let slice = T::slice_bitset(&self.data);
    let bits = unsafe { *slice.get_unchecked(j) };
    (bits >> (i.as_() % T::BITSET_WORD_BITS)) & T::BitsetWord::ONE
      == T::BitsetWord::ONE
  }

  /// Returns true if a specific member is in the set.
  pub fn get(&self, e: T) -> bool {
    self.get_internal(T::into_word(e))
  }

  /// Returns an iterator over the members of the set.
  pub fn iter(&self) -> EnumSetIter<T> {
    EnumSetIter {
      flags: self,
      iter: T::word_range(T::Word::ZERO, T::SIZE_WORD),
    }
  }

  /// Returns the number of members in the set.
  pub fn count(&self) -> usize {
    let slice = T::slice_bitset(&self.data);
    slice.iter().fold(0, |acc, &val| acc + val.count_ones())
  }

  /// Returns true if there are any members in the set.
  pub fn any(&self) -> bool {
    let slice = T::slice_bitset(&self.data);
    slice.iter().any(|&val| val != T::BitsetWord::ZERO)
  }

  /// Returns true if all possible members are in the set.
  pub fn all(&self) -> bool {
    let slice = T::slice_bitset(&self.data);
    let last = T::BitsetWord::ONES
      >> (T::BITSET_WORD_BITS - T::SIZE % T::BITSET_WORD_BITS);
    slice[..T::SIZE / T::BITSET_WORD_BITS]
      .iter()
      .all(|&val| val == T::BitsetWord::ONES)
      && (T::SIZE % T::BITSET_WORD_BITS == 0
        || slice[T::SIZE / T::BITSET_WORD_BITS] == last)
  }
}

impl<T: Enumoid + Debug> Debug for EnumSet<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.debug_map().entries(self.iter()).finish()
  }
}

impl<T: Enumoid> Default for EnumSet<T> {
  fn default() -> Self {
    EnumSet::<T>::new()
  }
}

impl<T: Enumoid> PartialEq for EnumSet<T> {
  fn eq(&self, other: &Self) -> bool {
    T::slice_bitset(&self.data) == T::slice_bitset(&other.data)
  }
}

impl<T: Enumoid> Eq for EnumSet<T> {}

impl<T: Enumoid> Hash for EnumSet<T> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    T::slice_bitset(&self.data).hash(state);
  }
}

const TRUE: &bool = &true;
const FALSE: &bool = &false;

impl<T: Enumoid> Index<T> for EnumSet<T> {
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

pub struct EnumSetIter<'a, T: Enumoid> {
  flags: &'a EnumSet<T>,
  iter: T::WordRange,
}

impl<'a, T: Enumoid> Iterator for EnumSetIter<'a, T> {
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

impl<'a, T: Enumoid> ExactSizeIterator for EnumSetIter<'a, T> {}
