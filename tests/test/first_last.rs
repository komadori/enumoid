use crate::test::types::{Seventeen, Sixteen, Three};
use enumoid::Enumoid1;

#[test]
fn test_first() {
  assert_eq!(Three::FIRST, Three::A);
  assert_eq!(Sixteen::FIRST, Sixteen::A);
  assert_eq!(Seventeen::FIRST, Seventeen::A);
}

#[test]
fn test_last() {
  assert_eq!(Three::LAST, Three::C);
  assert_eq!(Sixteen::LAST, Sixteen::P);
  assert_eq!(Seventeen::LAST, Seventeen::Q);
}
