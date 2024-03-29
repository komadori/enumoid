use crate::sub_base::BitsetWordTrait;
use crate::sub_base::RawSizeWord;
use std::fmt::Debug;
use std::iter::Iterator;
use std::iter::Map;
use std::mem;

macro_rules! hint_assert {
    ($x:expr, $($arg:tt)*) => {
      debug_assert!($x, $($arg)*);
      if !$x {
        std::hint::unreachable_unchecked();
      }
    }
}

/// Iterator for Enumoids.
pub type EnumoidIter<T> =
  Map<<T as Enumoid>::WordRange, fn(<T as Enumoid>::Word) -> T>;

/// A counter between 0 and the number of values inhabiting `T`.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EnumSize<T: Enumoid>(T::Word);

impl<T: Enumoid> Copy for EnumSize<T> {}

impl<T: Enumoid> Clone for EnumSize<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T: Enumoid> EnumSize<T> {
  pub const EMPTY: EnumSize<T> = EnumSize(T::Word::ZERO);
  pub const FULL: EnumSize<T> = EnumSize(T::SIZE_WORD);

  #[inline]
  pub fn from_last_value(value: T) -> Self {
    EnumSize(value.into_word().inc())
  }

  #[inline]
  pub(crate) unsafe fn from_word_unchecked(value: T::Word) -> Self {
    hint_assert!(
      value <= T::SIZE_WORD,
      "from_word_unchecked: Size out of bounds: {:?} >= {:?}",
      value,
      T::SIZE_WORD
    );
    EnumSize(value)
  }

  pub fn from_usize(sz: usize) -> Option<Self> {
    if sz <= T::SIZE {
      Some(EnumSize(T::Word::from_usize_unchecked(sz)))
    } else {
      None
    }
  }

  #[inline]
  pub fn into_word(&self) -> T::Word {
    self.0
  }

  #[inline]
  pub fn into_usize(&self) -> usize {
    self.0.as_()
  }

  #[inline]
  pub fn into_last_value(&self) -> Option<T> {
    if self.0 > T::Word::ZERO {
      Some(unsafe { T::from_word_unchecked(self.0.dec()) })
    } else {
      None
    }
  }

  /// Returns the next index or None.
  ///
  /// # Panics
  /// Panics if the value is beyond the size.
  #[inline]
  pub fn next_index(&self, index: EnumIndex<T>) -> Option<EnumIndex<T>> {
    assert!(index.0 < self.0);
    let nw = index.0.inc();
    if nw < self.0 {
      Some(unsafe { EnumIndex::from_word_unchecked(nw) })
    } else {
      None
    }
  }

  /// Returns the next element or None.
  ///
  /// # Panics
  /// Panics if the value is beyond the size.
  #[inline]
  pub fn next(&self, value: T) -> Option<T> {
    self.next_index(value.into()).map(|i| i.into_value())
  }

  /// Returns the previous index or None.
  ///
  /// # Panics
  /// Panics if the value is beyond the size.
  #[inline]
  pub fn prev_index(&self, index: EnumIndex<T>) -> Option<EnumIndex<T>> {
    assert!(index.0 < self.0);
    if index.0 > T::Word::ZERO {
      Some(unsafe { EnumIndex::from_word_unchecked(index.0.dec()) })
    } else {
      None
    }
  }

  /// Returns the previous element or None.
  ///
  /// # Panics
  /// Panics if the value is beyond the size.
  #[inline]
  pub fn prev(&self, value: T) -> Option<T> {
    self.prev_index(value.into()).map(|i| i.into_value())
  }

  /// Returns the next index or wraps around to the beginning.
  ///
  /// # Panics
  /// Panics if the value is beyond the size.
  #[inline]
  pub fn next_index_wrapped(&self, index: EnumIndex<T>) -> EnumIndex<T> {
    assert!(index.0 < self.0);
    let nw = index.0.inc();
    let q = if nw < self.0 { nw } else { T::Word::ZERO };
    unsafe { EnumIndex::from_word_unchecked(q) }
  }

  /// Returns the next element or wraps around to the beginning.
  ///
  /// # Panics
  /// Panics if the value is beyond the size.
  #[inline]
  pub fn next_wrapped(&self, value: T) -> T {
    self.next_index_wrapped(value.into()).into_value()
  }

  /// Returns the previous index or wraps around to the end.
  ///
  /// # Panics
  /// Panics if the value is beyond the size.
  #[inline]
  pub fn prev_index_wrapped(&self, index: EnumIndex<T>) -> EnumIndex<T> {
    assert!(index.0 < self.0);
    let q = if index.0 > T::Word::ZERO {
      index.0
    } else {
      self.0
    }
    .dec();
    unsafe { EnumIndex::from_word_unchecked(q) }
  }

  /// Returns the previous element or wraps around to the end.
  ///
  /// # Panics
  /// Panics if the value is beyond the size.
  #[inline]
  pub fn prev_wrapped(&self, value: T) -> T {
    self.prev_index_wrapped(value.into()).into_value()
  }

  #[inline]
  pub fn contains_index(&self, index: EnumIndex<T>) -> bool {
    index.0 < self.0
  }

  #[inline]
  pub fn contains(&self, value: T) -> bool {
    value.into_word() < self.0
  }

  #[inline]
  pub fn iter(&self) -> EnumoidIter<T> {
    T::word_range(T::Word::ZERO, self.0)
      .map(|w| unsafe { T::from_word_unchecked(w) })
  }

  #[inline]
  pub fn iter_until(&self, until: T) -> EnumoidIter<T> {
    let w = until.into_word();
    if w.inc() < self.0 {
      unsafe { EnumSize::from_word_unchecked(w) }.iter()
    } else {
      self.iter()
    }
  }

  #[inline]
  pub fn iter_from(&self, from: T) -> EnumoidIter<T> {
    T::word_range(from.into_word(), self.0)
      .map(|w| unsafe { T::from_word_unchecked(w) })
  }

  #[inline]
  pub fn iter_from_until(&self, from: T, until: T) -> EnumoidIter<T> {
    let w = until.into_word();
    if w.inc() < self.0 {
      unsafe { EnumSize::from_word_unchecked(w) }.iter_from(from)
    } else {
      self.iter_from(from)
    }
  }
}

/// A counter between 0 and the highest index into the values inhabiting `T`.
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct EnumIndex<T: Enumoid>(T::Word);

impl<T: Enumoid> Copy for EnumIndex<T> {}

impl<T: Enumoid> Clone for EnumIndex<T> {
  fn clone(&self) -> Self {
    *self
  }
}

impl<T: Enumoid> EnumIndex<T> {
  #[inline]
  pub(crate) unsafe fn from_word_unchecked(value: T::Word) -> Self {
    hint_assert!(
      value < T::SIZE_WORD,
      "from_word_unchecked: Index out of bounds: {:?} >= {:?}",
      value,
      T::SIZE_WORD
    );
    EnumIndex(value)
  }

  pub fn from_usize(sz: usize) -> Option<Self> {
    if sz < T::SIZE {
      Some(EnumIndex(T::Word::from_usize_unchecked(sz)))
    } else {
      None
    }
  }

  pub fn from_value(value: T) -> Self {
    unsafe { Self::from_word_unchecked(value.into_word()) }
  }

  #[inline]
  pub fn into_value(self) -> T {
    unsafe { T::from_word_unchecked(self.0) }
  }

  #[inline]
  pub fn into_word(self) -> T::Word {
    unsafe {
      hint_assert!(
        self.0 < T::SIZE_WORD,
        "Index out of bounds: {:?} >= {:?}",
        self.0,
        T::SIZE
      );
    };
    self.0
  }

  #[inline]
  pub fn into_usize(self) -> usize {
    self.into_word().as_()
  }

  /// Returns the next index or None.
  #[inline]
  pub fn next(self) -> Option<Self> {
    EnumSize::FULL.next_index(self)
  }

  /// Returns the previous index or None.
  #[inline]
  pub fn prev(self) -> Option<Self> {
    EnumSize::FULL.prev_index(self)
  }

  /// Returns the next index or wraps around to the beginning.
  #[inline]
  pub fn next_wrapped(self) -> Self {
    EnumSize::FULL.next_index_wrapped(self)
  }

  /// Returns the previous index or wraps around to the end.
  #[inline]
  pub fn prev_wrapped(self) -> Self {
    EnumSize::FULL.prev_index_wrapped(self)
  }
}

impl<T: Enumoid> From<T> for EnumIndex<T> {
  #[inline]
  fn from(value: T) -> Self {
    EnumIndex(value.into_word())
  }
}

/// Trait for enumerable types.
///
/// Some members are hidden. Impls should only be defined via the `Enumoid` derive macro.
pub trait Enumoid: Sized {
  type Word: RawSizeWord;
  const SIZE: usize;
  const FIRST: Self;
  const LAST: Self;
  fn into_word(self) -> Self::Word;

  #[doc(hidden)]
  type WordRange: Iterator<Item = Self::Word>;
  #[doc(hidden)]
  const SIZE_WORD: Self::Word;
  /// # Safety
  /// The input word must be less than SIZE.
  #[doc(hidden)]
  unsafe fn from_word_unchecked(value: Self::Word) -> Self;
  #[doc(hidden)]
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
    EnumSize::FULL.next(self)
  }

  #[inline]
  fn prev(self) -> Option<Self> {
    EnumSize::FULL.prev(self)
  }

  #[inline]
  fn next_wrapped(self) -> Self {
    EnumSize::FULL.next_wrapped(self)
  }

  #[inline]
  fn prev_wrapped(self) -> Self {
    EnumSize::FULL.prev_wrapped(self)
  }

  #[inline]
  fn iter() -> EnumoidIter<Self> {
    EnumSize::FULL.iter()
  }

  #[inline]
  fn iter_until(until: Self) -> EnumoidIter<Self> {
    EnumSize::from_last_value(until).iter()
  }

  #[inline]
  fn iter_from(from: Self) -> EnumoidIter<Self> {
    EnumSize::FULL.iter_from(from)
  }

  #[inline]
  fn iter_from_until(from: Self, until: Self) -> EnumoidIter<Self> {
    EnumSize::from_last_value(until).iter_from(from)
  }
}

/// Workaround for const generics not supporting associated consts yet.
///
/// All the members are hidden. Impls should only be defined via the `Enumoid` derive macro.
pub trait EnumArrayHelper<V: Sized>: Enumoid {
  #[doc(hidden)]
  type PartialArray: Sized;
  #[doc(hidden)]
  type TotalArray: Sized;

  #[doc(hidden)]
  fn partial_slice(p: &Self::PartialArray) -> &[mem::MaybeUninit<V>];
  #[doc(hidden)]
  fn partial_slice_mut(
    p: &mut Self::PartialArray,
  ) -> &mut [mem::MaybeUninit<V>];
  /// # Safety
  /// All the elements in the input array must be initialised.
  #[doc(hidden)]
  unsafe fn partial_to_total(p: Self::PartialArray) -> Self::TotalArray;

  #[doc(hidden)]
  fn total_slice(t: &Self::TotalArray) -> &[V];
  #[doc(hidden)]
  fn total_slice_mut(t: &mut Self::TotalArray) -> &mut [V];
  #[doc(hidden)]
  fn total_to_partial(t: Self::TotalArray) -> Self::PartialArray;

  #[inline]
  #[allow(clippy::uninit_assumed_init)]
  #[doc(hidden)]
  fn new_partial() -> Self::PartialArray {
    unsafe { mem::MaybeUninit::uninit().assume_init() }
  }
}

/// Workaround for const generics not supporting associated consts yet.
///
/// All the members are hidden. Impls should only be defined via the `Enumoid` derive macro.
pub trait EnumSetHelper<BitsetWord: BitsetWordTrait>: Enumoid {
  #[doc(hidden)]
  type BitsetWord: BitsetWordTrait;
  #[doc(hidden)]
  type BitsetArray: Sized;
  #[doc(hidden)]
  const BITSET_WORD_BITS: usize;
  #[doc(hidden)]
  const BITSET_WORDS: usize =
    (Self::SIZE + Self::BITSET_WORD_BITS - 1) / Self::BITSET_WORD_BITS;
  #[doc(hidden)]
  const DEFAULT_BITSET: Self::BitsetArray;
  #[doc(hidden)]
  fn slice_bitset(arr: &Self::BitsetArray) -> &[Self::BitsetWord];
  #[doc(hidden)]
  fn slice_bitset_mut(arr: &mut Self::BitsetArray) -> &mut [Self::BitsetWord];
}
