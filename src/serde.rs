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

struct OptMapSerdeVisitor<
  T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
  V,
  BitsetWord: BitsetWordTrait,
  R,
> {
  #[allow(clippy::type_complexity)]
  marker: marker::PhantomData<fn(EnumOptionMap<T, V, BitsetWord>) -> R>,
}

impl<
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord>,
    V,
    BitsetWord: BitsetWordTrait,
    R,
  > OptMapSerdeVisitor<T, V, BitsetWord, R>
{
  fn new() -> Self {
    OptMapSerdeVisitor {
      marker: marker::PhantomData,
    }
  }
}

impl<'de, K, V, BitsetWord: BitsetWordTrait, R> de::Visitor<'de>
  for OptMapSerdeVisitor<K, V, BitsetWord, R>
where
  K: EnumArrayHelper<V> + EnumSetHelper<BitsetWord> + de::Deserialize<'de>,
  V: de::Deserialize<'de>,
  R: TryFrom<EnumOptionMap<K, V, BitsetWord>>,
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

impl<
    T: EnumArrayHelper<V> + EnumSetHelper<u8> + ser::Serialize,
    V: ser::Serialize,
  > ser::Serialize for EnumOptionMap<T, V>
{
  fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
  where
    S: ser::Serializer,
  {
    use ser::SerializeMap;
    let mut map = ser.serialize_map(Some(self.count()))?;
    for (k, v) in self.iter() {
      map.serialize_entry(&k, v)?;
    }
    map.end()
  }
}

impl<
    'de,
    T: EnumArrayHelper<V> + EnumSetHelper<BitsetWord> + de::Deserialize<'de>,
    V: de::Deserialize<'de>,
    BitsetWord: BitsetWordTrait,
  > de::Deserialize<'de> for EnumOptionMap<T, V, BitsetWord>
{
  fn deserialize<D>(de: D) -> Result<Self, D::Error>
  where
    D: de::Deserializer<'de>,
  {
    de.deserialize_map(OptMapSerdeVisitor::<
      T,
      V,
      BitsetWord,
      EnumOptionMap<T, V, BitsetWord>,
    >::new())
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
    de.deserialize_map(OptMapSerdeVisitor::<T, V, u8, EnumMap<T, V>>::new())
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
    de.deserialize_map(OptMapSerdeVisitor::<T, V, u8, EnumVec<T, V>>::new())
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

  fn visit_seq<S>(self, mut access: S) -> Result<Self::Value, S::Error>
  where
    S: de::SeqAccess<'de>,
  {
    let mut set = EnumSet::new();
    while let Some(key) = access.next_element::<K>()? {
      set.set(key, true);
    }
    match R::try_from(set) {
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
    use ser::SerializeSeq;
    let mut set = ser.serialize_seq(Some(self.count()))?;
    for k in self.iter() {
      set.serialize_element(&k)?;
    }
    set.end()
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
    de.deserialize_seq(
      SetSerdeVisitor::<T, BitsetWord, EnumSet<T, BitsetWord>>::new(),
    )
  }
}
