use crate::test::types::{
  CompoundSeven, Seventeen, Sixteen, StructOne, StructThree, Three,
  ThreeHundred, WideThree,
};
use enumoid::{EnumIndex, EnumSize, Enumoid};
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

  // Test iterators.
  //
  // The static `Enumoid::*` iterators behave like the equivalent `EnumSize`
  // methods called on the full size. The `EnumSize` methods additionally clamp
  // their range to the size they are called on. Both `*_until` variants are
  // inclusive of `until`.

  // Enumoid::iter
  assert_eq!(
    T::iter().collect::<Vec<_>>(),
    values.to_vec(),
    "Enumoid::iter()"
  );

  // Enumoid::iter_from
  for (i, &from) in values.iter().enumerate() {
    assert_eq!(
      T::iter_from(from).collect::<Vec<_>>(),
      values[i..].to_vec(),
      "Enumoid::iter_from({from:?})"
    );
  }

  // Enumoid::iter_until
  for (j, &until) in values.iter().enumerate() {
    assert_eq!(
      T::iter_until(until).collect::<Vec<_>>(),
      values[..=j].to_vec(),
      "Enumoid::iter_until({until:?})"
    );
  }

  // Enumoid::iter_from_until
  for (i, &from) in values.iter().enumerate() {
    for (j, &until) in values.iter().enumerate() {
      let expected: Vec<T> = if i <= j {
        values[i..=j].to_vec()
      } else {
        Vec::new()
      };
      assert_eq!(
        T::iter_from_until(from, until).collect::<Vec<_>>(),
        expected,
        "Enumoid::iter_from_until({from:?}, {until:?})"
      );
    }
  }

  // The `EnumSize` methods are exercised across every size from EMPTY (0) up to
  // FULL (SIZE), so that the size-clamping behaviour is covered too. `sizes[s]`
  // is the `EnumSize` representing a size of `s`.
  let sizes: Vec<EnumSize<T>> = (0..=values.len())
    .map(|s| EnumSize::<T>::from_usize(s).unwrap())
    .collect();
  for (s, &size) in sizes.iter().enumerate() {
    // EnumSize::iter
    assert_eq!(
      size.iter().collect::<Vec<_>>(),
      values[..s].to_vec(),
      "EnumSize({s})::iter()"
    );

    // EnumSize::iter_from
    for (i, &from) in values.iter().enumerate() {
      let expected: Vec<T> = if i < s {
        values[i..s].to_vec()
      } else {
        Vec::new()
      };
      assert_eq!(
        size.iter_from(from).collect::<Vec<_>>(),
        expected,
        "EnumSize({s})::iter_from({from:?})"
      );
    }

    // EnumSize::iter_until
    for (j, &until) in values.iter().enumerate() {
      let lim = (j + 1).min(s);
      assert_eq!(
        size.iter_until(until).collect::<Vec<_>>(),
        values[..lim].to_vec(),
        "EnumSize({s})::iter_until({until:?})"
      );
    }

    // EnumSize::iter_from_until
    for (i, &from) in values.iter().enumerate() {
      for (j, &until) in values.iter().enumerate() {
        let lim = (j + 1).min(s);
        let expected: Vec<T> = if i < lim {
          values[i..lim].to_vec()
        } else {
          Vec::new()
        };
        assert_eq!(
          size.iter_from_until(from, until).collect::<Vec<_>>(),
          expected,
          "EnumSize({s})::iter_from_until({from:?}, {until:?})"
        );
      }
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
