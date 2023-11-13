use crate::test::types::{Seventeen, Sixteen, Three};
use enumoid::{Enumoid, Size};

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

#[test]
fn test_next_simple() {
  assert_eq!(Three::A.next(), Some(Three::B));
  assert_eq!(Sixteen::A.next(), Some(Sixteen::B));
  assert_eq!(Seventeen::A.next(), Some(Seventeen::B));
  assert_eq!(
    Size::from_last_key(Sixteen::D).next(Sixteen::A),
    Some(Sixteen::B)
  );
}

#[test]
fn test_next_boundary() {
  assert_eq!(Three::C.next(), None);
  assert_eq!(Sixteen::P.next(), None);
  assert_eq!(Seventeen::Q.next(), None);
  let size = Size::from_last_key(Sixteen::D);
  assert_eq!(size.next(Sixteen::D), None);
}

#[test]
fn test_next_wrapped() {
  assert_eq!(Three::C.next_wrapped(), Three::A);
  assert_eq!(Sixteen::P.next_wrapped(), Sixteen::A);
  assert_eq!(Seventeen::Q.next_wrapped(), Seventeen::A);
  let size = Size::from_last_key(Sixteen::D);
  assert_eq!(size.next_wrapped(Sixteen::D), Sixteen::A);
}

#[test]
fn test_prev_simple() {
  assert_eq!(Three::B.prev(), Some(Three::A));
  assert_eq!(Sixteen::B.prev(), Some(Sixteen::A));
  assert_eq!(Seventeen::B.prev(), Some(Seventeen::A));
  let size = Size::from_last_key(Sixteen::D);
  assert_eq!(size.prev(Sixteen::B), Some(Sixteen::A));
}

#[test]
fn test_prev_boundary() {
  assert_eq!(Three::A.prev(), None);
  assert_eq!(Sixteen::A.prev(), None);
  assert_eq!(Seventeen::A.prev(), None);
  let size = Size::from_last_key(Sixteen::D);
  assert_eq!(size.prev(Sixteen::A), None);
}

#[test]
fn test_prev_wrapped() {
  assert_eq!(Three::A.prev_wrapped(), Three::C);
  assert_eq!(Sixteen::A.prev_wrapped(), Sixteen::P);
  assert_eq!(Seventeen::A.prev_wrapped(), Seventeen::Q);
  let size = Size::from_last_key(Sixteen::D);
  assert_eq!(size.prev_wrapped(Sixteen::A), Sixteen::D);
}

#[test]
fn test_iter() {
  let collected3: Vec<_> = Three::iter().collect();
  assert_eq!(collected3, vec![Three::A, Three::B, Three::C]);
}
