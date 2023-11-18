use crate::test::types::Three;
use enumoid::EnumSet;

#[test]
fn test_flags() {
  let mut flags = EnumSet::<Three>::new();
  assert_eq!(flags.any(), false);
  assert_eq!(flags.count(), 0);
  assert_eq!(flags.get(Three::B), false);
  flags.set(Three::B, true);
  assert_eq!(flags.any(), true);
  assert_eq!(flags.count(), 1);
  assert_eq!(flags.get(Three::B), true);
  let collected: Vec<_> = flags.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, false), (Three::B, true), (Three::C, false)]
  );
}
