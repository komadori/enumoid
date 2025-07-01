use crate::test::types::Three;
use enumoid::EnumOptionMap;
use enumoid::EnumSize;

#[test]
fn test_option_map() {
  let mut map = EnumOptionMap::<Three, u16>::new();
  assert!(map.is_empty());
  assert_eq!(map.is_vec(), EnumSize::from_usize(0));
  assert_eq!(map.set(Three::B, Some(200)), None);
  assert!(!map.is_empty());
  assert_eq!(map.is_vec(), None);
  assert_eq!(map.set(Three::A, Some(1)), None);
  assert_eq!(map.set(Three::A, Some(99)), Some(1));
  *map.get_mut(Three::A).unwrap() += 1;
  assert!(!map.is_full());
  assert_eq!(map.is_vec(), EnumSize::from_usize(2));
  map.set(Three::C, Some(300));
  assert!(map.is_full());
  assert_eq!(map.is_vec(), EnumSize::from_usize(3));
  assert_eq!(map.get(Three::A), Some(&100));
  assert_eq!(map.get(Three::B), Some(&200));
  assert_eq!(map.get(Three::C), Some(&300));
  map.set(Three::B, None);
  let mut iter = map.iter_mut();
  *iter.next().unwrap().1 -= 1;
  *iter.next().unwrap().1 -= 1;
  assert_eq!(iter.next(), None);
  let collected: Vec<_> = map.iter().collect();
  assert_eq!(collected, vec![(Three::A, &99), (Three::C, &299)]);
}
