use crate::base::EnumFlagsHelper;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Index, Range};

#[derive(Copy, Clone)]
pub struct EnumFlags<T: EnumFlagsHelper> {
  data: T::FlagsArray,
}

const BITS: usize = 8;

impl<T: EnumFlagsHelper> EnumFlags<T> {
  pub fn new() -> Self {
    EnumFlags {
      data: Default::default(),
    }
  }

  #[inline]
  pub(crate) fn set_internal(&mut self, i: usize, x: bool) {
    debug_assert!(i < T::SIZE, "Enum out of bounds: {} >= {}", i, T::SIZE);
    let j = i / BITS;
    let mask = 1 << (i % BITS);
    let slice = T::slice_flags_mut(&mut self.data);
    slice[j] = if x { slice[j] | mask } else { slice[j] & !mask }
  }

  pub fn set(&mut self, e: T, x: bool) {
    self.set_internal(T::into_usize(e), x)
  }

  pub fn clear(&mut self) {
    self.data = Default::default();
  }

  #[inline]
  pub(crate) fn get_internal(&self, i: usize) -> bool {
    debug_assert!(i < T::SIZE, "Enum out of bounds: {} >= {}", i, T::SIZE);
    let j = i / BITS;
    let slice = T::slice_flags(&self.data);
    (slice[j] >> (i % BITS)) & 1 == 1
  }

  pub fn get(&self, e: T) -> bool {
    self.get_internal(T::into_usize(e))
  }

  pub fn iter(&self) -> EnumFlagsIter<T> {
    EnumFlagsIter {
      flags: self,
      iter: 0..T::SIZE,
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
    let last = !0 >> (BITS - T::SIZE % BITS);
    slice[..T::SIZE / BITS].iter().all(|&val| val == !0)
      && (T::SIZE % BITS == 0 || slice[T::SIZE / BITS] == last)
  }
}

impl<T: EnumFlagsHelper + Debug> Debug for EnumFlags<T> {
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.debug_map().entries(self.iter()).finish()
  }
}

impl<T: EnumFlagsHelper> Default for EnumFlags<T> {
  fn default() -> Self {
    EnumFlags::<T>::new()
  }
}

const TRUE: &bool = &true;
const FALSE: &bool = &false;

impl<T: EnumFlagsHelper> Index<T> for EnumFlags<T> {
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

pub struct EnumFlagsIter<'a, T: EnumFlagsHelper> {
  flags: &'a EnumFlags<T>,
  iter: Range<usize>,
}

impl<'a, T: EnumFlagsHelper> Iterator for EnumFlagsIter<'a, T> {
  type Item = (T, bool);

  fn next(&mut self) -> Option<Self::Item> {
    let flags = &self.flags;
    self
      .iter
      .next()
      .map(|i| (T::from_usize(i), flags.get_internal(i)))
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }

  fn fold<B, F>(self, init: B, f: F) -> B
  where
    F: FnMut(B, Self::Item) -> B,
  {
    let flags = &self.flags;
    self
      .iter
      .map(|i| (T::from_usize(i), flags.get_internal(i)))
      .fold(init, f)
  }
}
