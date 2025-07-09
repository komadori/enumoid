use crate::base::EnumSetHelper;
use crate::sub_base::BitsetWordTrait;
use crate::sub_base::RawSizeWord;
use crate::EnumIndex;
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

  /// Creates a new set with all members set to true.
  pub fn new_all() -> Self {
    let mut data = T::DEFAULT_BITSET;
    let slice = T::slice_bitset_mut(&mut data);
    for (i, word) in slice.iter_mut().enumerate() {
      if i < T::SIZE / T::BITSET_WORD_BITS {
        *word = T::BitsetWord::ALL_SET;
      } else {
        *word = T::BitsetWord::ALL_SET
          >> (T::BITSET_WORD_BITS - T::SIZE % T::BITSET_WORD_BITS);
      }
    }
    EnumSet { data }
  }

  /// Sets whether a member index is in the set.
  #[inline]
  pub fn set_by_index(&mut self, index: EnumIndex<T>, flag: bool) {
    let i = index.into_word();
    let j = i.as_() / T::BITSET_WORD_BITS;
    let mask = T::BitsetWord::ONE << (i.as_() % T::BITSET_WORD_BITS);
    let set = if flag { mask } else { T::BitsetWord::ZERO };
    let slice = T::slice_bitset_mut(&mut self.data);
    let bits = unsafe { slice.get_unchecked_mut(j) };
    *bits = *bits & !mask | set;
  }

  /// Sets whether a member is in the set.
  #[inline]
  pub fn set(&mut self, key: T, flag: bool) {
    self.set_by_index(key.into(), flag)
  }

  /// Adds a member index to the set and returns true if it was already present.
  #[inline]
  pub fn insert_by_index(&mut self, index: EnumIndex<T>) -> bool {
    let has = self.contains_index(index);
    self.set_by_index(index, true);
    has
  }

  /// Adds a member to the set and returns true if it was already present.
  #[inline]
  pub fn insert(&mut self, key: T) -> bool {
    self.insert_by_index(key.into())
  }

  /// Removes a member index from the set and returns true if it was present.
  #[inline]
  pub fn remove_by_index(&mut self, index: EnumIndex<T>) -> bool {
    let has = self.contains_index(index);
    self.set_by_index(index, false);
    has
  }

  /// Removes a member from the set and returns true if it was present.
  #[inline]
  pub fn remove(&mut self, key: T) -> bool {
    self.remove_by_index(key.into())
  }

  /// Clears all the members from the set.
  pub fn clear(&mut self) {
    self.data = T::DEFAULT_BITSET;
  }

  /// Returns true if a specific member index is in the set.
  #[inline]
  pub fn contains_index(&self, index: EnumIndex<T>) -> bool {
    let i = index.into_usize();
    let j = i / T::BITSET_WORD_BITS;
    let slice = T::slice_bitset(&self.data);
    let bits = unsafe { *slice.get_unchecked(j) };
    (bits >> (i % T::BITSET_WORD_BITS)) & T::BitsetWord::ONE
      != T::BitsetWord::ZERO
  }

  /// Returns true if a specific member is in the set.
  #[inline]
  pub fn contains(&self, key: T) -> bool {
    self.contains_index(key.into())
  }

  /// Returns an iterator over the indices of the members of the set.
  #[inline]
  pub fn iter_index(&self) -> EnumSetIndexIter<T, BitsetWord> {
    EnumSetIndexIter::new(self)
  }

  /// Returns an iterator over the members of the set.
  #[inline]
  pub fn iter(&self) -> EnumSetIter<T, BitsetWord> {
    EnumSetIter {
      iter: self.iter_index(),
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
    fmt.debug_set().entries(self.iter()).finish()
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
    if self.contains(i) {
      TRUE
    } else {
      FALSE
    }
  }
}

pub struct EnumSetIndexIter<
  'a,
  T: EnumSetHelper<BitsetWord>,
  BitsetWord: BitsetWordTrait,
> {
  flags: &'a EnumSet<T, BitsetWord>,
  current: T::BitsetWord,
  word_index: usize,
}

impl<'a, T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait>
  EnumSetIndexIter<'a, T, BitsetWord>
{
  pub fn new(flags: &'a EnumSet<T, BitsetWord>) -> Self {
    let slice = T::slice_bitset(&flags.data);
    let current = slice.first().copied().unwrap_or(T::BitsetWord::ZERO);
    Self {
      flags,
      current,
      word_index: 0,
    }
  }
}

impl<'a, T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> Iterator
  for EnumSetIndexIter<'a, T, BitsetWord>
{
  type Item = EnumIndex<T>;

  fn next(&mut self) -> Option<Self::Item> {
    let slice = T::slice_bitset(&self.flags.data);
    while self.current == T::BitsetWord::ZERO {
      self.word_index += 1;
      self.current = slice.get(self.word_index).copied()?;
    }
    let index =
      self.word_index * T::BITSET_WORD_BITS + self.current.trailing_zeros();
    self.current = self.current & (self.current - T::BitsetWord::ONE);
    EnumIndex::from_usize(index)
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    let current_count = self.current.count_ones();
    let eff_word_index =
      self.word_index + if current_count > 0 { 1 } else { 0 };
    (
      current_count,
      Some(
        T::SIZE.saturating_sub(eff_word_index * T::BITSET_WORD_BITS)
          + current_count,
      ),
    )
  }
}

pub struct EnumSetIter<
  'a,
  T: EnumSetHelper<BitsetWord>,
  BitsetWord: BitsetWordTrait,
> {
  iter: EnumSetIndexIter<'a, T, BitsetWord>,
}

impl<'a, T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> Iterator
  for EnumSetIter<'a, T, BitsetWord>
{
  type Item = T;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(|index| index.into_value())
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}
