use crate::test::types::{Seventeen, Sixteen, Three, ThreeHundred, Zero};
use enumoid::EnumArrayHelper;
use enumoid::EnumFlags;
use enumoid::EnumMap;
use enumoid::EnumOptionMap;
use enumoid::EnumVec;
use enumoid::Size;

fn test_type<T: EnumArrayHelper<u8>>(
  variants: usize,
  value_bytes: usize,
  word_bytes: usize,
  flags_bytes: usize,
) {
  assert_eq!(T::SIZE, variants);
  assert_eq!(std::mem::size_of::<T>(), value_bytes);
  assert_eq!(std::mem::size_of::<Size<T>>(), word_bytes);
  assert_eq!(std::mem::size_of::<EnumFlags<T>>(), flags_bytes);
  assert_eq!(std::mem::size_of::<EnumMap<T, u8>>(), variants);
  assert_eq!(
    std::mem::size_of::<EnumOptionMap<T, u8>>(),
    flags_bytes + variants
  );
  assert_eq!(std::mem::size_of::<EnumVec<T, u8>>(), word_bytes + variants);
}

#[test]
fn test_zero() {
  test_type::<Zero>(0, 0, 1, 0);
}

#[test]
fn test_three() {
  test_type::<Three>(3, 1, 1, 1);
}

#[test]
fn test_sixteen() {
  test_type::<Sixteen>(16, 1, 1, 2);
}

#[test]
fn test_seventeen() {
  test_type::<Seventeen>(17, 1, 1, 3);
}

#[test]
fn test_three_hundred() {
  test_type::<ThreeHundred>(300, 2, 2, 38);
}
