use num_traits::{AsPrimitive, FromPrimitive, PrimInt};
use std::fmt::Debug;
use std::iter::Iterator;
use std::iter::Map;
use std::mem;

/// Iterator for Enumoids.
pub type EnumoidIter<T> =
  Map<<T as Enumoid>::WordRange, fn(<T as Enumoid>::Word) -> T>;

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Size<T: Enumoid>(T::Word);

impl<T: Enumoid> Size<T> {
  pub const EMPTY: Size<T> = Size(T::ZERO_WORD);
  pub const FULL: Size<T> = Size(T::SIZE_WORD);

  #[inline]
  pub fn from_last_key(value: T) -> Self {
    Size(value.into_word() + T::ONE_WORD)
  }

  #[inline]
  pub(crate) unsafe fn from_word_unchecked(value: T::Word) -> Self {
    debug_assert!(
      value <= T::SIZE_WORD,
      "from_word_unchecked: Size out of bounds: {:?} >= {:?}",
      value,
      T::SIZE_WORD
    );
    Size(value)
  }

  #[inline]
  pub fn into_usize(self) -> usize {
    self.0.as_()
  }

  pub fn from_usize(sz: usize) -> Option<Self> {
    if sz <= T::SIZE {
      Some(Size(T::Word::from_usize(sz).unwrap()))
    } else {
      None
    }
  }

  #[inline]
  pub fn size(&self) -> T::Word {
    self.0
  }

  #[inline]
  pub fn last_key(&self) -> Option<T> {
    if self.0 > T::ZERO_WORD {
      Some(unsafe { T::from_word_unchecked(self.0 - T::ONE_WORD) })
    } else {
      None
    }
  }

  #[inline]
  pub fn next(self, value: T) -> Option<T> {
    let w = value.into_word() + T::ONE_WORD;
    if w < self.size() {
      Some(unsafe { T::from_word_unchecked(w) })
    } else {
      None
    }
  }

  #[inline]
  pub fn prev(self, value: T) -> Option<T> {
    value.prev()
  }

  #[inline]
  pub fn next_wrapped(self, value: T) -> T {
    let w = value.into_word() + T::ONE_WORD;
    let q = if w < self.size() { w } else { T::ZERO_WORD };
    unsafe { T::from_word_unchecked(q) }
  }

  #[inline]
  pub fn prev_wrapped(self, value: T) -> T {
    let w = value.into_word();
    let q = if w > T::ZERO_WORD { w } else { self.size() } - T::ONE_WORD;
    unsafe { T::from_word_unchecked(q) }
  }

  #[inline]
  pub fn iter(self) -> EnumoidIter<T> {
    T::word_range(T::ZERO_WORD, self.size())
      .map(|w| unsafe { T::from_word_unchecked(w) })
  }

  #[inline]
  pub fn iter_until(self, until: T) -> EnumoidIter<T> {
    let w = until.into_word();
    let s = if w + T::ONE_WORD < self.size() {
      unsafe { Size::from_word_unchecked(w) }
    } else {
      self
    };
    s.iter()
  }

  #[inline]
  pub fn iter_from(self, from: T) -> EnumoidIter<T> {
    T::word_range(from.into_word(), self.size())
      .map(|w| unsafe { T::from_word_unchecked(w) })
  }

  #[inline]
  pub fn iter_from_until(self, from: T, until: T) -> EnumoidIter<T> {
    let w = until.into_word();
    let s = if w + T::ONE_WORD < self.size() {
      unsafe { Size::from_word_unchecked(w) }
    } else {
      self
    };
    s.iter_from(from)
  }
}

/// Trait for enumerable types.
pub trait Enumoid: Sized {
  type Word: AsPrimitive<usize> + FromPrimitive + PrimInt + Debug;
  type WordRange: Iterator<Item = Self::Word>;
  const SIZE: usize;
  const SIZE_WORD: Self::Word;
  const ZERO_WORD: Self::Word;
  const ONE_WORD: Self::Word;
  fn into_word(self) -> Self::Word;
  /// # Safety
  /// The input word must be less than SIZE.
  unsafe fn from_word_unchecked(value: Self::Word) -> Self;
  fn word_range(base: Self::Word, sz: Self::Word) -> Self::WordRange;

  #[inline]
  fn from_word(value: Self::Word) -> Option<Self> {
    if value < Self::SIZE_WORD {
      Some(unsafe { Self::from_word_unchecked(value) })
    } else {
      None
    }
  }

  #[inline]
  fn next(self) -> Option<Self> {
    Size::FULL.next(self)
  }

  #[inline]
  fn prev(self) -> Option<Self> {
    let w = self.into_word();
    if w > Self::ZERO_WORD {
      Some(unsafe { Self::from_word_unchecked(w - Self::ONE_WORD) })
    } else {
      None
    }
  }

  #[inline]
  fn next_wrapped(self) -> Self {
    Size::FULL.next_wrapped(self)
  }

  #[inline]
  fn prev_wrapped(self) -> Self {
    Size::FULL.prev_wrapped(self)
  }

  #[inline]
  fn iter() -> EnumoidIter<Self> {
    Size::FULL.iter()
  }

  #[inline]
  fn iter_until(until: Self) -> EnumoidIter<Self> {
    Size::from_last_key(until).iter()
  }

  #[inline]
  fn iter_from(from: Self) -> EnumoidIter<Self> {
    Size::FULL.iter_from(from)
  }

  #[inline]
  fn iter_from_until(from: Self, until: Self) -> EnumoidIter<Self> {
    Size::from_last_key(until).iter_from(from)
  }
}

pub trait Enumoid1: Enumoid {
  const FIRST: Self;
  const LAST: Self;
}

pub trait EnumFlagsHelper: Enumoid {
  type FlagsArray: Sized + Default;
  const BITS: usize;
  const BITS_WORD: Self::Word;
  fn slice_flags(arr: &Self::FlagsArray) -> &[u8];
  fn slice_flags_mut(arr: &mut Self::FlagsArray) -> &mut [u8];
}

pub trait EnumArrayHelper<V: Sized>: Enumoid {
  type PartialArray: Sized;
  type TotalArray: Sized;

  fn partial_slice(p: &Self::PartialArray) -> &[mem::MaybeUninit<V>];
  fn partial_slice_mut(
    p: &mut Self::PartialArray,
  ) -> &mut [mem::MaybeUninit<V>];
  /// # Safety
  /// All the elements in the input array must be initialised.
  unsafe fn partial_to_total(p: Self::PartialArray) -> Self::TotalArray;

  fn total_slice(t: &Self::TotalArray) -> &[V];
  fn total_slice_mut(t: &mut Self::TotalArray) -> &mut [V];
  fn total_to_partial(t: Self::TotalArray) -> Self::PartialArray;

  #[inline]
  #[allow(clippy::uninit_assumed_init)]
  fn new_partial() -> Self::PartialArray {
    unsafe { mem::MaybeUninit::uninit().assume_init() }
  }
}
