use crate::test::types::Three;
use enumoid::EnumVec;
use enumoid::Size;

#[test]
fn test_vec() {
  let mut vec = EnumVec::<Three, u16>::new();
  assert_eq!(vec.size(), Size::EMPTY);
  assert_eq!(vec.pop(), None);
  vec.push(100);
  vec.push(200);
  assert_eq!(vec[Three::A], 100);
  assert_eq!(vec[Three::B], 200);
  assert_eq!(vec.size(), Size::from_last_key(Three::B));
  vec.push(300);
  assert_eq!(vec.size(), Size::from_last_key(Three::C));
  assert_eq!(vec.pop(), Some(300));
  assert_eq!(vec.get(Three::C), None);
  vec[Three::B] += 1;
  assert_eq!(vec[Three::B], 201);
  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(collected, vec![(Three::A, &100), (Three::B, &201)]);
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_push_panic() {
  let mut vec = EnumVec::<Three, u16>::new();
  vec.push(100);
  vec.push(200);
  vec.push(300);
  vec.push(400);
}
