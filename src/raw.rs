use std::fmt::Debug;

pub trait RawIndex: Copy + Debug + Eq + Ord {
  const ZERO: Self;
  fn inc(self) -> Self;
  fn dec(self) -> Self;
  fn as_(self) -> usize;
  fn from_usize_unchecked(x: usize) -> Self;
}

impl RawIndex for () {
  const ZERO: Self = ();
  #[inline(always)]
  fn inc(self) -> Self {}
  #[inline(always)]
  fn dec(self) -> Self {}
  #[inline(always)]
  fn as_(self) -> usize {
    0
  }
  #[inline(always)]
  fn from_usize_unchecked(_x: usize) -> Self {}
}

macro_rules! impl_raw_index {
  ($t: ty) => {
    impl RawIndex for $t {
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

impl_raw_index!(u8);
impl_raw_index!(u16);
impl_raw_index!(u32);
impl_raw_index!(usize);
