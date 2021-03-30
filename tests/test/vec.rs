use crate::test::types::Three;
use enumoid::EnumVec;

#[test]
fn test_vec() {
  let mut vec = EnumVec::<Three, u16>::new();
  assert_eq!(vec.last_key(), None);
  vec.push(100);
  vec.push(200);
  assert_eq!(vec.last_key(), Some(Three::B));
  assert_eq!(vec[Three::A], 100);
  assert_eq!(vec[Three::B], 200);
}
