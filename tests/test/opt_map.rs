use crate::test::types::Three;
use enumoid::EnumOptionMap;
use enumoid::Size;

#[test]
fn test_option_map() {
  let mut map = EnumOptionMap::<Three, u16>::new();
  assert_eq!(map.is_empty(), true);
  assert_eq!(map.is_vec(), Size::from_usize(0));
  map.set(Three::B, Some(200));
  assert_eq!(map.is_empty(), false);
  assert_eq!(map.is_vec(), None);
  map.set(Three::A, Some(99));
  *map.get_mut(Three::A).unwrap() += 1;
  assert_eq!(map.is_full(), false);
  assert_eq!(map.is_vec(), Size::from_usize(2));
  map.set(Three::C, Some(300));
  assert_eq!(map.is_full(), true);
  assert_eq!(map.is_vec(), Size::from_usize(3));
  assert_eq!(map.get(Three::A), Some(&100));
  assert_eq!(map.get(Three::B), Some(&200));
  assert_eq!(map.get(Three::C), Some(&300));
}
