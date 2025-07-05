use enumoid::{EnumMap, EnumOptionMap, EnumSet, EnumVec};

use super::types::Three;

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, Default)]
struct TestData {
  value: i32,
  name: String,
}

#[test]
fn test_enum_map_round_trip() {
  let mut map = EnumMap::new();

  map[Three::A] = TestData {
    value: 42,
    name: "answer".to_string(),
  };
  map[Three::B] = TestData {
    value: 100,
    name: "century".to_string(),
  };
  map[Three::C] = TestData {
    value: -5,
    name: "negative".to_string(),
  };

  let ron_str = ron::to_string(&map).expect("Serialization failed");
  let deserialized: EnumMap<Three, TestData> =
    ron::from_str(&ron_str).expect("Deserialization failed");

  assert_eq!(map, deserialized);
  assert_eq!(map[Three::A], deserialized[Three::A]);
  assert_eq!(map[Three::B], deserialized[Three::B]);
  assert_eq!(map[Three::C], deserialized[Three::C]);
}

#[test]
fn test_enum_option_map_round_trip() {
  let mut map = EnumOptionMap::new();
  map.insert(
    Three::A,
    TestData {
      value: 42,
      name: "answer".to_string(),
    },
  );
  map.insert(
    Three::C,
    TestData {
      value: 999,
      name: "large".to_string(),
    },
  );

  let ron_str = ron::to_string(&map).expect("Serialization failed");
  let deserialized: EnumOptionMap<Three, TestData> =
    ron::from_str(&ron_str).expect("Deserialization failed");

  assert_eq!(map, deserialized);
}

#[test]
fn test_enum_option_map_empty_round_trip() {
  let map: EnumOptionMap<Three, i32> = EnumOptionMap::new();

  let ron_str = ron::to_string(&map).expect("Serialization failed");
  let deserialized: EnumOptionMap<Three, i32> =
    ron::from_str(&ron_str).expect("Deserialization failed");

  assert_eq!(map, deserialized);
}

#[test]
fn test_enum_vec_round_trip() {
  let mut vec = EnumVec::new();
  vec.push(TestData {
    value: 10,
    name: "first".to_string(),
  });
  vec.push(TestData {
    value: 20,
    name: "second".to_string(),
  });
  vec.push(TestData {
    value: 30,
    name: "third".to_string(),
  });

  let ron_str = ron::to_string(&vec).expect("Serialization failed");
  let deserialized: EnumVec<Three, TestData> =
    ron::from_str(&ron_str).expect("Deserialization failed");

  assert_eq!(vec, deserialized);
}

#[test]
fn test_enum_vec_empty_round_trip() {
  let vec: EnumVec<Three, i32> = EnumVec::new();

  let ron_str = ron::to_string(&vec).expect("Serialization failed");
  let deserialized: EnumVec<Three, i32> =
    ron::from_str(&ron_str).expect("Deserialization failed");

  assert_eq!(vec, deserialized);
}

#[test]
fn test_enum_set_round_trip() {
  let mut set = EnumSet::new();
  set.insert(Three::A);
  set.insert(Three::C);

  let ron_str = ron::to_string(&set).expect("Serialization failed");
  let deserialized: EnumSet<Three> =
    ron::from_str(&ron_str).expect("Deserialization failed");

  assert_eq!(set, deserialized);
}

#[test]
fn test_enum_set_empty_round_trip() {
  let set: EnumSet<Three> = EnumSet::new();

  let ron_str = ron::to_string(&set).expect("Serialization failed");
  let deserialized: EnumSet<Three> =
    ron::from_str(&ron_str).expect("Deserialization failed");

  assert_eq!(set, deserialized);
}
