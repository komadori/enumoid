use crate::test::types::{
  CompoundOnWideSeven, CompoundSeven, CompoundWideOnSeven, Seventeen, Sixteen,
  StructOne, StructThree, Three, ThreeHundred, WideThree,
};
use enumoid::EnumArrayHelper;
use enumoid::EnumMap;
use enumoid::EnumOptionMap;
use enumoid::EnumSet;
use enumoid::EnumSetHelper;
use enumoid::EnumSize;
use enumoid::EnumVec;

fn align_word(x: usize, align: usize) -> usize {
  ((x + align - 1) / align) * align
}

fn test_type<T: EnumArrayHelper<u8> + EnumSetHelper<u8>>(
  variants: usize,
  value_bytes: usize,
  word_bytes: usize,
  set_bytes: usize,
) {
  assert_eq!(T::SIZE, variants);
  assert_eq!(std::mem::size_of::<T>(), value_bytes);
  assert_eq!(std::mem::size_of::<EnumSize<T>>(), word_bytes);
  assert_eq!(std::mem::size_of::<EnumSet<T>>(), set_bytes);
  assert_eq!(std::mem::size_of::<EnumMap<T, u8>>(), variants);
  assert_eq!(
    std::mem::size_of::<EnumOptionMap<T, u8>>(),
    set_bytes + variants
  );
  assert_eq!(
    std::mem::size_of::<EnumVec<T, u8>>(),
    word_bytes + align_word(variants, word_bytes)
  );
}

#[test]
fn test_three() {
  test_type::<Three>(3, 1, 1, 1);
}

#[test]
fn test_wide_three() {
  test_type::<WideThree>(3, 1, 4, 1);
}

#[test]
fn test_struct() {
  test_type::<StructOne>(1, 0, 1, 1);
  test_type::<StructThree>(3, 1, 1, 1);
}

#[test]
fn test_compound_seven() {
  test_type::<CompoundSeven>(7, 2, 1, 1);
  test_type::<CompoundOnWideSeven>(7, 2, 1, 1);
  test_type::<CompoundWideOnSeven>(7, 2, 4, 1);
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
