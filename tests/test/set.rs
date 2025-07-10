use crate::test::types::{Seventeen, Three};
use enumoid::EnumSet;

#[test]
fn test_empty_state() {
  let set = EnumSet::<Three>::new();

  assert!(!set.any(), "Expected new set to not have any members");
  assert!(!set.all(), "Expected new set to not have all members");
  assert_eq!(set.count(), 0, "Expected new set to have count of 0");

  // Test that no members are contained
  assert!(
    !set.contains(Three::A),
    "Expected new set to not contain Three::A"
  );
  assert!(
    !set.contains(Three::B),
    "Expected new set to not contain Three::B"
  );
  assert!(
    !set.contains(Three::C),
    "Expected new set to not contain Three::C"
  );

  // Test contains_index
  assert!(
    !set.contains_index(Three::A.into()),
    "Expected new set to not contain index A"
  );
  assert!(
    !set.contains_index(Three::B.into()),
    "Expected new set to not contain index B"
  );
  assert!(
    !set.contains_index(Three::C.into()),
    "Expected new set to not contain index C"
  );

  // Test empty iteration
  let collected: Vec<_> = set.iter().collect();
  assert_eq!(collected, vec![], "Expected empty iteration for empty set");

  let collected_indices: Vec<_> = set.iter_index().collect();
  assert_eq!(
    collected_indices,
    vec![],
    "Expected empty index iteration for empty set"
  );
}

#[test]
fn test_new_all() {
  let set = EnumSet::<Three>::new_all();

  assert!(set.any(), "Expected new_all set to have any members");
  assert!(set.all(), "Expected new_all set to have all members");
  assert_eq!(set.count(), 3, "Expected new_all set to have count of 3");

  // Test that all members are contained
  assert!(
    set.contains(Three::A),
    "Expected new_all set to contain Three::A"
  );
  assert!(
    set.contains(Three::B),
    "Expected new_all set to contain Three::B"
  );
  assert!(
    set.contains(Three::C),
    "Expected new_all set to contain Three::C"
  );

  // Test contains_index
  assert!(
    set.contains_index(Three::A.into()),
    "Expected new_all set to contain index A"
  );
  assert!(
    set.contains_index(Three::B.into()),
    "Expected new_all set to contain index B"
  );
  assert!(
    set.contains_index(Three::C.into()),
    "Expected new_all set to contain index C"
  );

  // Test full iteration
  let collected: Vec<_> = set.iter().collect();
  assert_eq!(
    collected,
    vec![Three::A, Three::B, Three::C],
    "Expected full iteration for new_all set"
  );

  let collected_indices: Vec<_> = set.iter_index().collect();
  let expected_indices =
    vec![Three::A.into(), Three::B.into(), Three::C.into()];
  assert_eq!(
    collected_indices, expected_indices,
    "Expected full index iteration for new_all set"
  );
}

#[test]
fn test_set_member_true() {
  let mut set = EnumSet::<Three>::new();

  set.set(Three::B, true);
  assert!(
    set.any(),
    "Expected set to have any members after setting one"
  );
  assert!(
    !set.all(),
    "Expected set to not have all members after setting one"
  );
  assert_eq!(
    set.count(),
    1,
    "Expected set to have count of 1 after setting one member"
  );
  assert!(
    set.contains(Three::B),
    "Expected set to contain Three::B after setting it"
  );
  assert!(
    !set.contains(Three::A),
    "Expected set to not contain Three::A"
  );
  assert!(
    !set.contains(Three::C),
    "Expected set to not contain Three::C"
  );
}

#[test]
fn test_set_member_false() {
  let mut set = EnumSet::<Three>::new_all();

  // Test setting an already set member to false
  set.set(Three::B, false);
  assert!(
    !set.contains(Three::B),
    "Expected set to not contain Three::B after unsetting it"
  );
  assert_eq!(
    set.count(),
    2,
    "Expected count to be 2 after unsetting one member"
  );
}

#[test]
fn test_insert() {
  let mut set = EnumSet::<Three>::new();

  // Test insert returns false for new member
  let was_present = set.insert(Three::B);
  assert!(
    !was_present,
    "Expected insert() to return false for new member"
  );
  assert!(
    set.contains(Three::B),
    "Expected set to contain Three::B after insert"
  );
  assert_eq!(set.count(), 1, "Expected count to be 1 after insert");

  // Test insert returns true for existing member
  let was_present = set.insert(Three::B);
  assert!(
    was_present,
    "Expected insert() to return true for existing member"
  );
  assert!(
    set.contains(Three::B),
    "Expected set to still contain Three::B after second insert"
  );
  assert_eq!(
    set.count(),
    1,
    "Expected count to remain 1 after inserting existing member"
  );

  // Test insert_by_index
  let was_present = set.insert_by_index(Three::C.into());
  assert!(
    !was_present,
    "Expected insert_by_index() to return false for new member"
  );
  assert!(
    set.contains_index(Three::C.into()),
    "Expected set to contain index C after insert_by_index"
  );
  assert_eq!(
    set.count(),
    2,
    "Expected count to be 2 after inserting second member"
  );
}

#[test]
fn test_remove() {
  let mut set = EnumSet::<Three>::new_all();

  // Add some members first
  set.insert(Three::A);
  set.insert(Three::B);
  set.insert(Three::C);
  assert_eq!(
    set.count(),
    3,
    "Expected count to be 3 after adding all members"
  );

  // Test remove returns true for existing member
  let was_present = set.remove(Three::B);
  assert!(
    was_present,
    "Expected remove() to return true for existing member"
  );
  assert!(
    !set.contains(Three::B),
    "Expected set to not contain Three::B after remove"
  );
  assert_eq!(
    set.count(),
    2,
    "Expected count to be 2 after removing one member"
  );

  // Test remove returns false for non-existing member
  let was_present = set.remove(Three::B);
  assert!(
    !was_present,
    "Expected remove() to return false for non-existing member"
  );
  assert_eq!(
    set.count(),
    2,
    "Expected count to remain 2 after removing non-existing member"
  );

  // Test remove_by_index
  let was_present = set.remove_by_index(Three::A.into());
  assert!(
    was_present,
    "Expected remove_by_index() to return true for existing member"
  );
  assert!(
    !set.contains_index(Three::A.into()),
    "Expected set to not contain index A after remove_by_index"
  );
  assert_eq!(
    set.count(),
    1,
    "Expected count to be 1 after removing second member"
  );

  // Test remove_by_index returns false for non-existing member
  let was_present = set.remove_by_index(Three::A.into());
  assert!(
    !was_present,
    "Expected remove_by_index() to return false for non-existing member"
  );
  assert_eq!(
    set.count(),
    1,
    "Expected count to remain 1 after removing non-existing member"
  );
}

#[test]
fn test_clear() {
  let mut set = EnumSet::<Three>::new();

  // Add all members
  set.insert(Three::A);
  set.insert(Three::B);
  set.insert(Three::C);
  assert!(set.all(), "Expected set to have all members before clear");
  assert_eq!(set.count(), 3, "Expected count to be 3 before clear");

  // Clear the set
  set.clear();
  assert!(
    !set.any(),
    "Expected set to not have any members after clear"
  );
  assert!(
    !set.all(),
    "Expected set to not have all members after clear"
  );
  assert_eq!(set.count(), 0, "Expected count to be 0 after clear");
  assert!(
    !set.contains(Three::A),
    "Expected set to not contain Three::A after clear"
  );
  assert!(
    !set.contains(Three::B),
    "Expected set to not contain Three::B after clear"
  );
  assert!(
    !set.contains(Three::C),
    "Expected set to not contain Three::C after clear"
  );
}

#[test]
fn test_all_state() {
  let mut set = EnumSet::<Three>::new();

  // Gradually add all members
  assert!(!set.all(), "Expected set to not have all members initially");

  set.insert(Three::A);
  assert!(
    !set.all(),
    "Expected set to not have all members with 1/3 members"
  );

  set.insert(Three::B);
  assert!(
    !set.all(),
    "Expected set to not have all members with 2/3 members"
  );

  set.insert(Three::C);
  assert!(
    set.all(),
    "Expected set to have all members with 3/3 members"
  );

  // Remove one member
  set.remove(Three::B);
  assert!(
    !set.all(),
    "Expected set to not have all members after removing one member"
  );
}

#[test]
fn test_index_trait() {
  let mut set = EnumSet::<Three>::new();

  // Test indexing empty set
  assert!(
    !set[Three::A],
    "Expected index operator to return false for empty set"
  );
  assert!(
    !set[Three::B],
    "Expected index operator to return false for empty set"
  );
  assert!(
    !set[Three::C],
    "Expected index operator to return false for empty set"
  );

  // Add a member and test indexing
  set.insert(Three::B);
  assert!(
    !set[Three::A],
    "Expected index operator to return false for unset member"
  );
  assert!(
    set[Three::B],
    "Expected index operator to return true for set member"
  );
  assert!(
    !set[Three::C],
    "Expected index operator to return false for unset member"
  );
}

#[test]
fn test_iteration_set_members() {
  let mut set = EnumSet::<Three>::new();

  // Test iteration on empty set
  let collected: Vec<_> = set.iter().collect();
  assert_eq!(collected, vec![], "Expected empty iteration for empty set");
  let (min, max) = set.iter().size_hint();
  assert_eq!(min, 0, "Expected minimum size hint to be 0 for empty set");
  assert!(
    max.is_some(),
    "Expected maximum size hint to be Some(_) for empty set"
  );

  // Add members non-contiguously
  set.insert(Three::A);
  set.insert(Three::C);

  // Test iteration yields members in order
  let collected: Vec<_> = set.iter().collect();
  assert_eq!(
    collected,
    vec![Three::A, Three::C],
    "Expected iteration to yield members in order"
  );
  assert_eq!(
    set.iter().size_hint(),
    (2, Some(2)),
    "Expected correct size hint for set with 2 members"
  );

  // Test index iteration
  let collected_indices: Vec<_> = set.iter_index().collect();
  let expected_indices = vec![Three::A.into(), Three::C.into()];
  assert_eq!(
    collected_indices, expected_indices,
    "Expected index iteration to yield indices in order"
  );

  // Add remaining member to test full iteration
  set.insert(Three::B);
  let collected: Vec<_> = set.iter().collect();
  assert_eq!(
    collected,
    vec![Three::A, Three::B, Three::C],
    "Expected full iteration to yield all members in order"
  );
  assert_eq!(
    set.iter().size_hint(),
    (3, Some(3)),
    "Expected correct size hint for full set"
  );
}

#[test]
fn test_iterator_exact_size_and_double_ended() {
  let mut set = EnumSet::<Three>::new();
  set.insert(Three::A);
  set.insert(Three::C);

  let mut iter = set.iter();

  // Test size_hint
  assert_eq!(
    iter.size_hint(),
    (2, Some(2)),
    "Expected correct size hint lower bound"
  );

  // Test Iterator::nth
  assert_eq!(
    iter.nth(1),
    Some(Three::C),
    "Expected nth(1) to return second element"
  );

  // Test iterator count
  assert_eq!(set.iter().count(), 2, "Expected iterator count to be 2");

  // Test empty iterator traits
  let empty_set = EnumSet::<Three>::new();
  let empty_iter = empty_set.iter();
  assert_eq!(
    empty_iter.size_hint(),
    (0, Some(0)),
    "Expected empty iterator lower bound to be 0"
  );
}

#[test]
fn test_index_iterator_exact_size_and_double_ended() {
  let mut set = EnumSet::<Three>::new();
  set.insert(Three::A);
  set.insert(Three::C);

  let mut iter = set.iter_index();

  // Test size_hint for iter_index
  assert_eq!(
    iter.size_hint(),
    (2, Some(2)),
    "Expected correct size hint lower bound for iter_index"
  );

  // Test Iterator::nth for iter_index
  assert_eq!(
    iter.nth(1),
    Some(Three::C.into()),
    "Expected nth(1) to return second index"
  );

  // Test iterator count for iter_index
  assert_eq!(
    set.iter_index().count(),
    2,
    "Expected iter_index count to be 2"
  );

  // Test empty iter_index traits
  let empty_set = EnumSet::<Three>::new();
  let empty_iter = empty_set.iter_index();
  assert_eq!(
    empty_iter.size_hint(),
    (0, Some(0)),
    "Expected empty iter_index lower bound to be 0"
  );
}

#[test]
fn test_iterator_partial_consumption() {
  let mut set = EnumSet::<Three>::new();
  set.insert(Three::A);
  set.insert(Three::B);
  set.insert(Three::C);

  let mut iter = set.iter();

  // Consume first element
  assert_eq!(iter.next(), Some(Three::A), "Expected first element");

  // Test remaining elements
  let remaining: Vec<_> = iter.collect();
  assert_eq!(
    remaining,
    vec![Three::B, Three::C],
    "Expected remaining elements after partial consumption"
  );
}

#[test]
fn test_iterator_single_element() {
  let mut set = EnumSet::<Three>::new();
  set.insert(Three::B);

  let collected: Vec<_> = set.iter().collect();
  assert_eq!(
    collected,
    vec![Three::B],
    "Expected single element iteration"
  );

  let mut iter = set.iter();
  assert_eq!(iter.next(), Some(Three::B), "Expected single element");
  assert_eq!(iter.next(), None, "Expected iterator to be exhausted");

  // Test single element iter_index
  let collected_indices: Vec<_> = set.iter_index().collect();
  assert_eq!(
    collected_indices,
    vec![Three::B.into()],
    "Expected single element index iteration"
  );
}

#[test]
fn test_iterator_full_set() {
  let mut set = EnumSet::<Three>::new();
  set.insert(Three::A);
  set.insert(Three::B);
  set.insert(Three::C);

  // Test that full set iteration works correctly
  let collected: Vec<_> = set.iter().collect();
  assert_eq!(
    collected,
    vec![Three::A, Three::B, Three::C],
    "Expected full set iteration"
  );

  let collected_indices: Vec<_> = set.iter_index().collect();
  let expected_indices =
    vec![Three::A.into(), Three::B.into(), Three::C.into()];
  assert_eq!(
    collected_indices, expected_indices,
    "Expected full set index iteration"
  );

  // Test iterator traits on full set
  let iter = set.iter();
  assert_eq!(
    iter.size_hint(),
    (3, Some(3)),
    "Expected correct size hint lower bound for full set"
  );

  let iter_index = set.iter_index();
  assert_eq!(
    iter_index.size_hint(),
    (3, Some(3)),
    "Expected correct size hint lower bound for full set iter_index"
  );
}

#[test]
fn test_empty_size_hint() {
  let empty_set = EnumSet::<Seventeen>::new();
  assert_eq!(
    empty_set.iter().size_hint(),
    (0, Some(9)),
    "Expected empty set iterator to have bounds based on empty first word and the maximum bits in the remaining words"
  );
}

#[test]
fn test_full_size_hint() {
  let full_set = EnumSet::<Seventeen>::new_all();
  assert_eq!(
    full_set.iter().size_hint(),
    (8, Some(17)),
    "Expected full set iterator to have bounds based on full first word and the maximum bits in the remaining words"
  );
}

#[test]
fn test_count_and_any() {
  let mut set = EnumSet::<Three>::new();

  // Test empty set
  assert_eq!(set.count(), 0, "Expected count to be 0 for empty set");
  assert!(!set.any(), "Expected any() to return false for empty set");

  // Add one member
  set.insert(Three::A);
  assert_eq!(
    set.count(),
    1,
    "Expected count to be 1 after adding one member"
  );
  assert!(
    set.any(),
    "Expected any() to return true after adding one member"
  );

  // Add another member
  set.insert(Three::C);
  assert_eq!(
    set.count(),
    2,
    "Expected count to be 2 after adding two members"
  );
  assert!(set.any(), "Expected any() to return true with two members");

  // Add all members
  set.insert(Three::B);
  assert_eq!(
    set.count(),
    3,
    "Expected count to be 3 after adding all members"
  );
  assert!(set.any(), "Expected any() to return true with all members");

  // Remove members one by one
  set.remove(Three::A);
  assert_eq!(
    set.count(),
    2,
    "Expected count to be 2 after removing one member"
  );
  assert!(
    set.any(),
    "Expected any() to return true with remaining members"
  );

  set.remove(Three::B);
  assert_eq!(
    set.count(),
    1,
    "Expected count to be 1 after removing two members"
  );
  assert!(
    set.any(),
    "Expected any() to return true with one remaining member"
  );

  set.remove(Three::C);
  assert_eq!(
    set.count(),
    0,
    "Expected count to be 0 after removing all members"
  );
  assert!(
    !set.any(),
    "Expected any() to return false after removing all members"
  );
}

#[test]
fn test_set_by_index() {
  let mut set = EnumSet::<Three>::new();

  // Test setting by index to true
  set.set_by_index(Three::A.into(), true);
  assert!(
    set.contains_index(Three::A.into()),
    "Expected set to contain index A after set_by_index(true)"
  );
  assert_eq!(
    set.count(),
    1,
    "Expected count to be 1 after setting one index"
  );

  // Test setting by index to false
  set.set_by_index(Three::A.into(), false);
  assert!(
    !set.contains_index(Three::A.into()),
    "Expected set to not contain index A after set_by_index(false)"
  );
  assert_eq!(
    set.count(),
    0,
    "Expected count to be 0 after unsetting index"
  );

  // Test setting multiple indices
  set.set_by_index(Three::A.into(), true);
  set.set_by_index(Three::C.into(), true);
  assert!(
    set.contains_index(Three::A.into()),
    "Expected set to contain index A"
  );
  assert!(
    !set.contains_index(Three::B.into()),
    "Expected set to not contain index B"
  );
  assert!(
    set.contains_index(Three::C.into()),
    "Expected set to contain index C"
  );
  assert_eq!(
    set.count(),
    2,
    "Expected count to be 2 after setting two indices"
  );
}
