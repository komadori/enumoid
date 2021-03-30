use std::iter::Map;
use std::mem;
use std::ops::Range;
use std::ptr;

pub trait Enumoid: Sized {
  type CompactSize: Copy;
  const SIZE: usize;
  //const LAST: Self;
  fn into_usize(value: Self) -> usize;
  fn from_usize(value: usize) -> Self;
  fn compact_size(value: usize) -> Self::CompactSize;
  fn uncompact_size(value: Self::CompactSize) -> usize;

  #[inline]
  fn wrapped_add(value: Self, last: Self, delta: isize) -> Self {
    let v = Self::into_usize(value) as isize;
    let s = Self::into_usize(last) as isize + 1;
    let y = (v + delta + s) % s;
    // Slow path if delta is less than -s
    let yy = if y < 0 { (y + s) % s } else { y };
    Self::from_usize(yy as usize)
  }

  #[inline]
  fn checked_add(value: Self, last: Self, delta: isize) -> Option<Self> {
    let v = Self::into_usize(value) as isize;
    let s = Self::into_usize(last) as isize;
    let y = v + delta;
    if y >= 0 && y <= s {
      Some(Self::from_usize(y as usize))
    } else {
      None
    }
  }

  #[inline]
  fn iter() -> Map<Range<usize>, fn(usize) -> Self> {
    (0..Self::SIZE).map(Self::from_usize)
  }

  #[inline]
  fn range_inclusive(self, to: Self) -> Map<Range<usize>, fn(usize) -> Self> {
    (Self::into_usize(self)..Self::into_usize(to) + 1).map(Self::from_usize)
  }
}

pub trait EnumFlagsHelper: Enumoid {
  type FlagsArray: Sized + Default;
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
  unsafe fn partial_to_total(p: Self::PartialArray) -> Self::TotalArray;

  fn total_slice(t: &Self::TotalArray) -> &[V];
  fn total_slice_mut(t: &mut Self::TotalArray) -> &mut [V];
  fn total_to_partial(t: Self::TotalArray) -> Self::PartialArray;

  #[inline]
  fn new_partial() -> Self::PartialArray {
    unsafe { mem::MaybeUninit::uninit().assume_init() }
  }
}

pub unsafe fn unconstrained_transmute<A, B>(a: A) -> B {
  let result = ptr::read(&a as *const A as *const B);
  mem::forget(a);
  result
}
