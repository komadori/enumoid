use crate::test::types::{Seventeen, Sixteen, Three, Zero};
use enumoid::EnumFlags;
use enumoid::EnumMap;
use enumoid::EnumOptionMap;
use enumoid::EnumVec;
use enumoid::Enumoid;

#[test]
fn test_zero() {
  assert_eq!(Zero::SIZE, 0);
  assert_eq!(std::mem::size_of::<EnumFlags::<Zero>>(), 0);
  assert_eq!(std::mem::size_of::<EnumMap::<Zero, u8>>(), 0);
  assert_eq!(std::mem::size_of::<EnumOptionMap::<Zero, u8>>(), 0);
  assert_eq!(std::mem::size_of::<EnumVec::<Zero, u8>>(), 1);
}

#[test]
fn test_three() {
  assert_eq!(Three::SIZE, 3);
  assert_eq!(std::mem::size_of::<EnumFlags::<Three>>(), 1);
  assert_eq!(std::mem::size_of::<EnumMap::<Three, u8>>(), 3);
  assert_eq!(std::mem::size_of::<EnumOptionMap::<Three, u8>>(), 4);
  assert_eq!(std::mem::size_of::<EnumVec::<Three, u8>>(), 4);
}

#[test]
fn test_sixteen() {
  assert_eq!(Sixteen::SIZE, 16);
  assert_eq!(std::mem::size_of::<EnumFlags::<Sixteen>>(), 2);
  assert_eq!(std::mem::size_of::<EnumMap::<Sixteen, u8>>(), 16);
  assert_eq!(std::mem::size_of::<EnumOptionMap::<Sixteen, u8>>(), 18);
  assert_eq!(std::mem::size_of::<EnumVec::<Sixteen, u8>>(), 17);
}

#[test]
fn test_seventeen() {
  assert_eq!(Seventeen::SIZE, 17);
  assert_eq!(std::mem::size_of::<EnumFlags::<Seventeen>>(), 3);
  assert_eq!(std::mem::size_of::<EnumMap::<Seventeen, u8>>(), 17);
  assert_eq!(std::mem::size_of::<EnumOptionMap::<Seventeen, u8>>(), 20);
  assert_eq!(std::mem::size_of::<EnumVec::<Seventeen, u8>>(), 18);
}
