use crate::test::types::Three;
use enumoid::EnumSet;

#[test]
fn test_flags() {
  let mut flags = EnumSet::<Three>::new();
  assert!(!flags.any());
  assert_eq!(flags.count(), 0);
  assert!(!flags.contains(Three::B));
  flags.set(Three::B, true);
  assert!(flags.any());
  assert_eq!(flags.count(), 1);
  assert!(flags.contains(Three::B));
  let collected: Vec<_> = flags.iter().collect();
  assert_eq!(collected, vec![Three::B]);
  assert_eq!(flags.iter().size_hint(), (1, Some(1)));
}
