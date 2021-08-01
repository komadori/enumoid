use crate::test::types::Three;
use enumoid::EnumVec;
use enumoid::Size;

#[test]
fn test_vec() {
  let mut vec = EnumVec::<Three, u16>::new();
  assert_eq!(vec.size(), Size::EMPTY);
  vec.push(100);
  vec.push(200);
  assert_eq!(vec[Three::A], 100);
  assert_eq!(vec[Three::B], 200);
  assert_eq!(vec.size(), Size::from_last_key(Three::B));
  vec.push(300);
  assert_eq!(vec.size(), Size::from_last_key(Three::C));
  vec.pop();
  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(collected, vec![(Three::A, &100), (Three::B, &200)]);
}
