use crate::test::drop_tracker::DropTracker;
use crate::test::types::Three;
use enumoid::EnumMap;
use enumoid::EnumOptionMap;
use std::cell::Cell;

#[test]
fn test_empty_state() {
  let map = EnumMap::<Three, u16>::new();

  // Test that all values are default initialized
  assert_eq!(*map.get(Three::A), 0, "Expected default value for Three::A");
  assert_eq!(*map.get(Three::B), 0, "Expected default value for Three::B");
  assert_eq!(*map.get(Three::C), 0, "Expected default value for Three::C");

  // Test iteration shows all default values
  let collected: Vec<_> = map.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &0), (Three::B, &0), (Three::C, &0)],
    "Expected iteration to yield all key-value pairs with default values"
  );
}

#[test]
fn test_index_assignment() {
  let mut map = EnumMap::<Three, u16>::new();

  map[Three::B] = 200;
  assert_eq!(
    *map.get(Three::B),
    200,
    "Expected value after index assignment"
  );
  assert_eq!(*map.get(Three::A), 0, "Expected unchanged default value");
  assert_eq!(*map.get(Three::C), 0, "Expected unchanged default value");
}

#[test]
fn test_mutable_get() {
  let mut map = EnumMap::<Three, u16>::new();

  *map.get_mut(Three::C) += 1;
  assert_eq!(
    *map.get(Three::C),
    1,
    "Expected value after mutable reference modification"
  );
  assert_eq!(*map.get(Three::A), 0, "Expected unchanged default value");
  assert_eq!(*map.get(Three::B), 0, "Expected unchanged default value");
}

#[test]
fn test_iteration_all_elements() {
  let mut map = EnumMap::<Three, u16>::new();
  map[Three::A] = 10;
  map[Three::B] = 20;
  map[Three::C] = 30;

  let collected: Vec<_> = map.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &10), (Three::B, &20), (Three::C, &30)],
    "Expected iteration to yield all key-value pairs in order"
  );
}

#[test]
fn test_mutable_iteration() {
  let mut map = EnumMap::<Three, i32>::new();
  map[Three::A] = 10;
  map[Three::B] = 20;
  map[Three::C] = 30;

  // Test mutable iteration
  for (_, value) in map.iter_mut() {
    *value *= 2;
  }

  let collected: Vec<_> = map.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &20), (Three::B, &40), (Three::C, &60)],
    "Expected values to be modified through iter_mut()"
  );
}

#[test]
fn test_iterator_exact_size() {
  let mut map = EnumMap::<Three, u16>::new();
  map[Three::A] = 10;
  map[Three::B] = 20;
  map[Three::C] = 30;

  let iter = map.iter();

  // Test ExactSizeIterator
  assert_eq!(iter.len(), 3, "Expected iterator length to be 3");

  // Test size_hint
  assert_eq!(iter.size_hint(), (3, Some(3)), "Expected correct size hint");

  // Test iterator count
  assert_eq!(iter.count(), 3, "Expected iterator count to be 3");
}

#[test]
fn test_iterator_double_ended() {
  let mut map = EnumMap::<Three, u16>::new();
  map[Three::A] = 10;
  map[Three::B] = 20;
  map[Three::C] = 30;

  // Test DoubleEndedIterator
  let mut iter = map.iter();
  assert_eq!(
    iter.next_back(),
    Some((Three::C, &30)),
    "Expected next_back() to return last element"
  );
  assert_eq!(
    iter.next_back(),
    Some((Three::B, &20)),
    "Expected next_back() to return second-to-last element"
  );
  assert_eq!(
    iter.next(),
    Some((Three::A, &10)),
    "Expected next() to return first element"
  );
  assert_eq!(iter.next(), None, "Expected iterator to be exhausted");
}

#[test]
fn test_into_iterator() {
  let mut map = EnumMap::<Three, u16>::new();
  map[Three::A] = 10;
  map[Three::B] = 20;
  map[Three::C] = 30;

  // Test IntoIterator for &EnumMap
  let collected: Vec<_> = (&map).into_iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &10), (Three::B, &20), (Three::C, &30)],
    "Expected IntoIterator to work for &EnumMap"
  );

  // Test IntoIterator for &mut EnumMap
  let collected_mut: Vec<_> =
    (&mut map).into_iter().map(|(k, v)| (k, *v)).collect();
  assert_eq!(
    collected_mut,
    vec![(Three::A, 10), (Three::B, 20), (Three::C, 30)],
    "Expected IntoIterator to work for &mut EnumMap"
  );

  // Test for loop syntax
  let mut sum = 0;
  for (_, value) in &map {
    sum += *value;
  }
  assert_eq!(sum, 60, "Expected for loop to sum all values");
}

#[test]
fn test_set_methods() {
  let mut map = EnumMap::<Three, i32>::new();

  // Test set method returns old value
  let old_value = map.set(Three::A, 42);
  assert_eq!(old_value, 0, "Expected set() to return old default value");
  assert_eq!(*map.get(Three::A), 42, "Expected new value after set()");

  // Test set method with existing value
  let old_value = map.set(Three::A, 100);
  assert_eq!(old_value, 42, "Expected set() to return previous value");
  assert_eq!(
    *map.get(Three::A),
    100,
    "Expected updated value after second set()"
  );

  // Test set_by_index method
  let old_value = map.set_by_index(Three::B.into(), 200);
  assert_eq!(
    old_value, 0,
    "Expected set_by_index() to return old default value"
  );
  assert_eq!(
    *map.get(Three::B),
    200,
    "Expected new value after set_by_index()"
  );
}

#[test]
fn test_new_with_constructor() {
  let map = EnumMap::<Three, i32>::new_with(|variant| match variant {
    Three::A => 10,
    Three::B => 20,
    Three::C => 30,
  });

  assert_eq!(*map.get(Three::A), 10, "Expected custom value for Three::A");
  assert_eq!(*map.get(Three::B), 20, "Expected custom value for Three::B");
  assert_eq!(*map.get(Three::C), 30, "Expected custom value for Three::C");
}

#[test]
fn test_swap_methods() {
  let mut map = EnumMap::<Three, i32>::new_with(|variant| match variant {
    Three::A => 1,
    Three::B => 2,
    Three::C => 3,
  });

  // Test swap method
  map.swap(Three::A, Three::C);
  assert_eq!(*map.get(Three::A), 3, "Expected swapped value for Three::A");
  assert_eq!(*map.get(Three::C), 1, "Expected swapped value for Three::C");
  assert_eq!(
    *map.get(Three::B),
    2,
    "Expected unchanged value for Three::B"
  );

  // Test swap_by_index method
  map.swap_by_index(Three::A.into(), Three::B.into());
  assert_eq!(
    *map.get(Three::A),
    2,
    "Expected swapped value for Three::A after swap_by_index"
  );
  assert_eq!(
    *map.get(Three::B),
    3,
    "Expected swapped value for Three::B after swap_by_index"
  );
}

#[test]
fn test_slice_access() {
  let mut map = EnumMap::<Three, i32>::new_with(|variant| match variant {
    Three::A => 10,
    Three::B => 20,
    Three::C => 30,
  });

  // Test as_slice
  let slice = map.as_slice();
  assert_eq!(
    slice,
    &[10, 20, 30],
    "Expected slice to contain all values in order"
  );

  // Test as_slice_mut
  let slice_mut = map.as_slice_mut();
  slice_mut[1] = 99;
  assert_eq!(
    *map.get(Three::B),
    99,
    "Expected value change through mutable slice"
  );
}

#[test]
fn test_get_by_index() {
  let map = EnumMap::<Three, i32>::new_with(|variant| match variant {
    Three::A => 100,
    Three::B => 200,
    Three::C => 300,
  });

  assert_eq!(
    *map.get_by_index(Three::A.into()),
    100,
    "Expected correct value for index A"
  );
  assert_eq!(
    *map.get_by_index(Three::B.into()),
    200,
    "Expected correct value for index B"
  );
  assert_eq!(
    *map.get_by_index(Three::C.into()),
    300,
    "Expected correct value for index C"
  );
}

#[test]
fn test_get_by_index_mut() {
  let mut map = EnumMap::<Three, i32>::new_with(|variant| match variant {
    Three::A => 100,
    Three::B => 200,
    Three::C => 300,
  });

  // Test mutable access by index
  *map.get_by_index_mut(Three::B.into()) += 50;
  assert_eq!(
    *map.get_by_index(Three::B.into()),
    250,
    "Expected value to be modified through get_by_index_mut"
  );

  // Test that other values remain unchanged
  assert_eq!(
    *map.get_by_index(Three::A.into()),
    100,
    "Expected unchanged value for index A"
  );
  assert_eq!(
    *map.get_by_index(Three::C.into()),
    300,
    "Expected unchanged value for index C"
  );
}

#[test]
fn test_try_from_full_option_map() {
  let mut opt = EnumOptionMap::<Three, i32>::new();
  opt.insert(Three::A, 10);
  opt.insert(Three::B, 20);
  opt.insert(Three::C, 30);

  let map =
    EnumMap::try_from(opt).expect("Expected Ok when option map is full");
  assert_eq!(*map.get(Three::A), 10, "Expected converted value for A");
  assert_eq!(*map.get(Three::B), 20, "Expected converted value for B");
  assert_eq!(*map.get(Three::C), 30, "Expected converted value for C");
}

#[test]
fn test_try_from_partial_option_map_fails() {
  let mut opt = EnumOptionMap::<Three, i32>::new();
  opt.insert(Three::A, 10);
  opt.insert(Three::C, 30); // Missing Three::B, so not full

  assert_eq!(
    EnumMap::try_from(opt),
    Err(()),
    "Expected Err when option map is not full"
  );
}

#[test]
fn test_from_iterator() {
  // Collecting key-value pairs builds a total map.
  let map: EnumMap<Three, i32> =
    [(Three::A, 10), (Three::B, 20), (Three::C, 30)]
      .into_iter()
      .collect();

  assert_eq!(*map.get(Three::A), 10, "Expected collected value for A");
  assert_eq!(*map.get(Three::B), 20, "Expected collected value for B");
  assert_eq!(*map.get(Three::C), 30, "Expected collected value for C");
}

#[test]
fn test_from_iterator_partial_uses_defaults() {
  // Keys absent from the iterator fall back to the default value.
  let map: EnumMap<Three, i32> =
    [(Three::A, 10), (Three::C, 30)].into_iter().collect();

  assert_eq!(*map.get(Three::A), 10, "Expected collected value for A");
  assert_eq!(
    *map.get(Three::B),
    0,
    "Expected default value for absent key B"
  );
  assert_eq!(*map.get(Three::C), 30, "Expected collected value for C");
}

#[test]
fn test_from_iterator_duplicate_keys_take_last() {
  // Later pairs for the same key override earlier ones.
  let map: EnumMap<Three, i32> =
    [(Three::A, 10), (Three::A, 99)].into_iter().collect();

  assert_eq!(
    *map.get(Three::A),
    99,
    "Expected the last value for a duplicated key"
  );
}

#[test]
fn test_from_iterator_empty() {
  // An empty iterator yields a fully default map.
  let map: EnumMap<Three, i32> = std::iter::empty().collect();

  assert_eq!(*map.get(Three::A), 0, "Expected default value for A");
  assert_eq!(*map.get(Three::B), 0, "Expected default value for B");
  assert_eq!(*map.get(Three::C), 0, "Expected default value for C");
}

#[test]
fn test_into_iterator_owned() {
  // The consuming IntoIterator yields owned (key, value) pairs, mirroring
  // FromIterator<(T, V)>.
  let map: EnumMap<Three, i32> =
    [(Three::A, 10), (Three::B, 20), (Three::C, 30)]
      .into_iter()
      .collect();
  let collected: Vec<(Three, i32)> = map.into_iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, 10), (Three::B, 20), (Three::C, 30)],
    "Expected consuming iteration to yield all key-value pairs in order"
  );
}

#[test]
fn test_into_iterator_owned_roundtrips() {
  let map = EnumMap::<Three, i32>::new_with(|k| match k {
    Three::A => 10,
    Three::B => 20,
    Three::C => 30,
  });
  let roundtripped: EnumMap<Three, i32> = map.into_iter().collect();
  let expected = EnumMap::<Three, i32>::new_with(|k| match k {
    Three::A => 10,
    Three::B => 20,
    Three::C => 30,
  });
  assert_eq!(
    roundtripped, expected,
    "Expected into_iter().collect() to round-trip"
  );
}

#[test]
fn test_into_iterator_owned_exact_size() {
  let map = EnumMap::<Three, i32>::new();
  let mut iter = map.into_iter();
  assert_eq!(iter.len(), 3, "Expected ExactSizeIterator len of 3");
  assert_eq!(iter.size_hint(), (3, Some(3)), "Expected exact size hint");
  iter.next();
  assert_eq!(iter.len(), 2, "Expected len to decrease after next()");
}

#[test]
fn test_into_iterator_owned_drops_all_when_consumed() {
  let drops = Cell::new(0);
  let map = EnumMap::<Three, DropTracker>::new_with(|k| match k {
    Three::A => DropTracker::new(1, &drops),
    Three::B => DropTracker::new(2, &drops),
    Three::C => DropTracker::new(3, &drops),
  });

  let ids: Vec<i32> = map.into_iter().map(|(_, v)| v.id()).collect();
  assert_eq!(ids, vec![1, 2, 3], "Expected ids in key order");
  assert_eq!(
    drops.get(),
    3,
    "Expected each yielded value to be dropped exactly once"
  );
}

#[test]
fn test_into_iterator_owned_drops_remainder_when_abandoned() {
  // Abandoning a partially-consumed iterator drops every unyielded value
  // exactly once.
  let drops = Cell::new(0);
  let map = EnumMap::<Three, DropTracker>::new_with(|k| match k {
    Three::A => DropTracker::new(1, &drops),
    Three::B => DropTracker::new(2, &drops),
    Three::C => DropTracker::new(3, &drops),
  });

  let mut iter = map.into_iter();
  {
    let (key, value) = iter.next().expect("Expected a first pair");
    assert_eq!(key, Three::A, "Expected first key");
    assert_eq!(value.id(), 1, "Expected first value id");
    assert_eq!(drops.get(), 0, "Expected no drops while value is held");
  }
  assert_eq!(drops.get(), 1, "Expected the moved-out value to drop");

  drop(iter);
  assert_eq!(
    drops.get(),
    3,
    "Expected the two unyielded values to drop when abandoned"
  );
}

#[test]
fn test_into_iterator_owned_double_ended() {
  // next() and next_back() meet in the middle, each pair yielded once.
  let map: EnumMap<Three, i32> =
    [(Three::A, 10), (Three::B, 20), (Three::C, 30)]
      .into_iter()
      .collect();
  let mut iter = map.into_iter();
  assert_eq!(
    iter.next_back(),
    Some((Three::C, 30)),
    "Expected next_back() to yield the last pair"
  );
  assert_eq!(
    iter.next(),
    Some((Three::A, 10)),
    "Expected next() to yield the first pair"
  );
  assert_eq!(
    iter.next_back(),
    Some((Three::B, 20)),
    "Expected next_back() to yield the remaining middle pair"
  );
  assert_eq!(iter.next(), None, "Expected exhaustion from the front");
  assert_eq!(iter.next_back(), None, "Expected exhaustion from the back");
}

#[test]
fn test_into_iterator_owned_rev() {
  // rev() relies on DoubleEndedIterator and yields keys in reverse order.
  let map: EnumMap<Three, i32> =
    [(Three::A, 10), (Three::B, 20), (Three::C, 30)]
      .into_iter()
      .collect();
  let collected: Vec<(Three, i32)> = map.into_iter().rev().collect();
  assert_eq!(
    collected,
    vec![(Three::C, 30), (Three::B, 20), (Three::A, 10)],
    "Expected reversed iteration to yield pairs in reverse key order"
  );
}

#[test]
fn test_into_iterator_owned_double_ended_exact_size() {
  // len() stays accurate as the range is consumed from both ends.
  let map = EnumMap::<Three, i32>::new();
  let mut iter = map.into_iter();
  assert_eq!(iter.len(), 3, "Expected initial len of 3");
  iter.next();
  assert_eq!(iter.len(), 2, "Expected len 2 after next()");
  iter.next_back();
  assert_eq!(iter.len(), 1, "Expected len 1 after next_back()");
  iter.next();
  assert_eq!(iter.len(), 0, "Expected len 0 once the ends meet");
}

#[test]
fn test_into_iterator_owned_fused() {
  // Once exhausted, the iterator keeps returning None from both ends.
  let map = EnumMap::<Three, i32>::new();
  let mut iter = map.into_iter();
  for _ in 0..3 {
    assert!(
      iter.next().is_some(),
      "Expected three pairs before exhaustion"
    );
  }
  assert_eq!(iter.next(), None, "Expected None once exhausted");
  assert_eq!(iter.next(), None, "Expected fused None on repeat next()");
  assert_eq!(
    iter.next_back(),
    None,
    "Expected fused None from the back once exhausted"
  );
}

#[test]
fn test_into_iterator_owned_double_ended_drops_remainder() {
  // After taking one pair from each end, abandoning the iterator drops only
  // the unyielded middle value, exactly once, and never the yielded ends.
  let drops = Cell::new(0);
  let map = EnumMap::<Three, DropTracker>::new_with(|k| match k {
    Three::A => DropTracker::new(1, &drops),
    Three::B => DropTracker::new(2, &drops),
    Three::C => DropTracker::new(3, &drops),
  });

  let mut iter = map.into_iter();
  {
    let (front_key, front_value) = iter.next().expect("Expected a front pair");
    let (back_key, back_value) =
      iter.next_back().expect("Expected a back pair");
    assert_eq!(front_key, Three::A, "Expected front key A");
    assert_eq!(front_value.id(), 1, "Expected front value id");
    assert_eq!(back_key, Three::C, "Expected back key C");
    assert_eq!(back_value.id(), 3, "Expected back value id");
    assert_eq!(drops.get(), 0, "Expected no drops while values are held");
  }
  assert_eq!(drops.get(), 2, "Expected the two moved-out values to drop");

  drop(iter);
  assert_eq!(
    drops.get(),
    3,
    "Expected the single unyielded middle value to drop exactly once"
  );
}

#[test]
fn test_iter_mut_double_ended() {
  // EnumSliceIterMut is double-ended; mutations through next_back must land.
  let mut map = EnumMap::<Three, u16>::new();
  map[Three::A] = 10;
  map[Three::B] = 20;
  map[Three::C] = 30;

  {
    let mut iter = map.iter_mut();
    let (back_key, back_value) =
      iter.next_back().expect("Expected a back element");
    assert_eq!(back_key, Three::C, "Expected next_back() key to be C");
    *back_value += 1;
    let (front_key, front_value) =
      iter.next().expect("Expected a front element");
    assert_eq!(front_key, Three::A, "Expected next() key to be A");
    *front_value += 1;
    assert_eq!(
      iter.next_back().map(|(k, v)| (k, *v)),
      Some((Three::B, 20)),
      "Expected next_back() to yield the middle element"
    );
    assert!(iter.next().is_none(), "Expected the ends to have met");
    assert!(
      iter.next_back().is_none(),
      "Expected fused None from the back"
    );
  }

  assert_eq!(map[Three::A], 11, "Expected front mutation to persist");
  assert_eq!(map[Three::C], 31, "Expected back mutation to persist");
}
