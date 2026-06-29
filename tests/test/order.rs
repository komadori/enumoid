use crate::test::types::{
  CompoundOnWideSeven, CompoundSeven, CompoundWideOnSeven, GoldenValues,
  Seventeen, Sixteen, StructOne, StructThree, Three, ThreeHundred, WideThree,
};
use enumoid::{EnumIndex, EnumSize, Enumoid};

// The ordering tests validate the conversion and navigation methods of
// `Enumoid`, `EnumIndex`, and `EnumSize` against a golden, in-order list of
// every value inhabiting a type (`GoldenValues::VALUES`). Each helper below
// checks one area of functionality, and the `order_tests!` macro wires them up
// as `#[test]` methods for a concrete type.

/// `FIRST`, `LAST`, and `SIZE` agree with the golden values.
fn check_constants<T: GoldenValues>() {
  assert_eq!(T::FIRST, *T::VALUES.first().unwrap());
  assert_eq!(T::LAST, *T::VALUES.last().unwrap());
  assert_eq!(T::SIZE, T::VALUES.len());
}

/// Value-level navigation: `next`/`prev` and their wrapping variants.
fn check_value_navigation<T: GoldenValues>() {
  let values = T::VALUES;
  for (i, &x) in values.iter().enumerate() {
    if i == 0 {
      assert_eq!(x.prev(), None, "prev({i})");
      assert_eq!(x.prev_wrapped(), T::LAST, "prev_wrapped({i})");
    } else {
      assert_eq!(x.prev(), Some(values[i - 1]), "prev({i})");
      assert_eq!(x.prev_wrapped(), values[i - 1], "prev_wrapped({i})");
    }
    if i == values.len() - 1 {
      assert_eq!(x.next(), None, "next({i})");
      assert_eq!(x.next_wrapped(), T::FIRST, "next_wrapped({i})");
    } else {
      assert_eq!(x.next(), Some(values[i + 1]), "next({i})");
      assert_eq!(x.next_wrapped(), values[i + 1], "next_wrapped({i})");
    }
  }
}

/// `EnumIndex` conversions and navigation, mirroring the value-level methods
/// but operating on indices.
fn check_index<T: GoldenValues>() {
  let values = T::VALUES;
  for (i, &x) in values.iter().enumerate() {
    let index = EnumIndex::from_value(x);
    assert_eq!(index.into_usize(), i, "into_usize({i})");
    assert_eq!(
      EnumIndex::<T>::from_usize(i),
      Some(index),
      "from_usize({i})"
    );
    // `into_word` is exercised by round-tripping it back through
    // `Enumoid::from_word`, which also covers that method's `Some` branch.
    assert_eq!(T::from_word(index.into_word()), Some(x), "into_word({i})");
    if i == 0 {
      assert_eq!(index.prev(), None, "EnumIndex::prev({i})");
      assert_eq!(
        index.prev_wrapped(),
        EnumIndex::from_value(T::LAST),
        "EnumIndex::prev_wrapped({i})"
      );
    } else {
      assert_eq!(
        index.prev(),
        Some(EnumIndex::from_value(values[i - 1])),
        "EnumIndex::prev({i})"
      );
      assert_eq!(
        index.prev_wrapped(),
        EnumIndex::from_value(values[i - 1]),
        "EnumIndex::prev_wrapped({i})"
      );
    }
    if i == values.len() - 1 {
      assert_eq!(index.next(), None, "EnumIndex::next({i})");
      assert_eq!(
        index.next_wrapped(),
        EnumIndex::from_value(T::FIRST),
        "EnumIndex::next_wrapped({i})"
      );
    } else {
      assert_eq!(
        index.next(),
        Some(EnumIndex::from_value(values[i + 1])),
        "EnumIndex::next({i})"
      );
      assert_eq!(
        index.next_wrapped(),
        EnumIndex::from_value(values[i + 1]),
        "EnumIndex::next_wrapped({i})"
      );
    }
  }
  // `from_usize` returns None for an out-of-range index.
  assert_eq!(
    EnumIndex::<T>::from_usize(T::SIZE),
    None,
    "EnumIndex::from_usize(SIZE)"
  );
}

/// The static `Enumoid::*` iterators. Both `*_until` variants are inclusive of
/// `until`.
fn check_value_iterators<T: GoldenValues>() {
  let values = T::VALUES;
  assert_eq!(
    T::iter().collect::<Vec<_>>(),
    values.to_vec(),
    "Enumoid::iter()"
  );
  for (i, &from) in values.iter().enumerate() {
    assert_eq!(
      T::iter_from(from).collect::<Vec<_>>(),
      values[i..].to_vec(),
      "Enumoid::iter_from({from:?})"
    );
  }
  for (j, &until) in values.iter().enumerate() {
    assert_eq!(
      T::iter_until(until).collect::<Vec<_>>(),
      values[..=j].to_vec(),
      "Enumoid::iter_until({until:?})"
    );
  }
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
}

/// `EnumSize` conversions and navigation, exercised across every size from
/// EMPTY (0) up to FULL (SIZE).
fn check_size_conversions<T: GoldenValues>() {
  let values = T::VALUES;
  let sizes = all_sizes::<T>();
  // `from_usize` returns None for a size larger than the full size.
  assert_eq!(
    EnumSize::<T>::from_usize(T::SIZE + 1),
    None,
    "EnumSize::from_usize(SIZE + 1)"
  );
  for (s, &size) in sizes.iter().enumerate() {
    assert_eq!(size.into_usize(), s, "EnumSize({s})::into_usize()");
    // `from_word` accepts any word up to and including the size, so feeding it
    // `into_word` round-trips. (The rejecting branch needs an out-of-range word
    // and is covered separately in `from_word_out_of_bounds`.)
    assert_eq!(
      EnumSize::from_word(size.into_word()),
      Some(size),
      "EnumSize({s})::from_word(into_word())"
    );

    // into_last / into_last_index.
    if s == 0 {
      assert_eq!(
        size.into_last_index(),
        None,
        "EnumSize(0)::into_last_index()"
      );
      assert_eq!(size.into_last(), None, "EnumSize(0)::into_last()");
    } else {
      assert_eq!(
        size.into_last_index(),
        Some(EnumIndex::from_value(values[s - 1])),
        "EnumSize({s})::into_last_index()"
      );
      assert_eq!(
        size.into_last(),
        Some(values[s - 1]),
        "EnumSize({s})::into_last()"
      );
    }

    // increase / decrease.
    if s < values.len() {
      assert_eq!(
        size.increase(),
        Some(sizes[s + 1]),
        "EnumSize({s})::increase()"
      );
    } else {
      assert_eq!(size.increase(), None, "EnumSize(FULL)::increase()");
    }
    if s > 0 {
      assert_eq!(
        size.decrease(),
        Some(sizes[s - 1]),
        "EnumSize({s})::decrease()"
      );
    } else {
      assert_eq!(size.decrease(), None, "EnumSize(0)::decrease()");
    }

    // contains / contains_index. A size of `s` contains exactly the first `s`
    // values/indices.
    for (i, &v) in values.iter().enumerate() {
      let expected = i < s;
      assert_eq!(size.contains(v), expected, "EnumSize({s})::contains({v:?})");
      assert_eq!(
        size.contains_index(EnumIndex::from_value(v)),
        expected,
        "EnumSize({s})::contains_index({v:?})"
      );
    }
  }
}

/// The `EnumSize::iter*` methods, exercised across every size so the
/// size-clamping behaviour is covered too. Both `*_until` variants are
/// inclusive of `until`.
fn check_size_iterators<T: GoldenValues>() {
  let values = T::VALUES;
  let sizes = all_sizes::<T>();
  for (s, &size) in sizes.iter().enumerate() {
    assert_eq!(
      size.iter().collect::<Vec<_>>(),
      values[..s].to_vec(),
      "EnumSize({s})::iter()"
    );
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
    for (j, &until) in values.iter().enumerate() {
      let lim = (j + 1).min(s);
      assert_eq!(
        size.iter_until(until).collect::<Vec<_>>(),
        values[..lim].to_vec(),
        "EnumSize({s})::iter_until({until:?})"
      );
    }
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

/// Every `EnumSize` from EMPTY (0) up to FULL (SIZE); element `s` is the size
/// `s`. Building this also covers the accepting branch of `EnumSize::from_usize`.
fn all_sizes<T: GoldenValues>() -> Vec<EnumSize<T>> {
  (0..=T::SIZE)
    .map(|s| EnumSize::<T>::from_usize(s).unwrap())
    .collect()
}

/// Defines a `#[test]` for each ordering helper, instantiated for a concrete
/// type. Tests are grouped in a module named `$name`.
macro_rules! order_tests {
  ($name:ident, $t:ty) => {
    mod $name {
      use super::*;

      #[test]
      fn constants() {
        check_constants::<$t>();
      }

      #[test]
      fn value_navigation() {
        check_value_navigation::<$t>();
      }

      #[test]
      fn index() {
        check_index::<$t>();
      }

      #[test]
      fn value_iterators() {
        check_value_iterators::<$t>();
      }

      #[test]
      fn size_conversions() {
        check_size_conversions::<$t>();
      }

      #[test]
      fn size_iterators() {
        check_size_iterators::<$t>();
      }
    }
  };
}

order_tests!(three, Three);
order_tests!(wide_three, WideThree);
order_tests!(struct_one, StructOne);
order_tests!(struct_three, StructThree);
order_tests!(compound_seven, CompoundSeven);
order_tests!(compound_on_wide_seven, CompoundOnWideSeven);
order_tests!(compound_wide_on_seven, CompoundWideOnSeven);
order_tests!(sixteen, Sixteen);
order_tests!(seventeen, Seventeen);

// `ThreeHundred` is deliberately excluded from `order_tests!`: with 300
// variants the combinatorial iterator checks (up to O(n^4)) would be
// infeasibly slow. A size sanity check confirms a large, `u16`-indexed enum is
// handled.
#[test]
fn test_three_hundred() {
  assert_eq!(ThreeHundred::FIRST, ThreeHundred::A1);
  assert_eq!(ThreeHundred::LAST, ThreeHundred::A300);
  assert_eq!(ThreeHundred::SIZE, 300);
}

// The rejecting branches of `EnumSize::from_word` and `Enumoid::from_word`
// require constructing a `Word` beyond the valid range. That is only possible
// against a concrete type whose `Word` is known: `Three` uses a `u8` index and
// `WideThree` uses a `u32` index, exercising two different word types.
#[test]
fn test_from_word_out_of_bounds() {
  // EnumSize::from_word accepts words up to and including the size (3).
  assert_eq!(EnumSize::<Three>::from_word(3), Some(EnumSize::FULL));
  assert_eq!(EnumSize::<Three>::from_word(4), None);
  assert_eq!(EnumSize::<WideThree>::from_word(3), Some(EnumSize::FULL));
  assert_eq!(EnumSize::<WideThree>::from_word(4), None);

  // Enumoid::from_word accepts words strictly below the size (3).
  assert_eq!(Three::from_word(2), Some(Three::C));
  assert_eq!(Three::from_word(3), None);
  assert_eq!(WideThree::from_word(2), Some(WideThree::C));
  assert_eq!(WideThree::from_word(3), None);
}
