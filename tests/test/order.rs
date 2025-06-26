use crate::test::types::{
  CompoundSeven, Seventeen, Sixteen, StructOne, StructThree, Three,
  ThreeHundred, WideThree,
};
use enumoid::{EnumIndex, Enumoid};
use std::{fmt::Debug, iter::zip};

use super::types::{CompoundOnWideSeven, CompoundWideOnSeven};

fn test_type<T: Enumoid + Copy + Debug + PartialEq>(values: &[T]) {
  assert_eq!(T::FIRST, *values.first().unwrap());
  assert_eq!(T::LAST, *values.last().unwrap());
  assert_eq!(T::SIZE, values.len());
  let test: Vec<T> = values.to_vec();
  assert_eq!(T::SIZE, test.len());
  for (i, (x, y)) in zip(test, values.iter().copied()).enumerate() {
    assert_eq!(x, y);
    assert_eq!(EnumIndex::from_value(x).into_usize(), i);
    if i == 0 {
      assert_eq!(x.prev(), None);
      assert_eq!(x.prev_wrapped(), T::LAST);
    } else {
      assert_eq!(x.prev(), Some(values[i - 1]));
      assert_eq!(x.prev_wrapped(), values[i - 1]);
    }
    if i == values.len() - 1 {
      assert_eq!(x.next(), None);
      assert_eq!(x.next_wrapped(), T::FIRST);
    } else {
      assert_eq!(x.next(), Some(values[i + 1]));
      assert_eq!(x.next_wrapped(), values[i + 1]);
    }
  }
}

#[test]
fn test_three() {
  test_type::<Three>(&[Three::A, Three::B, Three::C]);
  test_type::<WideThree>(&[WideThree::A, WideThree::B, WideThree::C]);
}

#[test]
fn test_struct() {
  test_type::<StructOne>(&[StructOne]);
  test_type::<StructThree>(&[
    StructThree(Three::A),
    StructThree(Three::B),
    StructThree(Three::C),
  ]);
}

#[test]
fn test_compound_seven() {
  test_type::<CompoundSeven>(&[
    CompoundSeven::X(Three::A),
    CompoundSeven::X(Three::B),
    CompoundSeven::X(Three::C),
    CompoundSeven::Y,
    CompoundSeven::Z(Three::A),
    CompoundSeven::Z(Three::B),
    CompoundSeven::Z(Three::C),
  ]);
  test_type::<CompoundOnWideSeven>(&[
    CompoundOnWideSeven::X(WideThree::A),
    CompoundOnWideSeven::X(WideThree::B),
    CompoundOnWideSeven::X(WideThree::C),
    CompoundOnWideSeven::Y,
    CompoundOnWideSeven::Z(WideThree::A),
    CompoundOnWideSeven::Z(WideThree::B),
    CompoundOnWideSeven::Z(WideThree::C),
  ]);
  test_type::<CompoundWideOnSeven>(&[
    CompoundWideOnSeven::X(Three::A),
    CompoundWideOnSeven::X(Three::B),
    CompoundWideOnSeven::X(Three::C),
    CompoundWideOnSeven::Y,
    CompoundWideOnSeven::Z(Three::A),
    CompoundWideOnSeven::Z(Three::B),
    CompoundWideOnSeven::Z(Three::C),
  ]);
}

#[test]
fn test_sixteen() {
  test_type::<Sixteen>(&[
    Sixteen::A,
    Sixteen::B,
    Sixteen::C,
    Sixteen::D,
    Sixteen::E,
    Sixteen::F,
    Sixteen::G,
    Sixteen::H,
    Sixteen::I,
    Sixteen::J,
    Sixteen::K,
    Sixteen::L,
    Sixteen::M,
    Sixteen::N,
    Sixteen::O,
    Sixteen::P,
  ]);
}

#[test]
fn test_seventeen() {
  test_type::<Seventeen>(&[
    Seventeen::A,
    Seventeen::B,
    Seventeen::C,
    Seventeen::D,
    Seventeen::E,
    Seventeen::F,
    Seventeen::G,
    Seventeen::H,
    Seventeen::I,
    Seventeen::J,
    Seventeen::K,
    Seventeen::L,
    Seventeen::M,
    Seventeen::N,
    Seventeen::O,
    Seventeen::P,
    Seventeen::Q,
  ]);
}

#[test]
fn test_three_hundred() {
  assert_eq!(ThreeHundred::SIZE, 300);
}
