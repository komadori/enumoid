use crate::base::EnumSetHelper;
use crate::sub_base::BitsetWordTrait;
use crate::sub_base::RawSizeWord;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Index;

/// A set of enumoid `T`'s members.
///
/// The optional type parameter `BitsetWord` specifies the size of the words used to store the
/// bitset. Traits are defined for both `u8` and `usize`.
#[derive(Copy, Clone)]
pub struct EnumSet<
  T: EnumSetHelper<BitsetWord>,
  BitsetWord: BitsetWordTrait = u8,
> {
  data: T::BitsetArray,
}

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait>
  EnumSet<T, BitsetWord>
{
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
  pub fn iter(&self) -> EnumSetIter<T, BitsetWord> {
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
    let last = T::BitsetWord::ALL_SET
      >> (T::BITSET_WORD_BITS - T::SIZE % T::BITSET_WORD_BITS);
    slice[..T::SIZE / T::BITSET_WORD_BITS]
      .iter()
      .all(|&val| val == T::BitsetWord::ALL_SET)
      && (T::SIZE % T::BITSET_WORD_BITS == 0
        || slice[T::SIZE / T::BITSET_WORD_BITS] == last)
  }
}

impl<T: EnumSetHelper<BitsetWord> + Debug, BitsetWord: BitsetWordTrait> Debug
  for EnumSet<T, BitsetWord>
{
  fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
    fmt.debug_map().entries(self.iter()).finish()
  }
}

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> Default
  for EnumSet<T, BitsetWord>
{
  fn default() -> Self {
    EnumSet::<T, BitsetWord>::new()
  }
}

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> PartialEq
  for EnumSet<T, BitsetWord>
{
  fn eq(&self, other: &Self) -> bool {
    T::slice_bitset(&self.data) == T::slice_bitset(&other.data)
  }
}

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> Eq
  for EnumSet<T, BitsetWord>
{
}

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> Hash
  for EnumSet<T, BitsetWord>
{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    T::slice_bitset(&self.data).hash(state);
  }
}

const TRUE: &bool = &true;
const FALSE: &bool = &false;

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> Index<T>
  for EnumSet<T, BitsetWord>
{
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

pub struct EnumSetIter<
  'a,
  T: EnumSetHelper<BitsetWord>,
  BitsetWord: BitsetWordTrait,
> {
  flags: &'a EnumSet<T, BitsetWord>,
  iter: T::WordRange,
}

impl<'a, T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> Iterator
  for EnumSetIter<'a, T, BitsetWord>
{
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

impl<'a, T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait>
  ExactSizeIterator for EnumSetIter<'a, T, BitsetWord>
{
}
