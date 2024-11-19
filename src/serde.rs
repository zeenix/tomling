use crate::{
    array::{self, Array},
    table::{self, Table},
    Value,
};
use serde::de::{
    self, DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Visitor,
};

/// Deserialize a TOML document from a string.
pub fn from_str<'de, T>(s: &'de str) -> Result<T, de::value::Error>
where
    T: de::Deserialize<'de>,
{
    let value = crate::parse(s).map_err(|_| de::Error::custom("Invalid TOML"))?;

    T::deserialize(&Value::Table(value))
}

impl<'de, 'a> Deserializer<'de> for &'a Value<'de> {
    type Error = de::value::Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(s) => visitor.visit_borrowed_str(s),
            Value::Integer(i) => visitor.visit_i64(*i),
            Value::Float(f) => visitor.visit_f64(*f),
            Value::Boolean(b) => visitor.visit_bool(*b),
            Value::Array(arr) => visitor.visit_seq(SeqDeserializer::new(arr)),
            Value::Table(table) => visitor.visit_map(MapDeserializer::new(table)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(s) => visitor.visit_borrowed_str(s),
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
        match self {
            Value::Integer(i) => visitor.visit_i64(*i),
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
        match self {
            Value::Float(f) => visitor.visit_f64(*f),
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
        match self {
            Value::Boolean(b) => visitor.visit_bool(*b),
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
        match self {
            Value::Array(arr) => visitor.visit_seq(SeqDeserializer::new(arr)),
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
        match self {
            Value::Table(table) => visitor.visit_map(MapDeserializer::new(table)),
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
        visitor.visit_some(self)
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

    serde::forward_to_deserialize_any! {
        i8 i16 i32 i128 u8 u16 u32 u64 u128 f32
        char string bytes byte_buf unit unit_struct
        tuple tuple_struct struct identifier enum ignored_any
    }
}

struct SeqDeserializer<'de, 'a> {
    iter: array::Iter<'a, 'de>,
}

impl<'de, 'a> SeqDeserializer<'de, 'a> {
    fn new(array: &'a Array<'de>) -> Self {
        SeqDeserializer { iter: array.iter() }
    }
}

impl<'de, 'a> SeqAccess<'de> for SeqDeserializer<'de, 'a> {
    type Error = de::value::Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'de>,
    {
        self.iter
            .next()
            .map_or(Ok(None), |value| seed.deserialize(value).map(Some))
    }
}

struct MapDeserializer<'de, 'a> {
    iter: table::Iter<'a, 'de>,
    value: Option<&'a Value<'de>>,
}

impl<'de, 'a> MapDeserializer<'de, 'a> {
    fn new(table: &'a Table<'de>) -> Self {
        MapDeserializer {
            iter: table.iter(),
            value: None,
        }
    }
}

impl<'de, 'a> MapAccess<'de> for MapDeserializer<'de, 'a> {
    type Error = de::value::Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            seed.deserialize((*key).into_deserializer()).map(Some)
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(value),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}
