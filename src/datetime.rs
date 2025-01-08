//! Parsed TOML datetime values.
//!
//! This module is mostly a copy of the `toml_datetime` crate with a few modifications. It is hoped
//! that it can be drop once `toml_datetime` is updated to support `no_std`, which will require API
//! breakage.

use alloc::format;
use core::{
    fmt,
    str::{self, FromStr},
};

use crate::Error;

/// A parsed TOML datetime value
///
/// This structure is intended to represent the datetime primitive type that can
/// be encoded into TOML documents. This type is a parsed version that contains
/// all metadata internally.
///
/// Currently this type is intentionally conservative and only supports
/// `to_string` as an accessor. Over time though it's intended that it'll grow
/// more support!
///
/// Depending on how the option values are used, this struct will correspond
/// with one of the following four datetimes from the [TOML v1.0.0 spec]:
///
/// | `date`    | `time`    | `offset`  | TOML type          |
/// | --------- | --------- | --------- | ------------------ |
/// | `Some(_)` | `Some(_)` | `Some(_)` | [Offset Date-Time] |
/// | `Some(_)` | `Some(_)` | `None`    | [Local Date-Time]  |
/// | `Some(_)` | `None`    | `None`    | [Local Date]       |
/// | `None`    | `Some(_)` | `None`    | [Local Time]       |
///
/// **1. Offset Date-Time**: If all the optional values are used, `Datetime`
/// corresponds to an [Offset Date-Time]. From the TOML v1.0.0 spec:
///
/// > To unambiguously represent a specific instant in time, you may use an
/// > RFC 3339 formatted date-time with offset.
/// >
/// > ```toml
/// > odt1 = 1979-05-27T07:32:00Z
/// > odt2 = 1979-05-27T00:32:00-07:00
/// > odt3 = 1979-05-27T00:32:00.999999-07:00
/// > ```
/// >
/// > For the sake of readability, you may replace the T delimiter between date
/// > and time with a space character (as permitted by RFC 3339 section 5.6).
/// >
/// > ```toml
/// > odt4 = 1979-05-27 07:32:00Z
/// > ```
///
/// **2. Local Date-Time**: If `date` and `time` are given but `offset` is
/// `None`, `Datetime` corresponds to a [Local Date-Time]. From the spec:
///
/// > If you omit the offset from an RFC 3339 formatted date-time, it will
/// > represent the given date-time without any relation to an offset or
/// > timezone. It cannot be converted to an instant in time without additional
/// > information. Conversion to an instant, if required, is implementation-
/// > specific.
/// >
/// > ```toml
/// > ldt1 = 1979-05-27T07:32:00
/// > ldt2 = 1979-05-27T00:32:00.999999
/// > ```
///
/// **3. Local Date**: If only `date` is given, `Datetime` corresponds to a
/// [Local Date]; see the docs for [`Date`].
///
/// **4. Local Time**: If only `time` is given, `Datetime` corresponds to a
/// [Local Time]; see the docs for [`Time`].
///
/// [TOML v1.0.0 spec]: https://toml.io/en/v1.0.0
/// [Offset Date-Time]: https://toml.io/en/v1.0.0#offset-date-time
/// [Local Date-Time]: https://toml.io/en/v1.0.0#local-date-time
/// [Local Date]: https://toml.io/en/v1.0.0#local-date
/// [Local Time]: https://toml.io/en/v1.0.0#local-time
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Datetime {
    /// Optional date.
    /// Required for: *Offset Date-Time*, *Local Date-Time*, *Local Date*.
    pub date: Option<Date>,

    /// Optional time.
    /// Required for: *Offset Date-Time*, *Local Date-Time*, *Local Time*.
    pub time: Option<Time>,

    /// Optional offset.
    /// Required for: *Offset Date-Time*.
    pub offset: Option<Offset>,
}

/// A parsed TOML date value
///
/// May be part of a [`Datetime`]. Alone, `Date` corresponds to a [Local Date].
/// From the TOML v1.0.0 spec:
///
/// > If you include only the date portion of an RFC 3339 formatted date-time,
/// > it will represent that entire day without any relation to an offset or
/// > timezone.
/// >
/// > ```toml
/// > ld1 = 1979-05-27
/// > ```
///
/// [Local Date]: https://toml.io/en/v1.0.0#local-date
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Date {
    /// Year: four digits
    pub year: u16,
    /// Month: 1 to 12
    pub month: u8,
    /// Day: 1 to {28, 29, 30, 31} (based on month/year)
    pub day: u8,
}

/// A parsed TOML time value
///
/// May be part of a [`Datetime`]. Alone, `Time` corresponds to a [Local Time].
/// From the TOML v1.0.0 spec:
///
/// > If you include only the time portion of an RFC 3339 formatted date-time,
/// > it will represent that time of day without any relation to a specific
/// > day or any offset or timezone.
/// >
/// > ```toml
/// > lt1 = 07:32:00
/// > lt2 = 00:32:00.999999
/// > ```
/// >
/// > Millisecond precision is required. Further precision of fractional
/// > seconds is implementation-specific. If the value contains greater
/// > precision than the implementation can support, the additional precision
/// > must be truncated, not rounded.
///
/// [Local Time]: https://toml.io/en/v1.0.0#local-time
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct Time {
    /// Hour: 0 to 23
    pub hour: u8,
    /// Minute: 0 to 59
    pub minute: u8,
    /// Second: 0 to {58, 59, 60} (based on leap second rules)
    pub second: u8,
    /// Nanosecond: 0 to `999_999_999`
    pub nanosecond: u32,
}

/// A parsed TOML time offset
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Debug)]
pub enum Offset {
    /// > A suffix which, when applied to a time, denotes a UTC offset of 00:00;
    /// > often spoken "Zulu" from the ICAO phonetic alphabet representation of
    /// > the letter "Z". --- [RFC 3339 section 2]
    ///
    /// [RFC 3339 section 2]: https://datatracker.ietf.org/doc/html/rfc3339#section-2
    Z,

    /// Offset between local time and UTC
    Custom {
        /// Minutes: -`1_440..1_440`
        minutes: i16,
    },
}

impl Offset {
    /// The offset in minutes.
    pub fn as_minutes(&self) -> i16 {
        match *self {
            Offset::Z => 0,
            Offset::Custom { minutes } => minutes,
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Offset {
    // deserialize as an i16.
    fn deserialize<D>(deserializer: D) -> Result<Offset, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match i16::deserialize(deserializer)? {
            0 => Ok(Offset::Z),
            minutes => Ok(Offset::Custom { minutes }),
        }
    }
}

impl From<Date> for Datetime {
    fn from(other: Date) -> Self {
        Datetime {
            date: Some(other),
            time: None,
            offset: None,
        }
    }
}

impl From<Time> for Datetime {
    fn from(other: Time) -> Self {
        Datetime {
            date: None,
            time: Some(other),
            offset: None,
        }
    }
}

impl From<Offset> for Datetime {
    fn from(other: Offset) -> Self {
        Datetime {
            date: None,
            time: None,
            offset: Some(other),
        }
    }
}

impl fmt::Display for Datetime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref date) = self.date {
            write!(f, "{date}")?;
        }
        if let Some(ref time) = self.time {
            if self.date.is_some() {
                write!(f, "T")?;
            }
            write!(f, "{time}")?;
        }
        if let Some(ref offset) = self.offset {
            write!(f, "{offset}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}:{:02}", self.hour, self.minute, self.second)?;
        if self.nanosecond != 0 {
            let s = format!("{:09}", self.nanosecond);
            write!(f, ".{}", s.trim_end_matches('0'))?;
        }
        Ok(())
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Offset::Z => write!(f, "Z"),
            Offset::Custom { mut minutes } => {
                let mut sign = '+';
                if minutes < 0 {
                    minutes *= -1;
                    sign = '-';
                }
                let hours = minutes / 60;
                let minutes = minutes % 60;
                write!(f, "{sign}{hours:02}:{minutes:02}")
            }
        }
    }
}

impl FromStr for Datetime {
    type Err = Error;

    fn from_str(date: &str) -> Result<Datetime, Error> {
        // Accepted formats:
        //
        // 0000-00-00T00:00:00.00Z
        // 0000-00-00T00:00:00.00
        // 0000-00-00
        // 00:00:00.00
        if date.len() < 3 {
            return Err(Error::Datetime);
        }
        let mut offset_allowed = true;
        let mut chars = date.chars();

        // First up, parse the full date if we can
        let full_date = if chars.clone().nth(2) == Some(':') {
            offset_allowed = false;
            None
        } else {
            let y1 = u16::from(digit(&mut chars)?);
            let y2 = u16::from(digit(&mut chars)?);
            let y3 = u16::from(digit(&mut chars)?);
            let y4 = u16::from(digit(&mut chars)?);

            match chars.next() {
                Some('-') => {}
                _ => return Err(Error::Datetime),
            }

            let m1 = digit(&mut chars)?;
            let m2 = digit(&mut chars)?;

            match chars.next() {
                Some('-') => {}
                _ => return Err(Error::Datetime),
            }

            let d1 = digit(&mut chars)?;
            let d2 = digit(&mut chars)?;

            let date = Date {
                year: y1 * 1000 + y2 * 100 + y3 * 10 + y4,
                month: m1 * 10 + m2,
                day: d1 * 10 + d2,
            };

            if date.month < 1 || date.month > 12 {
                return Err(Error::Datetime);
            }
            let is_leap_year =
                (date.year % 4 == 0) && ((date.year % 100 != 0) || (date.year % 400 == 0));
            let max_days_in_month = match date.month {
                2 if is_leap_year => 29,
                2 => 28,
                4 | 6 | 9 | 11 => 30,
                _ => 31,
            };
            if date.day < 1 || date.day > max_days_in_month {
                return Err(Error::Datetime);
            }

            Some(date)
        };

        // Next parse the "partial-time" if available
        let next = chars.clone().next();
        let partial_time = if full_date.is_some()
            && (next == Some('T') || next == Some('t') || next == Some(' '))
        {
            chars.next();
            true
        } else {
            full_date.is_none()
        };

        let time = if partial_time {
            let h1 = digit(&mut chars)?;
            let h2 = digit(&mut chars)?;
            match chars.next() {
                Some(':') => {}
                _ => return Err(Error::Datetime),
            }
            let m1 = digit(&mut chars)?;
            let m2 = digit(&mut chars)?;
            match chars.next() {
                Some(':') => {}
                _ => return Err(Error::Datetime),
            }
            let s1 = digit(&mut chars)?;
            let s2 = digit(&mut chars)?;

            let mut nanosecond = 0;
            if chars.clone().next() == Some('.') {
                chars.next();
                let whole = chars.as_str();

                let mut end = whole.len();
                for (i, byte) in whole.bytes().enumerate() {
                    #[allow(clippy::single_match_else)]
                    match byte {
                        b'0'..=b'9' => {
                            if i < 9 {
                                let p = 10_u32.pow(8 - i as u32);
                                nanosecond += p * u32::from(byte - b'0');
                            }
                        }
                        _ => {
                            end = i;
                            break;
                        }
                    }
                }
                if end == 0 {
                    return Err(Error::Datetime);
                }
                chars = whole[end..].chars();
            }

            let time = Time {
                hour: h1 * 10 + h2,
                minute: m1 * 10 + m2,
                second: s1 * 10 + s2,
                nanosecond,
            };

            if time.hour > 24 {
                return Err(Error::Datetime);
            }
            if time.minute > 59 {
                return Err(Error::Datetime);
            }
            // 00-58, 00-59, 00-60 based on leap second rules
            if time.second > 60 {
                return Err(Error::Datetime);
            }
            if time.nanosecond > 999_999_999 {
                return Err(Error::Datetime);
            }

            Some(time)
        } else {
            offset_allowed = false;
            None
        };

        // And finally, parse the offset
        let offset = if offset_allowed {
            let next = chars.clone().next();
            if next == Some('Z') || next == Some('z') {
                chars.next();
                Some(Offset::Z)
            } else if next.is_none() {
                None
            } else {
                let sign = match next {
                    Some('+') => 1,
                    Some('-') => -1,
                    _ => return Err(Error::Datetime),
                };
                chars.next();
                let h1 = digit(&mut chars)? as i16;
                let h2 = digit(&mut chars)? as i16;
                match chars.next() {
                    Some(':') => {}
                    _ => return Err(Error::Datetime),
                }
                let m1 = digit(&mut chars)? as i16;
                let m2 = digit(&mut chars)? as i16;

                let hours = h1 * 10 + h2;
                let minutes = m1 * 10 + m2;

                let total_minutes = sign * (hours * 60 + minutes);

                if !((-24 * 60)..=(24 * 60)).contains(&total_minutes) {
                    return Err(Error::Datetime);
                }

                Some(Offset::Custom {
                    minutes: total_minutes,
                })
            }
        } else {
            None
        };

        // Return an error if we didn't hit eof, otherwise return our parsed
        // date
        if chars.next().is_some() {
            return Err(Error::Datetime);
        }

        Ok(Datetime {
            date: full_date,
            time,
            offset,
        })
    }
}

fn digit(chars: &mut str::Chars<'_>) -> Result<u8, Error> {
    match chars.next() {
        Some(c) if c.is_ascii_digit() => Ok(c as u8 - b'0'),
        _ => Err(Error::Datetime),
    }
}

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;

    // Serde deserialization tests that takes a TOML document.
    #[test]
    fn serde_datetime_deserialize() {
        #[derive(serde::Deserialize)]
        struct DatetimeTest {
            datetime: Datetime,
        }

        let toml = r#"
            datetime = 1979-05-27T07:32:00Z
        "#;
        let t: DatetimeTest = crate::from_str(toml).unwrap();
        assert_eq!(
            t.datetime,
            Datetime {
                date: Some(Date {
                    year: 1979,
                    month: 5,
                    day: 27
                }),
                time: Some(Time {
                    hour: 7,
                    minute: 32,
                    second: 0,
                    nanosecond: 0
                }),
                offset: Some(Offset::Z)
            }
        );
    }
}
