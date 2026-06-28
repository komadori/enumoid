use crate::base::EnumSetHelper;
use crate::sub_base::BitsetWordTrait;
use crate::sub_base::RawSizeWord;
use crate::EnumIndex;
use std::borrow::Borrow;
use std::fmt;
use std::fmt::Debug;
use std::hash::Hash;
use std::iter;
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
  pub fn iter_index(&self) -> EnumSetIndexIter<&T::BitsetArray, T, BitsetWord> {
    EnumSetIndexIter::from_storage(&self.data)
  }

  /// Returns an iterator over the members of the set.
  #[inline]
  pub fn iter(&self) -> EnumSetIter<&T::BitsetArray, T, BitsetWord> {
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
    let full_words = T::SIZE / T::BITSET_WORD_BITS;
    let rem = T::SIZE % T::BITSET_WORD_BITS;
    slice[..full_words]
      .iter()
      .all(|&val| val == T::BitsetWord::ALL_SET)
      && (rem == 0 || {
        let last = T::BitsetWord::ALL_SET >> (T::BITSET_WORD_BITS - rem);
        slice[full_words] == last
      })
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

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait>
  iter::FromIterator<T> for EnumSet<T, BitsetWord>
{
  fn from_iter<I: iter::IntoIterator<Item = T>>(iter: I) -> Self {
    let mut set = EnumSet::<T, BitsetWord>::new();
    for key in iter {
      set.insert(key);
    }
    set
  }
}

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> IntoIterator
  for EnumSet<T, BitsetWord>
{
  type Item = T;
  type IntoIter = EnumSetIter<T::BitsetArray, T, BitsetWord>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    EnumSetIter {
      iter: EnumSetIndexIter::new(self),
    }
  }
}

impl<'a, T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait> IntoIterator
  for &'a EnumSet<T, BitsetWord>
{
  type Item = T;
  type IntoIter = EnumSetIter<&'a T::BitsetArray, T, BitsetWord>;

  #[inline]
  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

/// An iterator over the indices of the members of a set.
///
/// The storage `S` borrows the underlying bitset array: `&T::BitsetArray` when
/// iterating over a borrowed set, or the owned `T::BitsetArray` when iterating
/// over a set by value.
pub struct EnumSetIndexIter<
  S,
  T: EnumSetHelper<BitsetWord>,
  BitsetWord: BitsetWordTrait,
> {
  data: S,
  current: T::BitsetWord,
  word_index: usize,
}

impl<
    S: Borrow<T::BitsetArray>,
    T: EnumSetHelper<BitsetWord>,
    BitsetWord: BitsetWordTrait,
  > EnumSetIndexIter<S, T, BitsetWord>
{
  fn from_storage(data: S) -> Self {
    let (current, word_index) =
      match T::slice_bitset(data.borrow()).first().copied() {
        Some(first_word) => (first_word, 0),
        None => (T::BitsetWord::ZERO, 1),
      };
    Self {
      data,
      current,
      word_index,
    }
  }
}

impl<
    S: Borrow<T::BitsetArray>,
    T: EnumSetHelper<BitsetWord>,
    BitsetWord: BitsetWordTrait,
  > Iterator for EnumSetIndexIter<S, T, BitsetWord>
{
  type Item = EnumIndex<T>;

  fn next(&mut self) -> Option<Self::Item> {
    if self.word_index >= T::BITSET_WORDS {
      return None;
    }
    let slice = T::slice_bitset(self.data.borrow());
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
    (
      current_count,
      Some(
        T::SIZE.saturating_sub((self.word_index + 1) * T::BITSET_WORD_BITS)
          + current_count,
      ),
    )
  }
}

impl<
    S: Borrow<T::BitsetArray>,
    T: EnumSetHelper<BitsetWord>,
    BitsetWord: BitsetWordTrait,
  > iter::FusedIterator for EnumSetIndexIter<S, T, BitsetWord>
{
}

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait>
  EnumSetIndexIter<T::BitsetArray, T, BitsetWord>
{
  pub(crate) fn new(set: EnumSet<T, BitsetWord>) -> Self {
    Self::from_storage(set.data)
  }
}

/// An iterator over the members of a set.
///
/// Wraps an [`EnumSetIndexIter`], mapping each index to its member value. The
/// storage `S` selects borrowed (`&T::BitsetArray`) or owned (`T::BitsetArray`)
/// iteration, just as for [`EnumSetIndexIter`].
pub struct EnumSetIter<
  S,
  T: EnumSetHelper<BitsetWord>,
  BitsetWord: BitsetWordTrait,
> {
  iter: EnumSetIndexIter<S, T, BitsetWord>,
}

impl<
    S: Borrow<T::BitsetArray>,
    T: EnumSetHelper<BitsetWord>,
    BitsetWord: BitsetWordTrait,
  > Iterator for EnumSetIter<S, T, BitsetWord>
{
  type Item = T;

  #[inline]
  fn next(&mut self) -> Option<Self::Item> {
    self.iter.next().map(EnumIndex::into_value)
  }

  #[inline]
  fn size_hint(&self) -> (usize, Option<usize>) {
    self.iter.size_hint()
  }
}

impl<
    S: Borrow<T::BitsetArray>,
    T: EnumSetHelper<BitsetWord>,
    BitsetWord: BitsetWordTrait,
  > iter::FusedIterator for EnumSetIter<S, T, BitsetWord>
{
}
