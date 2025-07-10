use crate::test::types::Three;
use enumoid::EnumOptionMap;
use enumoid::EnumSize;

#[test]
fn test_empty_state() {
  let map = EnumOptionMap::<Three, u16>::new();

  assert!(map.is_empty(), "Expected new map to be empty");
  assert!(!map.is_full(), "Expected new map to not be full");
  assert_eq!(map.count(), 0, "Expected new map to have count of 0");
  assert_eq!(
    map.is_vec(),
    EnumSize::from_usize(0),
    "Expected empty map to be representable as 0-length vector"
  );

  // Test that all keys return None
  assert_eq!(
    map.get(Three::A),
    None,
    "Expected None for Three::A in empty map"
  );
  assert_eq!(
    map.get(Three::B),
    None,
    "Expected None for Three::B in empty map"
  );
  assert_eq!(
    map.get(Three::C),
    None,
    "Expected None for Three::C in empty map"
  );

  // Test contains
  assert!(
    !map.contains(Three::A),
    "Expected empty map to not contain Three::A"
  );
  assert!(
    !map.contains(Three::B),
    "Expected empty map to not contain Three::B"
  );
  assert!(
    !map.contains(Three::C),
    "Expected empty map to not contain Three::C"
  );
}

#[test]
fn test_set_and_get() {
  let mut map = EnumOptionMap::<Three, u16>::new();

  // Test setting a value
  let old_value = map.set(Three::B, Some(200));
  assert_eq!(
    old_value, None,
    "Expected set() to return None for previously empty key"
  );
  assert!(
    !map.is_empty(),
    "Expected map to not be empty after setting a value"
  );
  assert_eq!(
    map.count(),
    1,
    "Expected map to have count of 1 after setting one value"
  );

  // Test getting the value
  assert_eq!(
    map.get(Three::B),
    Some(&200),
    "Expected get() to return the set value"
  );
  assert_eq!(
    map.get(Three::A),
    None,
    "Expected get() to return None for unset key"
  );
  assert_eq!(
    map.get(Three::C),
    None,
    "Expected get() to return None for unset key"
  );

  // Test overwriting a value
  let old_value = map.set(Three::B, Some(300));
  assert_eq!(
    old_value,
    Some(200),
    "Expected set() to return previous value when overwriting"
  );
  assert_eq!(
    map.get(Three::B),
    Some(&300),
    "Expected get() to return the new value"
  );
  assert_eq!(
    map.count(),
    1,
    "Expected count to remain 1 after overwriting"
  );

  // Test setting to None (removal)
  let old_value = map.set(Three::B, None);
  assert_eq!(
    old_value,
    Some(300),
    "Expected set(None) to return previous value"
  );
  assert_eq!(
    map.get(Three::B),
    None,
    "Expected get() to return None after setting to None"
  );
  assert!(
    map.is_empty(),
    "Expected map to be empty after setting only value to None"
  );
  assert_eq!(
    map.count(),
    0,
    "Expected count to be 0 after removing only value"
  );
}

#[test]
fn test_insert_and_remove() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test insert
  let old_value = map.insert(Three::A, 100);
  assert_eq!(
    old_value, None,
    "Expected insert() to return None for new key"
  );
  assert_eq!(
    map.get(Three::A),
    Some(&100),
    "Expected get() to return inserted value"
  );
  assert_eq!(map.count(), 1, "Expected count to be 1 after insert");

  // Test insert with overwrite
  let old_value = map.insert(Three::A, 200);
  assert_eq!(
    old_value,
    Some(100),
    "Expected insert() to return previous value when overwriting"
  );
  assert_eq!(
    map.get(Three::A),
    Some(&200),
    "Expected get() to return new inserted value"
  );
  assert_eq!(
    map.count(),
    1,
    "Expected count to remain 1 after overwriting"
  );

  // Test remove
  let removed_value = map.remove(Three::A);
  assert_eq!(
    removed_value,
    Some(200),
    "Expected remove() to return the removed value"
  );
  assert_eq!(
    map.get(Three::A),
    None,
    "Expected get() to return None after removal"
  );
  assert_eq!(map.count(), 0, "Expected count to be 0 after removal");
  assert!(
    map.is_empty(),
    "Expected map to be empty after removing only value"
  );

  // Test remove non-existent key
  let removed_value = map.remove(Three::B);
  assert_eq!(
    removed_value, None,
    "Expected remove() to return None for non-existent key"
  );
}

#[test]
fn test_insert_remove_by_index() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test insert_by_index
  let old_value = map.insert_by_index(Three::A.into(), 42);
  assert_eq!(
    old_value, None,
    "Expected insert_by_index() to return None for new index"
  );
  assert_eq!(
    map.get_by_index(Three::A.into()),
    Some(&42),
    "Expected get_by_index() to return inserted value"
  );

  // Test remove_by_index
  let removed_value = map.remove_by_index(Three::A.into());
  assert_eq!(
    removed_value,
    Some(42),
    "Expected remove_by_index() to return the removed value"
  );
  assert_eq!(
    map.get_by_index(Three::A.into()),
    None,
    "Expected get_by_index() to return None after removal"
  );

  // Test remove_by_index non-existent
  let removed_value = map.remove_by_index(Three::B.into());
  assert_eq!(
    removed_value, None,
    "Expected remove_by_index() to return None for non-existent index"
  );
}

#[test]
fn test_clear() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Add some values
  map.insert(Three::A, 10);
  map.insert(Three::B, 20);
  map.insert(Three::C, 30);

  assert!(!map.is_empty(), "Expected map to not be empty before clear");
  assert_eq!(map.count(), 3, "Expected count to be 3 before clear");

  // Clear the map
  map.clear();

  assert!(map.is_empty(), "Expected map to be empty after clear");
  assert_eq!(map.count(), 0, "Expected count to be 0 after clear");
  assert_eq!(
    map.get(Three::A),
    None,
    "Expected get() to return None after clear"
  );
  assert_eq!(
    map.get(Three::B),
    None,
    "Expected get() to return None after clear"
  );
  assert_eq!(
    map.get(Three::C),
    None,
    "Expected get() to return None after clear"
  );
}

#[test]
fn test_contains() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test contains on empty map
  assert!(
    !map.contains(Three::A),
    "Expected empty map to not contain Three::A"
  );
  assert!(
    !map.contains(Three::B),
    "Expected empty map to not contain Three::B"
  );
  assert!(
    !map.contains(Three::C),
    "Expected empty map to not contain Three::C"
  );

  // Add a value and test contains
  map.insert(Three::B, 100);
  assert!(
    !map.contains(Three::A),
    "Expected map to not contain Three::A"
  );
  assert!(map.contains(Three::B), "Expected map to contain Three::B");
  assert!(
    !map.contains(Three::C),
    "Expected map to not contain Three::C"
  );

  // Test contains_index
  assert!(
    !map.contains_index(Three::A.into()),
    "Expected map to not contain index A"
  );
  assert!(
    map.contains_index(Three::B.into()),
    "Expected map to contain index B"
  );
  assert!(
    !map.contains_index(Three::C.into()),
    "Expected map to not contain index C"
  );
}

#[test]
fn test_keys() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test keys on empty map
  let keys = map.keys();
  assert!(
    !keys.contains(Three::A),
    "Expected empty map keys to not contain Three::A"
  );
  assert!(
    !keys.contains(Three::B),
    "Expected empty map keys to not contain Three::B"
  );
  assert!(
    !keys.contains(Three::C),
    "Expected empty map keys to not contain Three::C"
  );

  // Add values and test keys
  map.insert(Three::A, 10);
  map.insert(Three::C, 30);

  let keys = map.keys();
  assert!(keys.contains(Three::A), "Expected keys to contain Three::A");
  assert!(
    !keys.contains(Three::B),
    "Expected keys to not contain Three::B"
  );
  assert!(keys.contains(Three::C), "Expected keys to contain Three::C");
}

#[test]
fn test_full_state() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Gradually fill the map
  assert!(!map.is_full(), "Expected map to not be full initially");

  map.insert(Three::A, 10);
  assert!(
    !map.is_full(),
    "Expected map to not be full with 1/3 values"
  );

  map.insert(Three::B, 20);
  assert!(
    !map.is_full(),
    "Expected map to not be full with 2/3 values"
  );

  map.insert(Three::C, 30);
  assert!(map.is_full(), "Expected map to be full with 3/3 values");

  // Remove one value
  map.remove(Three::B);
  assert!(
    !map.is_full(),
    "Expected map to not be full after removing a value"
  );
}

#[test]
fn test_is_vec() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Empty map should be representable as 0-length vector
  assert_eq!(
    map.is_vec(),
    EnumSize::from_usize(0),
    "Expected empty map to be representable as 0-length vector"
  );

  // Add first value - should be representable as 1-length vector
  map.insert(Three::A, 10);
  assert_eq!(
    map.is_vec(),
    EnumSize::from_usize(1),
    "Expected [A] to be representable as 1-length vector"
  );

  // Add second contiguous value - should be representable as 2-length vector
  map.insert(Three::B, 20);
  assert_eq!(
    map.is_vec(),
    EnumSize::from_usize(2),
    "Expected [A,B] to be representable as 2-length vector"
  );

  // Add third contiguous value - should be representable as 3-length vector
  map.insert(Three::C, 30);
  assert_eq!(
    map.is_vec(),
    EnumSize::from_usize(3),
    "Expected [A,B,C] to be representable as 3-length vector"
  );

  // Remove middle value - should not be representable as vector
  map.remove(Three::B);
  assert_eq!(
    map.is_vec(),
    None,
    "Expected [A,_,C] to not be representable as vector"
  );

  // Remove first value, leaving only C - should not be representable as vector
  map.remove(Three::A);
  assert_eq!(
    map.is_vec(),
    None,
    "Expected [_,_,C] to not be representable as vector"
  );
}

#[test]
fn test_mutable_get() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test get_mut on empty map
  assert_eq!(
    map.get_mut(Three::A),
    None,
    "Expected get_mut() to return None for empty map"
  );

  // Insert a value and test get_mut
  map.insert(Three::A, 100);

  let value_mut = map
    .get_mut(Three::A)
    .expect("Expected get_mut to return Some for existing key");
  *value_mut += 50;

  assert_eq!(
    map.get(Three::A),
    Some(&150),
    "Expected value to be modified through get_mut()"
  );

  // Test get_by_index_mut
  let value_mut = map
    .get_by_index_mut(Three::A.into())
    .expect("Expected get_by_index_mut to return Some for existing key");
  *value_mut *= 2;

  assert_eq!(
    map.get(Three::A),
    Some(&300),
    "Expected value to be modified through get_by_index_mut()"
  );
}

#[test]
fn test_iteration_present_elements() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test iteration on empty map
  let collected: Vec<_> = map.iter().collect();
  assert_eq!(collected, vec![], "Expected empty iteration for empty map");

  // Add values non-contiguously
  map.insert(Three::A, 10);
  map.insert(Three::C, 30);

  // Test immutable iteration
  let collected: Vec<_> = map.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &10), (Three::C, &30)],
    "Expected iteration to yield inserted values in order"
  );

  // Test mutable iteration
  for (_, value) in map.iter_mut() {
    *value *= 10;
  }

  let collected: Vec<_> = map.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &100), (Three::C, &300)],
    "Expected values to be modified through iter_mut()"
  );
}

#[test]
fn test_iterator_exact_size_and_double_ended() {
  let mut map = EnumOptionMap::<Three, i32>::new();
  map.insert(Three::A, 10);
  map.insert(Three::C, 30);

  let mut iter = map.iter();

  // Test size_hint
  assert_eq!(
    iter.size_hint().0,
    2,
    "Expected correct size hint lower bound"
  );

  // Test Iterator::nth
  assert_eq!(
    iter.nth(1),
    Some((Three::C, &30)),
    "Expected nth(1) to return second element"
  );

  // Test iterator count
  assert_eq!(map.iter().count(), 2, "Expected iterator count to be 2");

  // Test empty iterator traits
  let empty_map = EnumOptionMap::<Three, i32>::new();
  let empty_iter = empty_map.iter();
  assert_eq!(
    empty_iter.size_hint().0,
    0,
    "Expected empty iterator lower bound to be 0"
  );
}

#[test]
fn test_iterator_partial_consumption() {
  let mut map = EnumOptionMap::<Three, i32>::new();
  map.insert(Three::A, 10);
  map.insert(Three::B, 20);
  map.insert(Three::C, 30);

  let mut iter = map.iter();

  // Consume first element
  assert_eq!(iter.next(), Some((Three::A, &10)), "Expected first element");

  // Test remaining elements
  let remaining: Vec<_> = iter.collect();
  assert_eq!(
    remaining,
    vec![(Three::B, &20), (Three::C, &30)],
    "Expected remaining elements after partial consumption"
  );
}

#[test]
fn test_iterator_single_element() {
  let mut map = EnumOptionMap::<Three, i32>::new();
  map.insert(Three::B, 42);

  let collected: Vec<_> = map.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::B, &42)],
    "Expected single element iteration"
  );

  let mut iter = map.iter();
  assert_eq!(
    iter.next(),
    Some((Three::B, &42)),
    "Expected single element"
  );
  assert_eq!(iter.next(), None, "Expected iterator to be exhausted");
}

#[test]
fn test_swap_present_with_present() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test swap with both values present
  map.insert(Three::A, 10);
  map.insert(Three::C, 30);

  map.swap(Three::A, Three::C);
  assert_eq!(
    map.get(Three::A),
    Some(&30),
    "Expected swapped value for Three::A"
  );
  assert_eq!(
    map.get(Three::C),
    Some(&10),
    "Expected swapped value for Three::C"
  );
  assert_eq!(map.count(), 2, "Expected count to remain 2 after swap");
}

#[test]
fn test_swap_by_index_present_with_present() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test swap_by_index with both values present
  map.insert(Three::A, 15);
  map.insert(Three::B, 25);

  map.swap_by_index(Three::A.into(), Three::B.into());
  assert_eq!(
    map.get(Three::A),
    Some(&25),
    "Expected swapped value for Three::A after swap_by_index"
  );
  assert_eq!(
    map.get(Three::B),
    Some(&15),
    "Expected swapped value for Three::B after swap_by_index"
  );
  assert_eq!(
    map.count(),
    2,
    "Expected count to remain 2 after swap_by_index"
  );
}

#[test]
fn test_swap_present_with_absent() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test swap with one value present, one absent
  // Based on the implementation, swapping present with absent does nothing
  map.insert(Three::A, 100);

  map.swap(Three::A, Three::B);
  assert_eq!(
    map.get(Three::A),
    Some(&100),
    "Expected Three::A to remain unchanged after swap with absent value"
  );
  assert_eq!(
    map.get(Three::B),
    None,
    "Expected Three::B to remain None after swap with present value"
  );
  assert_eq!(map.count(), 1, "Expected count to remain 1 after swap");
}

#[test]
fn test_swap_by_index_present_with_absent() {
  let mut map = EnumOptionMap::<Three, i32>::new();

  // Test swap_by_index with one present, one absent
  map.insert(Three::C, 200);

  map.swap_by_index(Three::C.into(), Three::A.into());
  assert_eq!(
    map.get(Three::C),
    Some(&200),
    "Expected Three::C to remain unchanged after swap_by_index with absent value"
  );
  assert_eq!(
    map.get(Three::A),
    None,
    "Expected Three::A to remain None after swap_by_index with present value"
  );
  assert_eq!(
    map.count(),
    1,
    "Expected count to remain 1 after swap_by_index"
  );
}
