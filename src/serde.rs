use alloc::borrow::Cow;

use crate::{
    array::{self, Array},
    datetime::Offset,
    table::{self, Table},
    Date, Datetime, Error, Value,
};
use serde::de::{
    self,
    value::{BorrowedStrDeserializer, I64Deserializer, StrDeserializer},
    DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Visitor,
};

/// Deserialize a TOML document from a string. Requires the `serde` feature.
pub fn from_str<'de, T>(s: &'de str) -> Result<T, Error>
where
    T: de::Deserialize<'de>,
{
    let value = crate::parse(s)?;

    T::deserialize(&Value::Table(value))
}

impl<'de> Deserializer<'de> for &Value<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(Cow::Borrowed(s)) => visitor.visit_borrowed_str(s),
            Value::String(Cow::Owned(s)) => visitor.visit_str(s),
            Value::Integer(i) => visitor.visit_i64(*i),
            Value::Float(f) => visitor.visit_f64(*f),
            Value::Boolean(b) => visitor.visit_bool(*b),
            Value::Array(arr) => visitor.visit_seq(SeqDeserializer::new(arr)),
            Value::Table(table) => visitor.visit_map(MapDeserializer::new(table)),
            Value::Datetime(dt) => visitor.visit_map(DatetimeDeserializer::new(*dt)),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(Cow::Borrowed(s)) => visitor.visit_borrowed_str(s),
            Value::String(Cow::Owned(s)) => visitor.visit_str(s),
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
            // The offset of the datetime is deserialized as an integer.
            Value::Datetime(dt) => {
                let offset = dt
                    .offset
                    .ok_or_else(|| -> Self::Error {
                        de::Error::invalid_type(de::Unexpected::Other("None"), &visitor)
                    })
                    .map(|offset| match offset {
                        Offset::Custom { minutes } => minutes,
                        Offset::Z => 0,
                    })?;
                visitor.visit_i64(offset as i64)
            }
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

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self {
            Value::String(s) => visitor.visit_enum(s.clone().into_deserializer()),
            // TODO: Support non-unit enums.
            _ => Err(de::Error::invalid_type(
                de::Unexpected::Other("non-string"),
                &visitor,
            )),
        }
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match name {
            "tomlingDatetime" => {
                let dt = match self {
                    Value::Datetime(dt) => dt,
                    _ => return Err(de::Error::custom("expected a datetime")),
                };
                visitor.visit_map(DatetimeDeserializer::new(*dt))
            }
            "tomlingDate" => {
                let date = match self {
                    Value::Datetime(Datetime {
                        date: Some(date), ..
                    }) => date,
                    _ => return Err(de::Error::custom("expected a date")),
                };
                visitor.visit_map(DateDeserializer::new(*date))
            }
            "tomlingTime" => {
                let time = match self {
                    Value::Datetime(Datetime {
                        time: Some(time), ..
                    }) => time,
                    _ => return Err(de::Error::custom("expected a time")),
                };
                visitor.visit_map(TimeDeserializer::new(*time))
            }
            _ => self.deserialize_any(visitor),
        }
    }

    serde::forward_to_deserialize_any! {
        i8 i16 i32 i128 u8 u16 u32 u64 u128 f32
        char string bytes byte_buf unit unit_struct
        tuple tuple_struct identifier ignored_any
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

impl<'de> SeqAccess<'de> for SeqDeserializer<'de, '_> {
    type Error = Error;

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

impl<'de> MapAccess<'de> for MapDeserializer<'de, '_> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        if let Some((key, value)) = self.iter.next() {
            self.value = Some(value);
            match key {
                Cow::Owned(s) => seed.deserialize(StrDeserializer::new(s).into_deserializer()),
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
            Some(value) => seed.deserialize(value),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}

struct DatetimeDeserializer {
    dt: Datetime,
    stage: DatetimeDeserializerStage,
}

impl DatetimeDeserializer {
    fn new(dt: Datetime) -> Self {
        DatetimeDeserializer {
            dt,
            stage: DatetimeDeserializerStage::Date,
        }
    }
}

enum DatetimeDeserializerStage {
    Date,
    Time,
    Offset,
    Done,
}

// Implment MapAccess for DatetimeDeserializer.
impl<'de> MapAccess<'de> for DatetimeDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        let de = match self.stage {
            DatetimeDeserializerStage::Date => "date".into_deserializer(),
            DatetimeDeserializerStage::Time => "time".into_deserializer(),
            DatetimeDeserializerStage::Offset => "offset".into_deserializer(),
            DatetimeDeserializerStage::Done => return Ok(None),
        };
        seed.deserialize(de).map(Some)
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        match self.stage {
            DatetimeDeserializerStage::Date => {
                self.stage = DatetimeDeserializerStage::Time;

                match self.dt.date {
                    Some(_) => seed.deserialize(&Value::Datetime(self.dt)),
                    None => seed.deserialize(NoneDeserializer),
                }
            }
            DatetimeDeserializerStage::Time => {
                self.stage = DatetimeDeserializerStage::Offset;
                match self.dt.time {
                    Some(_) => seed.deserialize(&Value::Datetime(self.dt)),
                    None => seed.deserialize(NoneDeserializer),
                }
            }
            DatetimeDeserializerStage::Offset => {
                self.stage = DatetimeDeserializerStage::Done;
                match self
                    .dt
                    .offset
                    .map(|offset| I64Deserializer::new(offset.as_minutes() as i64))
                {
                    Some(de) => seed.deserialize(de),
                    None => seed.deserialize(NoneDeserializer),
                }
            }
            DatetimeDeserializerStage::Done => Err(de::Error::custom("unexpected key")),
        }
    }
}

struct DateDeserializer {
    date: Date,
    stage: DateDeserializerStage,
}

impl DateDeserializer {
    fn new(date: Date) -> Self {
        DateDeserializer {
            date,
            stage: DateDeserializerStage::Year,
        }
    }
}

enum DateDeserializerStage {
    Year,
    Month,
    Day,
    Done,
}

impl<'de> MapAccess<'de> for DateDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.stage {
            DateDeserializerStage::Year => {
                self.stage = DateDeserializerStage::Month;
                seed.deserialize("year".into_deserializer()).map(Some)
            }
            DateDeserializerStage::Month => {
                self.stage = DateDeserializerStage::Day;
                seed.deserialize("month".into_deserializer()).map(Some)
            }
            DateDeserializerStage::Day => {
                self.stage = DateDeserializerStage::Done;
                seed.deserialize("day".into_deserializer()).map(Some)
            }
            DateDeserializerStage::Done => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = match self.stage {
            DateDeserializerStage::Year => Value::Integer(self.date.year as i64),
            DateDeserializerStage::Month => Value::Integer(self.date.month as i64),
            DateDeserializerStage::Day => Value::Integer(self.date.day as i64),
            DateDeserializerStage::Done => return Err(de::Error::custom("unexpected key")),
        };
        seed.deserialize(&value)
    }
}

struct TimeDeserializer {
    time: crate::Time,
    stage: TimeDeserializerStage,
}

impl TimeDeserializer {
    fn new(time: crate::Time) -> Self {
        TimeDeserializer {
            time,
            stage: TimeDeserializerStage::Hour,
        }
    }
}

enum TimeDeserializerStage {
    Hour,
    Minute,
    Second,
    Nanosecond,
    Done,
}

impl<'de> MapAccess<'de> for TimeDeserializer {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'de>,
    {
        match self.stage {
            TimeDeserializerStage::Hour => {
                self.stage = TimeDeserializerStage::Minute;
                seed.deserialize("hour".into_deserializer()).map(Some)
            }
            TimeDeserializerStage::Minute => {
                self.stage = TimeDeserializerStage::Second;
                seed.deserialize("minute".into_deserializer()).map(Some)
            }
            TimeDeserializerStage::Second => {
                self.stage = TimeDeserializerStage::Nanosecond;
                seed.deserialize("second".into_deserializer()).map(Some)
            }
            TimeDeserializerStage::Nanosecond => {
                self.stage = TimeDeserializerStage::Done;
                seed.deserialize("nanosecond".into_deserializer()).map(Some)
            }
            TimeDeserializerStage::Done => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = match self.stage {
            TimeDeserializerStage::Hour => Value::Integer(self.time.hour as i64),
            TimeDeserializerStage::Minute => Value::Integer(self.time.minute as i64),
            TimeDeserializerStage::Second => Value::Integer(self.time.second as i64),
            TimeDeserializerStage::Nanosecond => Value::Integer(self.time.nanosecond as i64),
            TimeDeserializerStage::Done => return Err(de::Error::custom("unexpected key")),
        };
        seed.deserialize(&value)
    }
}

struct NoneDeserializer;

impl<'de> Deserializer<'de> for NoneDeserializer {
    type Error = Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(de::Error::custom("unexpected value"))
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        visitor.visit_none()
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string bytes byte_buf unit unit_struct
        newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
    }
}
