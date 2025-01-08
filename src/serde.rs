use alloc::borrow::Cow;

use crate::{
    array::{self, Array},
    datetime::Offset,
    table::{self, Table},
    Date, Datetime, Error, Time, Value,
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

    T::deserialize(ValueDeserializer {
        value: Some(Value::Table(value)),
        date: None,
        time: None,
    })
}

#[derive(Debug)]
struct ValueDeserializer<'de> {
    value: Option<Value<'de>>,
    // If any of these are set, we're deserializing the fields of a `Datetime` value.
    date: Option<Date>,
    time: Option<Time>,
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
            Some(Value::Datetime(_)) => self.deserialize_struct("", &[], visitor),
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

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Some(Value::Datetime(dt)) => {
                if let Some(date) = self.date {
                    visitor.visit_map(DateDeserializer::new(date))
                } else if let Some(time) = self.time {
                    visitor.visit_map(TimeDeserializer::new(time))
                } else {
                    visitor.visit_map(DatetimeDeserializer::new(dt))
                }
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
            let de = ValueDeserializer {
                value: Some(value),
                date: None,
                time: None,
            };
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
            Some(value) => seed.deserialize(ValueDeserializer {
                value: Some(value),
                date: None,
                time: None,
            }),
            None => Err(de::Error::custom("value is missing")),
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
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
        let de = match self.stage {
            DatetimeDeserializerStage::Date => {
                self.stage = DatetimeDeserializerStage::Time;
                match self.dt.date {
                    Some(date) => ValueDeserializer {
                        value: Some(Value::Datetime(self.dt)),
                        date: Some(date),
                        time: None,
                    },
                    None => ValueDeserializer {
                        value: None,
                        date: None,
                        time: None,
                    },
                }
            }
            DatetimeDeserializerStage::Time => {
                self.stage = DatetimeDeserializerStage::Offset;
                match self.dt.time {
                    Some(time) => ValueDeserializer {
                        value: Some(Value::Datetime(self.dt)),
                        date: None,
                        time: Some(time),
                    },
                    None => ValueDeserializer {
                        value: None,
                        date: None,
                        time: None,
                    },
                }
            }
            DatetimeDeserializerStage::Offset => {
                self.stage = DatetimeDeserializerStage::Done;
                // The offset of the datetime is deserialized as an integer.
                let offset = self.dt.offset.map(|offset| match offset {
                    Offset::Custom { minutes } => minutes as i64,
                    Offset::Z => 0,
                });
                ValueDeserializer {
                    value: offset.map(Value::Integer),
                    date: None,
                    time: None,
                }
            }
            DatetimeDeserializerStage::Done => return Err(de::Error::custom("unexpected key")),
        };

        seed.deserialize(de)
    }
}

#[derive(Debug)]
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

#[derive(Debug, PartialEq)]
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
            DateDeserializerStage::Year => seed.deserialize("year".into_deserializer()).map(Some),
            DateDeserializerStage::Month => seed.deserialize("month".into_deserializer()).map(Some),
            DateDeserializerStage::Day => seed.deserialize("day".into_deserializer()).map(Some),
            DateDeserializerStage::Done => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'de>,
    {
        let value = match self.stage {
            DateDeserializerStage::Year => {
                self.stage = DateDeserializerStage::Month;
                self.date.year as i64
            }
            DateDeserializerStage::Month => {
                self.stage = DateDeserializerStage::Day;
                self.date.month as i64
            }
            DateDeserializerStage::Day => {
                self.stage = DateDeserializerStage::Done;
                self.date.day as i64
            }
            DateDeserializerStage::Done => return Err(de::Error::custom("unexpected key")),
        };

        seed.deserialize(I64Deserializer::new(value))
    }
}

#[derive(Debug)]
struct TimeDeserializer {
    time: Time,
    stage: TimeDeserializerStage,
}

impl TimeDeserializer {
    fn new(time: Time) -> Self {
        TimeDeserializer {
            time,
            stage: TimeDeserializerStage::Hour,
        }
    }
}

#[derive(Debug, PartialEq)]
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
            TimeDeserializerStage::Hour => seed.deserialize("hour".into_deserializer()).map(Some),
            TimeDeserializerStage::Minute => {
                seed.deserialize("minute".into_deserializer()).map(Some)
            }
            TimeDeserializerStage::Second => {
                seed.deserialize("second".into_deserializer()).map(Some)
            }
            TimeDeserializerStage::Nanosecond => {
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
            TimeDeserializerStage::Hour => {
                self.stage = TimeDeserializerStage::Minute;
                self.time.hour as i64
            }
            TimeDeserializerStage::Minute => {
                self.stage = TimeDeserializerStage::Second;
                self.time.minute as i64
            }
            TimeDeserializerStage::Second => {
                self.stage = TimeDeserializerStage::Nanosecond;
                self.time.second as i64
            }
            TimeDeserializerStage::Nanosecond => {
                self.stage = TimeDeserializerStage::Done;
                self.time.nanosecond as i64
            }
            TimeDeserializerStage::Done => return Err(de::Error::custom("unexpected key")),
        };

        seed.deserialize(I64Deserializer::new(value))
    }
}
