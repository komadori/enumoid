use crate::Enumoid;
use num_traits::AsPrimitive;
use std::fmt;
use std::fmt::Debug;
use std::ops::Index;

/// A set of enumoid `T`'s members.
#[derive(Copy, Clone)]
pub struct EnumFlags<T: Enumoid> {
  data: T::FlagsArray,
}

impl<T: Enumoid> EnumFlags<T> {
  /// Creates a new, unset `EnumFlags<T>`.
  pub fn new() -> Self {
    EnumFlags {
      data: T::DEFAULT_FLAGS,
    }
  }

  #[inline]
  pub(crate) fn set_internal(&mut self, i: T::Word, x: bool) {
    debug_assert!(
      i < T::SIZE_WORD,
      "Index out of bounds: {:?} >= {:?}",
      i,
      T::SIZE
    );
    let j = (i / T::FLAGS_BITS_WORD).as_();
    let mask = 1 << (i % T::FLAGS_BITS_WORD).as_();
    let set = if x { mask } else { 0 };
    let slice = T::slice_flags_mut(&mut self.data);
    let bits = unsafe { slice.get_unchecked_mut(j) };
    *bits = *bits & !mask | set;
  }

  pub fn set(&mut self, e: T, x: bool) {
    self.set_internal(T::into_word(e), x)
  }

  pub fn clear(&mut self) {
    self.data = T::DEFAULT_FLAGS;
  }

  #[inline]
  pub(crate) fn get_internal(&self, i: T::Word) -> bool {
    debug_assert!(
      i < T::SIZE_WORD,
      "Index out of bounds: {:?} >= {:?}",
      i,
      T::SIZE
    );
    let j = (i / T::FLAGS_BITS_WORD).as_();
    let slice = T::slice_flags(&self.data);
    let bits = unsafe { slice.get_unchecked(j) };
    (bits >> (i % T::FLAGS_BITS_WORD).as_()) & 1 == 1
  }

  pub fn get(&self, e: T) -> bool {
    self.get_internal(T::into_word(e))
  }

  pub fn iter(&self) -> EnumFlagsIter<T> {
    EnumFlagsIter {
      flags: self,
      iter: T::word_range(T::ZERO_WORD, T::SIZE_WORD),
    }
  }

  pub fn count(&self) -> usize {
    let slice = T::slice_flags(&self.data);
    slice
      .iter()
      .fold(0, |acc, &val| acc + val.count_ones() as usize)
  }

  pub fn any(&self) -> bool {
    let slice = T::slice_flags(&self.data);
    slice.iter().any(|&val| val != 0)
  }

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
