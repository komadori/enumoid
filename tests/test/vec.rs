use crate::test::types::Three;
use enumoid::EnumSize;
use enumoid::EnumVec;

use super::types::Sixteen;

#[test]
fn test_vec() {
  let mut vec = EnumVec::<Three, u16>::new();
  assert_eq!(vec.size(), EnumSize::EMPTY);
  assert_eq!(vec.pop(), None);
  vec.push(100);
  vec.push(200);
  assert_eq!(vec[Three::A], 100);
  assert_eq!(vec[Three::B], 200);
  assert_eq!(vec.size(), EnumSize::from_last(Three::B));
  vec.push(300);
  assert_eq!(vec.size(), EnumSize::from_last(Three::C));
  assert_eq!(vec.pop(), Some(300));
  assert_eq!(vec.get(Three::C), None);
  vec[Three::B] += 1;
  assert_eq!(vec[Three::B], 201);
  let collected: Vec<_> = vec.iter().collect();
  assert_eq!(collected, vec![(Three::A, &100), (Three::B, &201)]);
}

#[test]
fn test_swap() {
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
fn test_push_panic() {
  let mut vec = EnumVec::<Three, u16>::new();
  vec.push(100);
  vec.push(200);
  vec.push(300);
  vec.push(400);
}
