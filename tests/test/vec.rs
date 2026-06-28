use crate::test::drop_tracker::DropTracker;
use crate::test::types::{Sixteen, Three};
use enumoid::EnumOptionMap;
use enumoid::EnumSize;
use enumoid::EnumVec;
use std::cell::Cell;

#[test]
fn test_empty_state() {
  let vec = EnumVec::<Three, u16>::new();

  assert!(vec.is_empty(), "Expected new vec to be empty");
  assert!(!vec.is_full(), "Expected new vec to not be full");
  assert_eq!(
    vec.size(),
    EnumSize::EMPTY,
    "Expected new vec to have empty size"
  );

  // Test that no keys are contained
  assert!(
    !vec.contains(Three::A),
    "Expected new vec to not contain Three::A"
  );
  assert!(
    !vec.contains(Three::B),
    "Expected new vec to not contain Three::B"
  );
  assert!(
    !vec.contains(Three::C),
    "Expected new vec to not contain Three::C"
  );

  // Test contains_index
  assert!(
    !vec.contains_index(Three::A.into()),
    "Expected new vec to not contain index A"
  );
  assert!(
    !vec.contains_index(Three::B.into()),
    "Expected new vec to not contain index B"
  );
  assert!(
    !vec.contains_index(Three::C.into()),
    "Expected new vec to not contain index C"
  );

  // Test get methods return None
  assert_eq!(
    vec.get(Three::A),
    None,
    "Expected get() to return None for empty vec"
  );
  assert_eq!(
    vec.get(Three::B),
    None,
    "Expected get() to return None for empty vec"
  );
  assert_eq!(
    vec.get(Three::C),
    None,
    "Expected get() to return None for empty vec"
  );

  assert_eq!(
    vec.get_by_index(Three::A.into()),
    None,
    "Expected get_by_index() to return None for empty vec"
  );
  assert_eq!(
    vec.get_by_index(Three::B.into()),
    None,
    "Expected get_by_index() to return None for empty vec"
  );
  assert_eq!(
    vec.get_by_index(Three::C.into()),
    None,
    "Expected get_by_index() to return None for empty vec"
  );

  // Test empty iteration
  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(collected, vec![], "Expected empty iteration for empty vec");
}

#[test]
fn test_push_pop() {
  let mut vec = EnumVec::<Three, u16>::new();

  // Test pop on empty vec
  assert_eq!(
    vec.pop(),
    None,
    "Expected pop() to return None for empty vec"
  );

  // Test push and indexing
  vec
    .try_push(100)
    .expect("Failed to push first element to empty vec");
  assert!(!vec.is_empty(), "Expected vec to not be empty after push");
  assert!(!vec.is_full(), "Expected vec to not be full after one push");
  assert_eq!(
    vec[Three::A],
    100,
    "Expected index access to return pushed value"
  );
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::A),
    "Expected size to reflect one element"
  );

  vec
    .try_push(200)
    .expect("Failed to push second element to vec");
  assert_eq!(
    vec[Three::A],
    100,
    "Expected first element to remain unchanged"
  );
  assert_eq!(
    vec[Three::B],
    200,
    "Expected second element to be accessible"
  );
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::B),
    "Expected size to reflect two elements"
  );

  vec
    .try_push(300)
    .expect("Failed to push third element to vec");
  assert!(
    vec.is_full(),
    "Expected vec to be full after pushing all elements"
  );
  assert_eq!(
    vec[Three::C],
    300,
    "Expected third element to be accessible"
  );
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::C),
    "Expected size to reflect three elements"
  );

  // Test pop
  assert_eq!(
    vec.pop(),
    Some(300),
    "Expected pop() to return last pushed value"
  );
  assert!(!vec.is_full(), "Expected vec to not be full after pop");
  assert_eq!(
    vec.get(Three::C),
    None,
    "Expected get() to return None for popped element"
  );
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::B),
    "Expected size to reflect pop"
  );

  // Test mutable indexing
  vec[Three::B] += 1;
  assert_eq!(
    vec[Three::B],
    201,
    "Expected mutable indexing to modify value"
  );
}

#[test]
fn test_new_with_constructor() {
  let vec =
    EnumVec::<Three, i32>::new_with(EnumSize::from_last(Three::B), |variant| {
      match variant {
        Three::A => 10,
        Three::B => 20,
        Three::C => 30,
      }
    });

  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::B),
    "Expected size to match constructor parameter"
  );
  assert_eq!(vec[Three::A], 10, "Expected correct value for Three::A");
  assert_eq!(vec[Three::B], 20, "Expected correct value for Three::B");
  assert_eq!(
    vec.get(Three::C),
    None,
    "Expected None for Three::C since it's beyond size"
  );

  // Test with full size
  let full_vec =
    EnumVec::<Three, i32>::new_with(EnumSize::from_last(Three::C), |variant| {
      match variant {
        Three::A => 100,
        Three::B => 200,
        Three::C => 300,
      }
    });

  assert!(full_vec.is_full(), "Expected full vec to be full");
  assert_eq!(
    full_vec[Three::A],
    100,
    "Expected correct value for Three::A in full vec"
  );
  assert_eq!(
    full_vec[Three::B],
    200,
    "Expected correct value for Three::B in full vec"
  );
  assert_eq!(
    full_vec[Three::C],
    300,
    "Expected correct value for Three::C in full vec"
  );
}

#[test]
fn test_get_methods() {
  let mut vec: EnumVec<Three, i32> = [42, 84].into_iter().collect();

  // Test get and get_by_index for valid indices
  assert_eq!(
    vec.get(Three::A),
    Some(&42),
    "Expected get() to return reference to first element"
  );
  assert_eq!(
    vec.get(Three::B),
    Some(&84),
    "Expected get() to return reference to second element"
  );
  assert_eq!(
    vec.get(Three::C),
    None,
    "Expected get() to return None for index beyond size"
  );

  assert_eq!(
    vec.get_by_index(Three::A.into()),
    Some(&42),
    "Expected get_by_index() to return reference to first element"
  );
  assert_eq!(
    vec.get_by_index(Three::B.into()),
    Some(&84),
    "Expected get_by_index() to return reference to second element"
  );
  assert_eq!(
    vec.get_by_index(Three::C.into()),
    None,
    "Expected get_by_index() to return None for index beyond size"
  );

  // Test mutable get methods
  *vec
    .get_mut(Three::A)
    .expect("Expected get_mut to return Some for existing element") = 100;
  assert_eq!(
    vec[Three::A],
    100,
    "Expected value to be modified through get_mut()"
  );

  *vec
    .get_by_index_mut(Three::B.into())
    .expect("Expected get_by_index_mut to return Some for existing element") =
    200;
  assert_eq!(
    vec[Three::B],
    200,
    "Expected value to be modified through get_by_index_mut()"
  );

  // Test mutable get methods return None for out-of-bounds
  assert_eq!(
    vec.get_mut(Three::C),
    None,
    "Expected get_mut() to return None for index beyond size"
  );
  assert_eq!(
    vec.get_by_index_mut(Three::C.into()),
    None,
    "Expected get_by_index_mut() to return None for index beyond size"
  );
}

#[test]
fn test_contains() {
  let vec = EnumVec::<Three, i32>::new();

  // Test contains on empty vec
  assert!(
    !vec.contains(Three::A),
    "Expected empty vec to not contain Three::A"
  );
  assert!(
    !vec.contains(Three::B),
    "Expected empty vec to not contain Three::B"
  );
  assert!(
    !vec.contains(Three::C),
    "Expected empty vec to not contain Three::C"
  );

  // Add elements and test contains
  let mut vec_with_one: EnumVec<Three, i32> = [10].into_iter().collect();
  assert!(
    vec_with_one.contains(Three::A),
    "Expected vec to contain Three::A"
  );
  assert!(
    !vec_with_one.contains(Three::B),
    "Expected vec to not contain Three::B"
  );
  assert!(
    !vec_with_one.contains(Three::C),
    "Expected vec to not contain Three::C"
  );

  vec_with_one
    .try_push(20)
    .expect("Failed to push second element in contains test");
  assert!(
    vec_with_one.contains(Three::A),
    "Expected vec to contain Three::A"
  );
  assert!(
    vec_with_one.contains(Three::B),
    "Expected vec to contain Three::B"
  );
  assert!(
    !vec_with_one.contains(Three::C),
    "Expected vec to not contain Three::C"
  );

  // Test contains_index
  assert!(
    vec_with_one.contains_index(Three::A.into()),
    "Expected vec to contain index A"
  );
  assert!(
    vec_with_one.contains_index(Three::B.into()),
    "Expected vec to contain index B"
  );
  assert!(
    !vec_with_one.contains_index(Three::C.into()),
    "Expected vec to not contain index C"
  );
}

#[test]
fn test_clear() {
  let mut vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();

  assert!(vec.is_full(), "Expected vec to be full before clear");
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::C),
    "Expected size to be full before clear"
  );

  // Clear the vec
  vec.clear();

  assert!(vec.is_empty(), "Expected vec to be empty after clear");
  assert_eq!(
    vec.size(),
    EnumSize::EMPTY,
    "Expected size to be empty after clear"
  );
  assert_eq!(
    vec.get(Three::A),
    None,
    "Expected get() to return None after clear"
  );
  assert_eq!(
    vec.get(Three::B),
    None,
    "Expected get() to return None after clear"
  );
  assert_eq!(
    vec.get(Three::C),
    None,
    "Expected get() to return None after clear"
  );

  // Test that we can push again after clear
  vec
    .try_push(100)
    .expect("Failed to push element after clear");
  assert_eq!(
    vec[Three::A],
    100,
    "Expected to be able to push after clear"
  );
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::A),
    "Expected size to reflect new element after clear"
  );
}

#[test]
fn test_remove_at_index() {
  let mut vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();

  // Test remove_at_index returns value for valid index
  let removed = vec.remove_at_index(Three::B.into());
  assert_eq!(
    removed,
    Some(20),
    "Expected remove_at_index() to return removed value"
  );
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::B),
    "Expected size to decrease after remove"
  );

  // Test that remaining elements shifted down
  assert_eq!(
    vec[Three::A],
    10,
    "Expected first element to remain unchanged"
  );
  assert_eq!(
    vec[Three::B],
    30,
    "Expected third element to shift down to second position"
  );
  assert_eq!(
    vec.get(Three::C),
    None,
    "Expected get() to return None for index beyond new size"
  );

  // Test remove_at_index returns None for out-of-bounds index
  let removed = vec.remove_at_index(Three::C.into());
  assert_eq!(
    removed, None,
    "Expected remove_at_index() to return None for out-of-bounds index"
  );
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::B),
    "Expected size to remain unchanged after failed remove"
  );
}

#[test]
fn test_swap_remove_at_index() {
  let mut vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();

  // Test swap_remove_at_index returns value and replaces with last element
  let removed = vec.swap_remove_at_index(Three::A.into());
  assert_eq!(
    removed,
    Some(10),
    "Expected swap_remove_at_index() to return removed value"
  );
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::B),
    "Expected size to decrease after swap_remove"
  );

  // Test that removed element was replaced with last element
  assert_eq!(
    vec[Three::A],
    30,
    "Expected first element to be replaced with last element"
  );
  assert_eq!(
    vec[Three::B],
    20,
    "Expected second element to remain unchanged"
  );
  assert_eq!(
    vec.get(Three::C),
    None,
    "Expected get() to return None for index beyond new size"
  );

  // Test swap_remove_at_index returns None for out-of-bounds index
  let removed = vec.swap_remove_at_index(Three::C.into());
  assert_eq!(
    removed, None,
    "Expected swap_remove_at_index() to return None for out-of-bounds index"
  );
  assert_eq!(
    vec.size(),
    EnumSize::from_last(Three::B),
    "Expected size to remain unchanged after failed swap_remove"
  );
}

#[test]
fn test_slice_access() {
  let mut vec: EnumVec<Three, i32> = [10, 20].into_iter().collect();

  // Test as_slice
  let slice = vec.as_slice();
  assert_eq!(
    slice,
    &[10, 20],
    "Expected slice to contain all values in order"
  );

  // Test as_slice_mut
  {
    let slice_mut = vec.as_slice_mut();
    slice_mut[0] = 100;
    assert_eq!(
      slice_mut.len(),
      2,
      "Expected slice length to match vec size"
    );
  }
  assert_eq!(
    vec[Three::A],
    100,
    "Expected value change through mutable slice"
  );
}

#[test]
fn test_iteration_elements_in_order() {
  let vec = EnumVec::<Three, i32>::new();

  // Test iteration on empty vec
  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(collected, vec![], "Expected empty iteration for empty vec");

  // Add elements and test iteration
  let mut vec: EnumVec<Three, i32> = [10, 20].into_iter().collect();

  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &10), (Three::B, &20)],
    "Expected iteration to yield key-value pairs in order"
  );

  // Test mutable iteration
  for (_, value) in vec.iter_mut() {
    *value *= 10;
  }

  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &100), (Three::B, &200)],
    "Expected values to be modified through iter_mut()"
  );
}

#[test]
fn test_iterator_exact_size_and_double_ended() {
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();

  let mut iter = vec.iter();

  // Test ExactSizeIterator
  assert_eq!(iter.len(), 3, "Expected iterator length to be 3");

  // Test size_hint
  assert_eq!(iter.size_hint(), (3, Some(3)), "Expected correct size hint");

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

  // Test DoubleEndedIterator
  let mut iter = vec.iter();
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

  // Test iterator count
  assert_eq!(vec.iter().count(), 3, "Expected iterator count to be 3");

  // Test empty iterator traits
  let empty_vec = EnumVec::<Three, i32>::new();
  let empty_iter = empty_vec.iter();
  assert_eq!(
    empty_iter.len(),
    0,
    "Expected empty iterator length to be 0"
  );
  assert!(empty_iter.len() == 0, "Expected empty iterator to be empty");
  assert_eq!(
    empty_iter.size_hint(),
    (0, Some(0)),
    "Expected correct size hint for empty iterator"
  );
}

#[test]
fn test_into_iterator() {
  let mut vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();

  // Test IntoIterator for &EnumVec
  let collected: Vec<_> = (&vec).into_iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &10), (Three::B, &20), (Three::C, &30)],
    "Expected IntoIterator to work for &EnumVec"
  );

  // Test IntoIterator for &mut EnumVec
  let collected_mut: Vec<_> =
    (&mut vec).into_iter().map(|(k, v)| (k, *v)).collect();
  assert_eq!(
    collected_mut,
    vec![(Three::A, 10), (Three::B, 20), (Three::C, 30)],
    "Expected IntoIterator to work for &mut EnumVec"
  );

  // Test for loop syntax
  let mut sum = 0;
  for (_, value) in &vec {
    sum += *value;
  }
  assert_eq!(sum, 60, "Expected for loop to sum all values");

  // Test with mutable reference
  for (_, value) in &mut vec {
    *value += 1;
  }

  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &11), (Three::B, &21), (Three::C, &31)],
    "Expected values to be modified through mutable for loop"
  );
}

#[test]
fn test_from_iterator() {
  // Test FromIterator implementation
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();

  assert_eq!(vec[Three::A], 10, "Expected first element from iterator");
  assert_eq!(vec[Three::B], 20, "Expected second element from iterator");
  assert_eq!(vec[Three::C], 30, "Expected third element from iterator");
  assert!(
    vec.is_full(),
    "Expected vec to be full after collecting all elements"
  );

  // Test partial collection
  let partial_vec: EnumVec<Three, i32> = [100, 200].into_iter().collect();
  assert_eq!(
    partial_vec[Three::A],
    100,
    "Expected first element in partial vec"
  );
  assert_eq!(
    partial_vec[Three::B],
    200,
    "Expected second element in partial vec"
  );
  assert_eq!(
    partial_vec.get(Three::C),
    None,
    "Expected no third element in partial vec"
  );

  // Test empty collection
  let empty_vec: EnumVec<Three, i32> = std::iter::empty().collect();
  assert!(
    empty_vec.is_empty(),
    "Expected empty vec from empty iterator"
  );
}

#[test]
fn test_extend() {
  // Extending pushes additional elements onto an existing vec.
  let mut vec = EnumVec::<Three, i32>::new();
  vec.try_push(10).unwrap();
  vec.extend([20, 30]);

  assert_eq!(vec[Three::A], 10, "Expected preexisting element");
  assert_eq!(vec[Three::B], 20, "Expected first extended element");
  assert_eq!(vec[Three::C], 30, "Expected second extended element");
  assert!(vec.is_full(), "Expected vec to be full after extending");
}

#[test]
fn test_extend_overflow() {
  // Extending beyond capacity stops at capacity rather than panicking.
  let mut vec = EnumVec::<Three, i32>::new();
  vec.extend([10, 20, 30, 40, 50]);

  assert!(
    vec.is_full(),
    "Expected vec to be full after extending beyond capacity"
  );
  assert_eq!(vec[Three::C], 30, "Expected last in-capacity element");
}

#[test]
fn test_from_iterator_overflow() {
  // Collecting more elements than the type can hold stops at capacity rather
  // than panicking, and stops pulling from the iterator once full.
  let mut iter = [10, 20, 30, 40, 50].into_iter();
  let vec: EnumVec<Three, i32> = iter.by_ref().collect();

  assert!(
    vec.is_full(),
    "Expected vec to be full after collecting beyond capacity"
  );
  assert_eq!(vec[Three::A], 10, "Expected first element");
  assert_eq!(vec[Three::B], 20, "Expected second element");
  assert_eq!(vec[Three::C], 30, "Expected third element");

  // The overflow `break` must fire on the first element past capacity (40),
  // leaving the remainder of the iterator unconsumed.
  let remaining: Vec<i32> = iter.collect();
  assert_eq!(
    remaining,
    vec![50],
    "Expected collect to stop pulling once the vec is full"
  );
}

#[test]
fn test_iterator_partial_consumption() {
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();

  let mut iter = vec.iter();

  // Consume first element
  assert_eq!(iter.next(), Some((Three::A, &10)), "Expected first element");
  assert_eq!(iter.len(), 2, "Expected length to decrease after next()");

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
  let vec: EnumVec<Three, i32> = [42].into_iter().collect();

  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &42)],
    "Expected single element iteration"
  );

  let mut iter = vec.iter();
  assert_eq!(iter.len(), 1, "Expected single element iterator length");
  assert_eq!(
    iter.next(),
    Some((Three::A, &42)),
    "Expected single element"
  );
  assert_eq!(iter.next(), None, "Expected iterator to be exhausted");
  assert_eq!(iter.len(), 0, "Expected exhausted iterator length to be 0");
}

#[test]
fn test_swap_elements() {
  let mut vec: EnumVec<Sixteen, u16> = [1, 2, 3, 4, 5].into_iter().collect();
  vec.swap(Sixteen::B, Sixteen::D);
  let collected: Vec<u16> = vec.iter().map(|x| *x.1).collect();
  assert_eq!(collected, vec![1, 4, 3, 2, 5]);
}

#[test]
fn test_swap_remove() {
  let mut vec: EnumVec<Sixteen, u16> = [1, 2, 3, 4, 5].into_iter().collect();
  assert_eq!(vec.swap_remove(Sixteen::F), None);
  assert_eq!(vec.swap_remove(Sixteen::B), Some(2));
  let collected: Vec<u16> = vec.iter().map(|x| *x.1).collect();
  assert_eq!(collected, vec![1, 5, 3, 4]);
}

#[test]
fn test_remove() {
  let mut vec: EnumVec<Sixteen, u16> = [1, 2, 3, 4, 5].into_iter().collect();
  assert_eq!(vec.remove(Sixteen::F), None);
  assert_eq!(vec.remove(Sixteen::B), Some(2));
  let collected: Vec<u16> = vec.iter().map(|x| *x.1).collect();
  assert_eq!(collected, vec![1, 3, 4, 5]);
}

#[test]
fn test_push() {
  let mut vec = EnumVec::<Three, u16>::new();
  vec
    .try_push(100)
    .expect("Failed to push first element in push test");
  vec
    .try_push(200)
    .expect("Failed to push second element in push test");
  vec
    .try_push(300)
    .expect("Failed to push third element in push test");
  let overflow = vec.try_push(400);
  assert_eq!(overflow, Err(400), "Expected overflow to return the value");
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_out_of_bounds_panics() {
  let mut vec = EnumVec::<Three, u16>::new();
  vec.swap(Three::A, Three::C);
}

#[test]
fn test_try_from_contiguous_option_map() {
  let mut opt = EnumOptionMap::<Three, i32>::new();
  opt.insert(Three::A, 10);
  opt.insert(Three::B, 20); // [A, B] is a contiguous prefix, so vec-representable

  let vec = EnumVec::try_from(opt)
    .expect("Expected Ok when option map is a contiguous prefix");
  assert_eq!(vec[Three::A], 10, "Expected converted value for A");
  assert_eq!(vec[Three::B], 20, "Expected converted value for B");
  assert_eq!(vec.get(Three::C), None, "Expected C to be absent");
}

#[test]
fn test_try_from_gapped_option_map_fails() {
  let mut opt = EnumOptionMap::<Three, i32>::new();
  opt.insert(Three::A, 10);
  opt.insert(Three::C, 30); // [A, _, C] has a gap, so not vec-representable

  assert_eq!(
    EnumVec::try_from(opt),
    Err(()),
    "Expected Err when option map is not a contiguous prefix"
  );
}

#[test]
fn test_into_iterator_owned() {
  // The consuming IntoIterator yields owned values, mirroring FromIterator<V>.
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();
  let collected: Vec<i32> = vec.into_iter().collect();
  assert_eq!(
    collected,
    vec![10, 20, 30],
    "Expected consuming iteration to yield owned values in order"
  );
}

#[test]
fn test_into_iterator_owned_partial() {
  // A partially-filled vec only yields its populated prefix.
  let vec: EnumVec<Three, i32> = [10, 20].into_iter().collect();
  let collected: Vec<i32> = vec.into_iter().collect();
  assert_eq!(
    collected,
    vec![10, 20],
    "Expected consuming iteration to yield only the populated prefix"
  );
}

#[test]
fn test_into_iterator_owned_roundtrips() {
  // into_iter().collect() reconstructs an equal vec.
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();
  let roundtripped: EnumVec<Three, i32> = vec.into_iter().collect();
  let expected: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();
  assert_eq!(
    roundtripped, expected,
    "Expected into_iter().collect() to round-trip"
  );
}

#[test]
fn test_into_iterator_owned_exact_size() {
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();
  let mut iter = vec.into_iter();
  assert_eq!(iter.len(), 3, "Expected ExactSizeIterator len of 3");
  assert_eq!(iter.size_hint(), (3, Some(3)), "Expected exact size hint");
  iter.next();
  assert_eq!(iter.len(), 2, "Expected len to decrease after next()");
}

#[test]
fn test_into_iterator_owned_drops_all_when_consumed() {
  let drops = Cell::new(0);
  let vec: EnumVec<Three, DropTracker> = [
    DropTracker::new(1, &drops),
    DropTracker::new(2, &drops),
    DropTracker::new(3, &drops),
  ]
  .into_iter()
  .collect();

  let ids: Vec<i32> = vec.into_iter().map(|t| t.id()).collect();
  assert_eq!(ids, vec![1, 2, 3], "Expected ids in order");
  assert_eq!(
    drops.get(),
    3,
    "Expected each yielded element to be dropped exactly once"
  );
}

#[test]
fn test_into_iterator_owned_drops_remainder_when_abandoned() {
  // Abandoning a partially-consumed iterator must drop the unyielded elements
  // exactly once (and not the one already moved out).
  let drops = Cell::new(0);
  let vec: EnumVec<Three, DropTracker> = [
    DropTracker::new(1, &drops),
    DropTracker::new(2, &drops),
    DropTracker::new(3, &drops),
  ]
  .into_iter()
  .collect();

  let mut iter = vec.into_iter();
  {
    let first = iter.next().expect("Expected a first element");
    assert_eq!(first.id(), 1, "Expected first element id");
    assert_eq!(drops.get(), 0, "Expected no drops while first is held");
  }
  assert_eq!(drops.get(), 1, "Expected the moved-out element to drop");

  drop(iter);
  assert_eq!(
    drops.get(),
    3,
    "Expected the two unyielded elements to drop when abandoned"
  );
}

#[test]
fn test_into_iterator_owned_double_ended() {
  // next() and next_back() meet in the middle, each element yielded once.
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();
  let mut iter = vec.into_iter();
  assert_eq!(
    iter.next_back(),
    Some(30),
    "Expected next_back() to yield the last element"
  );
  assert_eq!(
    iter.next(),
    Some(10),
    "Expected next() to yield the first element"
  );
  assert_eq!(
    iter.next_back(),
    Some(20),
    "Expected next_back() to yield the remaining middle element"
  );
  assert_eq!(iter.next(), None, "Expected exhaustion from the front");
  assert_eq!(iter.next_back(), None, "Expected exhaustion from the back");
}

#[test]
fn test_into_iterator_owned_double_ended_partial() {
  // A partially-filled vec is double-ended only over its populated prefix.
  let vec: EnumVec<Three, i32> = [10, 20].into_iter().collect();
  let collected: Vec<i32> = vec.into_iter().rev().collect();
  assert_eq!(
    collected,
    vec![20, 10],
    "Expected reversed iteration over only the populated prefix"
  );
}

#[test]
fn test_into_iterator_owned_rev() {
  // rev() relies on DoubleEndedIterator and yields values in reverse order.
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();
  let collected: Vec<i32> = vec.into_iter().rev().collect();
  assert_eq!(
    collected,
    vec![30, 20, 10],
    "Expected reversed iteration to yield values in reverse order"
  );
}

#[test]
fn test_into_iterator_owned_double_ended_exact_size() {
  // len() stays accurate as the range is consumed from both ends.
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();
  let mut iter = vec.into_iter();
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
  let vec: EnumVec<Three, i32> = [10, 20, 30].into_iter().collect();
  let mut iter = vec.into_iter();
  for _ in 0..3 {
    assert!(
      iter.next().is_some(),
      "Expected three elements before exhaustion"
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
  // After taking one element from each end, abandoning the iterator drops only
  // the unyielded middle element, exactly once, and never the yielded ends.
  let drops = Cell::new(0);
  let vec: EnumVec<Three, DropTracker> = [
    DropTracker::new(1, &drops),
    DropTracker::new(2, &drops),
    DropTracker::new(3, &drops),
  ]
  .into_iter()
  .collect();

  let mut iter = vec.into_iter();
  {
    let front = iter.next().expect("Expected a front element");
    let back = iter.next_back().expect("Expected a back element");
    assert_eq!(front.id(), 1, "Expected front element id");
    assert_eq!(back.id(), 3, "Expected back element id");
    assert_eq!(drops.get(), 0, "Expected no drops while elements are held");
  }
  assert_eq!(
    drops.get(),
    2,
    "Expected the two moved-out elements to drop"
  );

  drop(iter);
  assert_eq!(
    drops.get(),
    3,
    "Expected the single unyielded middle element to drop exactly once"
  );
}

#[test]
fn test_iter_mut_double_ended() {
  // EnumSliceIterMut is double-ended; mutations through next_back must land.
  let mut vec: EnumVec<Three, u16> = [10, 20, 30].into_iter().collect();

  {
    let mut iter = vec.iter_mut();
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

  assert_eq!(vec[Three::A], 11, "Expected front mutation to persist");
  assert_eq!(vec[Three::C], 31, "Expected back mutation to persist");
}
