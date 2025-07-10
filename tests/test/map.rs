use crate::test::types::Three;
use enumoid::EnumMap;

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
fn test_iterator_nth() {
  let mut map = EnumMap::<Three, u16>::new();
  map[Three::A] = 10;
  map[Three::B] = 20;
  map[Three::C] = 30;

  let mut iter = map.iter();

  // Test Iterator::nth
  assert_eq!(
    iter.nth(1),
    Some((Three::B, &20)),
    "Expected nth(1) to return second element"
  );
  assert_eq!(
    iter.len(),
    1,
    "Expected iterator length to be 1 after nth(1)"
  );
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
