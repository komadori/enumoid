use num_traits::{AsPrimitive, FromPrimitive, PrimInt};
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

/// A counter between 0 and the number of values inhabiting `T`
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
    hint_assert!(
      value <= T::SIZE_WORD,
      "from_word_unchecked: Size out of bounds: {:?} >= {:?}",
      value,
      T::SIZE_WORD
    );
    Size(value)
  }

  pub fn from_usize(sz: usize) -> Option<Self> {
    if sz <= T::SIZE {
      Some(Size(T::Word::from_usize(sz).unwrap()))
    } else {
      None
    }
  }

  #[inline]
  pub fn to_word(&self) -> T::Word {
    self.0
  }

  #[inline]
  pub fn to_usize(&self) -> usize {
    self.0.as_()
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
  pub fn next(&self, value: T) -> Option<T> {
    let w = value.into_word() + T::ONE_WORD;
    if w < self.to_word() {
      Some(unsafe { T::from_word_unchecked(w) })
    } else {
      None
    }
  }

  #[inline]
  pub fn prev(&self, value: T) -> Option<T> {
    value.prev()
  }

  #[inline]
  pub fn next_wrapped(&self, value: T) -> T {
    let w = value.into_word() + T::ONE_WORD;
    let q = if w < self.to_word() { w } else { T::ZERO_WORD };
    unsafe { T::from_word_unchecked(q) }
  }

  #[inline]
  pub fn prev_wrapped(&self, value: T) -> T {
    let w = value.into_word();
    let q = if w > T::ZERO_WORD { w } else { self.to_word() } - T::ONE_WORD;
    unsafe { T::from_word_unchecked(q) }
  }

  #[inline]
  pub fn iter(&self) -> EnumoidIter<T> {
    T::word_range(T::ZERO_WORD, self.to_word())
      .map(|w| unsafe { T::from_word_unchecked(w) })
  }

  #[inline]
  pub fn iter_until(&self, until: T) -> EnumoidIter<T> {
    let w = until.into_word();
    if w + T::ONE_WORD < self.to_word() {
      unsafe { Size::from_word_unchecked(w) }.iter()
    } else {
      self.iter()
    }
  }

  #[inline]
  pub fn iter_from(&self, from: T) -> EnumoidIter<T> {
    T::word_range(from.into_word(), self.to_word())
      .map(|w| unsafe { T::from_word_unchecked(w) })
  }

  #[inline]
  pub fn iter_from_until(&self, from: T, until: T) -> EnumoidIter<T> {
    let w = until.into_word();
    if w + T::ONE_WORD < self.to_word() {
      unsafe { Size::from_word_unchecked(w) }.iter_from(from)
    } else {
      self.iter_from(from)
    }
  }
}

/// Trait for enumerable types.
///
/// Some members are hidden. Impls should only be defined via the `Enumoid` derive macro.
pub trait Enumoid: Sized {
  type Word: AsPrimitive<usize> + FromPrimitive + PrimInt + Debug;
  const SIZE: usize;
  fn into_word(self) -> Self::Word;

  #[doc(hidden)]
  type WordRange: Iterator<Item = Self::Word>;
  #[doc(hidden)]
  type FlagsArray: Sized;
  #[doc(hidden)]
  const SIZE_WORD: Self::Word;
  #[doc(hidden)]
  const ZERO_WORD: Self::Word;
  #[doc(hidden)]
  const ONE_WORD: Self::Word;
  #[doc(hidden)]
  const DEFAULT_FLAGS: Self::FlagsArray;
  #[doc(hidden)]
  const FLAGS_BITS: usize;
  #[doc(hidden)]
  const FLAGS_BITS_WORD: Self::Word;
  /// # Safety
  /// The input word must be less than SIZE.
  #[doc(hidden)]
  unsafe fn from_word_unchecked(value: Self::Word) -> Self;
  #[doc(hidden)]
  fn word_range(base: Self::Word, sz: Self::Word) -> Self::WordRange;
  #[doc(hidden)]
  fn slice_flags(arr: &Self::FlagsArray) -> &[u8];
  #[doc(hidden)]
  fn slice_flags_mut(arr: &mut Self::FlagsArray) -> &mut [u8];

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

/// Trait for enumerable types with at least one value.
pub trait Enumoid1: Enumoid {
  const FIRST: Self;
  const LAST: Self;
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
