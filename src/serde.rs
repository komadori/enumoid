use crate::base::EnumArrayHelper;
use crate::base::EnumSetHelper;
use crate::map::EnumMap;
use crate::opt_map::EnumOptionMap;
use crate::set::EnumSet;
use crate::sub_base::BitsetWordTrait;
use crate::sub_base::RawSizeWord;
use crate::vec::EnumVec;
use serde::{de, ser};
use std::convert::TryFrom;
use std::fmt;
use std::marker;

struct OptMapSerdeVisitor<T: EnumArrayHelper<V> + EnumSetHelper<u8>, V, R> {
  marker: marker::PhantomData<fn(EnumOptionMap<T, V>) -> R>,
}

impl<T: EnumArrayHelper<V> + EnumSetHelper<u8>, V, R>
  OptMapSerdeVisitor<T, V, R>
{
  fn new() -> Self {
    OptMapSerdeVisitor {
      marker: marker::PhantomData,
    }
  }
}

impl<'de, K, V, R> de::Visitor<'de> for OptMapSerdeVisitor<K, V, R>
where
  K: EnumArrayHelper<V> + EnumSetHelper<u8> + de::Deserialize<'de>,
  V: de::Deserialize<'de>,
  R: TryFrom<EnumOptionMap<K, V>>,
{
  type Value = R;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("Enumoid indexed container")
  }

  fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
  where
    M: de::MapAccess<'de>,
  {
    let mut map = EnumOptionMap::new();
    while let Some((key, value)) = access.next_entry::<K, V>()? {
      map.set(key, Some(value));
    }
    match R::try_from(map) {
      Ok(r) => Ok(r),
      Err(_) => Err(de::Error::custom("malformed")),
    }
  }
}

impl<T: EnumArrayHelper<V> + serde::ser::Serialize, V: serde::ser::Serialize>
  serde::ser::Serialize for EnumMap<T, V>
{
  fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
  where
    S: serde::ser::Serializer,
  {
    use serde::ser::SerializeMap;
    let mut map = ser.serialize_map(Some(T::SIZE))?;
    for (k, v) in self.iter() {
      map.serialize_entry(&k, v)?;
    }
    map.end()
  }
}

impl<
    'de,
    T: EnumArrayHelper<V> + EnumSetHelper<u8> + de::Deserialize<'de>,
    V: de::Deserialize<'de>,
  > de::Deserialize<'de> for EnumMap<T, V>
{
  fn deserialize<D>(de: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    de.deserialize_map(OptMapSerdeVisitor::<T, V, EnumMap<T, V>>::new())
  }
}

impl<T: EnumArrayHelper<V> + ser::Serialize, V: ser::Serialize> ser::Serialize
  for EnumVec<T, V>
{
  fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    use ser::SerializeMap;
    let mut map = ser.serialize_map(Some(self.len.as_()))?;
    for (k, v) in self.iter() {
      map.serialize_entry(&k, v)?;
    }
    map.end()
  }
}

impl<
    'de,
    T: EnumArrayHelper<V> + EnumSetHelper<u8> + de::Deserialize<'de>,
    V: de::Deserialize<'de>,
  > de::Deserialize<'de> for EnumVec<T, V>
{
  fn deserialize<D>(de: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    de.deserialize_map(OptMapSerdeVisitor::<T, V, EnumVec<T, V>>::new())
  }
}

struct SetSerdeVisitor<
  T: EnumSetHelper<BitsetWord>,
  BitsetWord: BitsetWordTrait,
  R,
> {
  marker: marker::PhantomData<fn(EnumSet<T, BitsetWord>) -> R>,
}

impl<T: EnumSetHelper<BitsetWord>, BitsetWord: BitsetWordTrait, R>
  SetSerdeVisitor<T, BitsetWord, R>
{
  fn new() -> Self {
    SetSerdeVisitor {
      marker: marker::PhantomData,
    }
  }
}

impl<'de, K, BitsetWord: BitsetWordTrait, R> de::Visitor<'de>
  for SetSerdeVisitor<K, BitsetWord, R>
where
  K: EnumSetHelper<BitsetWord> + de::Deserialize<'de>,
  R: TryFrom<EnumSet<K, BitsetWord>>,
{
  type Value = R;

  fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
    formatter.write_str("Enumoid indexed container")
  }

  fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
  where
    M: de::MapAccess<'de>,
  {
    let mut map = EnumSet::new();
    while let Some((key, value)) = access.next_entry::<K, bool>()? {
      map.set(key, value);
    }
    match R::try_from(map) {
      Ok(r) => Ok(r),
      Err(_) => Err(de::Error::custom("malformed")),
    }
  }
}

impl<
    T: EnumSetHelper<BitsetWord> + ser::Serialize,
    BitsetWord: BitsetWordTrait,
  > ser::Serialize for EnumSet<T, BitsetWord>
{
  fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    use ser::SerializeMap;
    let mut map = ser.serialize_map(Some(T::SIZE))?;
    for (k, v) in self.iter() {
      map.serialize_entry(&k, &v)?;
    }
    map.end()
  }
}

impl<
    'de,
    T: EnumSetHelper<BitsetWord> + de::Deserialize<'de>,
    BitsetWord: BitsetWordTrait,
  > de::Deserialize<'de> for EnumSet<T, BitsetWord>
{
  fn deserialize<D>(de: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    de.deserialize_map(
      SetSerdeVisitor::<T, BitsetWord, EnumSet<T, BitsetWord>>::new(),
    )
  }
}
