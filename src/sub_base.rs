use std::fmt::Debug;
use std::hash::Hash;
use std::ops::{BitAnd, BitOr, Not, Shl, Shr};

pub trait RawSizeWord: Copy + Debug + Eq + Ord {
  const ZERO: Self;
  fn inc(self) -> Self;
  fn dec(self) -> Self;
  fn as_(self) -> usize;
  fn from_usize_unchecked(x: usize) -> Self;
}

macro_rules! impl_size_word {
  ($t: ty) => {
    impl RawSizeWord for $t {
      const ZERO: Self = 0;
      #[inline(always)]
      fn inc(self) -> Self {
        self + 1
      }
      #[inline(always)]
      fn dec(self) -> Self {
        self - 1
      }
      #[inline(always)]
      fn as_(self) -> usize {
        self as usize
      }
      #[inline(always)]
      fn from_usize_unchecked(x: usize) -> Self {
        x as $t
      }
    }
  };
}

impl_size_word!(u8);
impl_size_word!(u16);
impl_size_word!(u32);
impl_size_word!(usize);

pub trait RawBitsetWord:
  Copy
  + Debug
  + Eq
  + Hash
  + BitAnd<Output = Self>
  + BitOr<Output = Self>
  + Not<Output = Self>
  + Shl<usize, Output = Self>
  + Shr<usize, Output = Self>
{
  const ZERO: Self;
  const ONE: Self;
  const ONES: Self;
  fn count_ones(self) -> usize;
}

macro_rules! impl_raw_bitset_word {
  ($t: ty) => {
    impl RawBitsetWord for $t {
      const ZERO: Self = 0;
      const ONE: Self = 1;
      const ONES: Self = !0;
      fn count_ones(self) -> usize {
        self.count_ones() as usize
      }
    }
  };
}

impl_raw_bitset_word!(u8);
