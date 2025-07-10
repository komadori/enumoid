use crate::test::types::{Sixteen, Three};
use enumoid::EnumSize;
use enumoid::EnumVec;

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
  vec.push(100);
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

  vec.push(200);
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

  vec.push(300);
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
  let mut vec = EnumVec::<Three, i32>::new();
  vec.push(42);
  vec.push(84);

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
  *vec.get_mut(Three::A).unwrap() = 100;
  assert_eq!(
    vec[Three::A],
    100,
    "Expected value to be modified through get_mut()"
  );

  *vec.get_by_index_mut(Three::B.into()).unwrap() = 200;
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
  let mut vec = EnumVec::<Three, i32>::new();

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
  vec.push(10);
  assert!(vec.contains(Three::A), "Expected vec to contain Three::A");
  assert!(
    !vec.contains(Three::B),
    "Expected vec to not contain Three::B"
  );
  assert!(
    !vec.contains(Three::C),
    "Expected vec to not contain Three::C"
  );

  vec.push(20);
  assert!(vec.contains(Three::A), "Expected vec to contain Three::A");
  assert!(vec.contains(Three::B), "Expected vec to contain Three::B");
  assert!(
    !vec.contains(Three::C),
    "Expected vec to not contain Three::C"
  );

  // Test contains_index
  assert!(
    vec.contains_index(Three::A.into()),
    "Expected vec to contain index A"
  );
  assert!(
    vec.contains_index(Three::B.into()),
    "Expected vec to contain index B"
  );
  assert!(
    !vec.contains_index(Three::C.into()),
    "Expected vec to not contain index C"
  );
}

#[test]
fn test_clear() {
  let mut vec = EnumVec::<Three, i32>::new();

  // Add some elements
  vec.push(10);
  vec.push(20);
  vec.push(30);

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
  vec.push(100);
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
  let mut vec = EnumVec::<Three, i32>::new();
  vec.push(10);
  vec.push(20);
  vec.push(30);

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
  let mut vec = EnumVec::<Three, i32>::new();
  vec.push(10);
  vec.push(20);
  vec.push(30);

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
  let mut vec = EnumVec::<Three, i32>::new();
  vec.push(10);
  vec.push(20);

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
  let mut vec = EnumVec::<Three, i32>::new();

  // Test iteration on empty vec
  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(collected, vec![], "Expected empty iteration for empty vec");

  // Add elements and test iteration
  vec.push(10);
  vec.push(20);

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
  let mut vec = EnumVec::<Three, i32>::new();
  vec.push(10);
  vec.push(20);
  vec.push(30);

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
  let mut vec = EnumVec::<Three, i32>::new();
  vec.push(10);
  vec.push(20);
  vec.push(30);

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
fn test_iterator_partial_consumption() {
  let mut vec = EnumVec::<Three, i32>::new();
  vec.push(10);
  vec.push(20);
  vec.push(30);

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
  let mut vec = EnumVec::<Three, i32>::new();
  vec.push(42);

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
#[should_panic(expected = "index out of bounds")]
fn test_push_when_full_panics() {
  let mut vec = EnumVec::<Three, u16>::new();
  vec.push(100);
  vec.push(200);
  vec.push(300);
  vec.push(400);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_swap_out_of_bounds_panics() {
  let mut vec = EnumVec::<Three, u16>::new();
  vec.swap(Three::A, Three::C);
}
