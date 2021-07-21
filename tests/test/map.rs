use crate::test::types::Three;
use enumoid::EnumMap;

#[test]
fn test_map() {
  let mut map = EnumMap::<Three, u16>::new();
  assert_eq!(*map.get(Three::B), 0);
  map[Three::B] = 200;
  assert_eq!(*map.get(Three::B), 200);
  let collected: Vec<_> = map.iter().collect();
  assert_eq!(
    collected,
    vec![(Three::A, &0), (Three::B, &200), (Three::C, &0)]
  );
}
