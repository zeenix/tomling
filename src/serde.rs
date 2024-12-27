use alloc::borrow::Cow;

use crate::{
    array::{self, Array},
    table::{self, Table},
    Error, Value,
};
use serde::de::{
    self,
    value::{BorrowedStrDeserializer, StrDeserializer},
    DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Visitor,
};

/// Deserialize a TOML document from a string. Requires the `serde` feature.
pub fn from_str<'de, T>(s: &'de str) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    let value = crate::parse(s)?;

    T::deserialize(ValueDeserializer {
        value: Some(Value::Table(value)),
    })
}

#[derive(Debug)]
struct ValueDeserializer<'de> {
    value: Option<Value<'de>>,
}

impl<'de> Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::String(Cow::Borrowed(s))) => visitor.visit_borrowed_str(s),
            Some(Value::String(Cow::Owned(s))) => visitor.visit_str(&s),
            Some(Value::Integer(i)) => visitor.visit_i64(i),
            Some(Value::Float(f)) => visitor.visit_f64(f),
            Some(Value::Boolean(b)) => visitor.visit_bool(b),
            Some(Value::Array(arr)) => visitor.visit_seq(SeqDeserializer::new(arr)),
            Some(Value::Table(table)) => visitor.visit_map(MapDeserializer::new(table)),
            None => Err(de::Error::custom("value is missing")),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::String(Cow::Borrowed(s))) => visitor.visit_borrowed_str(s),
            Some(Value::String(Cow::Owned(s))) => visitor.visit_str(&s),
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("non-string"),
                &visitor,
            )),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Integer(i)) => visitor.visit_i64(i),
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("non-integer"),
                &visitor,
            )),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Float(f)) => visitor.visit_f64(f),
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("non-float"),
                &visitor,
            )),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Boolean(b)) => visitor.visit_bool(b),
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("non-boolean"),
                &visitor,
            )),
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Array(arr)) => visitor.visit_seq(SeqDeserializer::new(arr)),
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("non-array"),
                &visitor,
            )),
        }
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Table(table)) => visitor.visit_map(MapDeserializer::new(table)),
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("non-map"),
                &visitor,
            )),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(_) => visitor.visit_some(self),
            None => visitor.visit_none(),
        }
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::String(s)) => visitor.visit_enum(s.clone().into_deserializer()),
            // TODO: Support non-unit enums.
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("non-string"),
                &visitor,
            )),
        }
    }

    serde::forward_to_deserialize_any! {
        i8 i16 i32 i128 u8 u16 u32 u64 u128 f32
        char string bytes byte_buf unit unit_struct
        tuple tuple_struct struct identifier ignored_any
    }
}

struct SeqDeserializer<'de> {
    iter: array::IntoIter<'de>,
}

impl<'de> SeqDeserializer<'de> {
    fn new(array: Array<'de>) -> Self {
        SeqDeserializer {
            iter: array.into_iter(),
        }
    }
}

impl<'de> SeqAccess<'de> for SeqDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.iter.next().map_or(Ok(None), |value| {
            let de = ValueDeserializer { value: Some(value) };

            seed.deserialize(de).map(Some)
        })
    }
}

struct MapDeserializer<'de> {
    iter: table::IntoIter<'de>,
    value: Option<Value<'de>>,
}

impl<'de> MapDeserializer<'de> {
    fn new(table: Table<'de>) -> Self {
        MapDeserializer {
            iter: table.into_iter(),
            value: None,
        }
    }
}

impl<'de> MapAccess<'de> for MapDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            match key {
                Cow::Owned(s) => seed.deserialize(StrDeserializer::new(&s).into_deserializer()),
                Cow::Borrowed(s) => {
                    seed.deserialize(BorrowedStrDeserializer::new(s).into_deserializer())
                }
            }
            .map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(ValueDeserializer { value: Some(value) }),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}
