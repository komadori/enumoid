use crate::test::types::{
  CompoundSeven, Seventeen, Sixteen, StructThree, Three,
};
use enumoid::{EnumSize, Enumoid};

#[test]
fn test_first() {
  assert_eq!(Three::FIRST, Three::A);
  assert_eq!(Sixteen::FIRST, Sixteen::A);
  assert_eq!(Seventeen::FIRST, Seventeen::A);
  assert_eq!(StructThree::FIRST, StructThree(Three::A));
  assert_eq!(CompoundSeven::FIRST, CompoundSeven::X(Three::A));
}

#[test]
fn test_last() {
  assert_eq!(Three::LAST, Three::C);
  assert_eq!(Sixteen::LAST, Sixteen::P);
  assert_eq!(Seventeen::LAST, Seventeen::Q);
  assert_eq!(StructThree::LAST, StructThree(Three::C));
  assert_eq!(CompoundSeven::LAST, CompoundSeven::Z(Three::C));
}

#[test]
fn test_next_simple() {
  assert_eq!(Three::A.next(), Some(Three::B));
  assert_eq!(Sixteen::A.next(), Some(Sixteen::B));
  assert_eq!(Seventeen::A.next(), Some(Seventeen::B));
  assert_eq!(StructThree(Three::A).next(), Some(StructThree(Three::B)));
  assert_eq!(
    EnumSize::from_last_key(Sixteen::D).next(Sixteen::A),
    Some(Sixteen::B)
  );
  assert_eq!(CompoundSeven::Y.next(), Some(CompoundSeven::Z(Three::A)))
}

#[test]
fn test_next_boundary() {
  assert_eq!(Three::C.next(), None);
  assert_eq!(Sixteen::P.next(), None);
  assert_eq!(Seventeen::Q.next(), None);
  assert_eq!(StructThree(Three::C).next(), None);
  let size = EnumSize::from_last_key(Sixteen::D);
  assert_eq!(size.next(Sixteen::D), None);
  assert_eq!(CompoundSeven::Z(Three::C).next(), None)
}

#[test]
fn test_next_wrapped() {
  assert_eq!(Three::C.next_wrapped(), Three::A);
  assert_eq!(Sixteen::P.next_wrapped(), Sixteen::A);
  assert_eq!(Seventeen::Q.next_wrapped(), Seventeen::A);
  assert_eq!(StructThree(Three::C).next_wrapped(), StructThree(Three::A));
  let size = EnumSize::from_last_key(Sixteen::D);
  assert_eq!(size.next_wrapped(Sixteen::D), Sixteen::A);
  assert_eq!(
    CompoundSeven::Z(Three::C).next_wrapped(),
    CompoundSeven::X(Three::A)
  )
}

#[test]
fn test_prev_simple() {
  assert_eq!(Three::B.prev(), Some(Three::A));
  assert_eq!(Sixteen::B.prev(), Some(Sixteen::A));
  assert_eq!(Seventeen::B.prev(), Some(Seventeen::A));
  assert_eq!(StructThree(Three::B).prev(), Some(StructThree(Three::A)));
  let size = EnumSize::from_last_key(Sixteen::D);
  assert_eq!(size.prev(Sixteen::B), Some(Sixteen::A));
  assert_eq!(CompoundSeven::Y.prev_wrapped(), CompoundSeven::X(Three::C))
}

#[test]
fn test_prev_boundary() {
  assert_eq!(Three::A.prev(), None);
  assert_eq!(Sixteen::A.prev(), None);
  assert_eq!(Seventeen::A.prev(), None);
  assert_eq!(StructThree(Three::A).prev(), None);
  let size = EnumSize::from_last_key(Sixteen::D);
  assert_eq!(size.prev(Sixteen::A), None);
  assert_eq!(CompoundSeven::X(Three::A).prev(), None);
}

#[test]
fn test_prev_wrapped() {
  assert_eq!(Three::A.prev_wrapped(), Three::C);
  assert_eq!(Sixteen::A.prev_wrapped(), Sixteen::P);
  assert_eq!(Seventeen::A.prev_wrapped(), Seventeen::Q);
  assert_eq!(StructThree(Three::A).prev_wrapped(), StructThree(Three::C));
  let size = EnumSize::from_last_key(Sixteen::D);
  assert_eq!(size.prev_wrapped(Sixteen::A), Sixteen::D);
  assert_eq!(
    CompoundSeven::X(Three::A).prev_wrapped(),
    CompoundSeven::Z(Three::C)
  );
}

#[test]
fn test_iter() {
  let collected3: Vec<_> = Three::iter().collect();
  assert_eq!(collected3, vec![Three::A, Three::B, Three::C]);
  let collected7: Vec<_> = CompoundSeven::iter().collect();
  assert_eq!(
    collected7,
    vec![
      CompoundSeven::X(Three::A),
      CompoundSeven::X(Three::B),
      CompoundSeven::X(Three::C),
      CompoundSeven::Y,
      CompoundSeven::Z(Three::A),
      CompoundSeven::Z(Three::B),
      CompoundSeven::Z(Three::C),
    ]
  );
}
