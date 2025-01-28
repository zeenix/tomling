use core::ops::RangeInclusive;

use crate::Error;

use crate::{datetime::Offset, Date, Datetime, Time};
use winnow::{
    combinator::{alt, cut_err, opt, preceded, trace},
    error::{FromExternalError, StrContext},
    stream::Stream as _,
    token::{one_of, take_while},
    ModalResult, Parser,
};

// ;; Date and Time (as defined in RFC 3339)

// date-time = offset-date-time / local-date-time / local-date / local-time
// offset-date-time = full-date time-delim full-time
// local-date-time = full-date time-delim partial-time
// local-date = full-date
// local-time = partial-time
// full-time = partial-time time-offset
pub(crate) fn date_time(input: &mut &str) -> ModalResult<Datetime> {
    trace(
        "date-time",
        alt((
            (full_date, opt((time_delim, partial_time, opt(time_offset))))
                .map(|(date, opt)| {
                    match opt {
                        // Offset Date-Time
                        Some((_, time, offset)) => Datetime {
                            date: Some(date),
                            time: Some(time),
                            offset,
                        },
                        // Local Date
                        None => Datetime {
                            date: Some(date),
                            time: None,
                            offset: None,
                        },
                    }
                })
                .context(StrContext::Label("date-time")),
            partial_time
                .map(|t| t.into())
                .context(StrContext::Label("time")),
        )),
    )
    .parse_next(input)
}

// full-date      = date-fullyear "-" date-month "-" date-mday
fn full_date(input: &mut &str) -> ModalResult<Date> {
    trace("full-date", full_date_).parse_next(input)
}

fn full_date_(input: &mut &str) -> ModalResult<Date> {
    let year = date_fullyear.parse_next(input)?;
    let _ = '-'.parse_next(input)?;
    let month = cut_err(date_month).parse_next(input)?;
    let _ = cut_err('-').parse_next(input)?;
    let day_start = input.checkpoint();
    let day = cut_err(date_mday).parse_next(input)?;

    let is_leap_year = (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0));
    let max_days_in_month = match month {
        2 if is_leap_year => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    if max_days_in_month < day {
        input.reset(&day_start);
        return Err(winnow::error::ErrMode::from_external_error(
            input,
            winnow::error::ErrorKind::Verify,
            Error::Datetime,
        )
        .cut());
    }

    Ok(Date { year, month, day })
}

// partial-time   = time-hour ":" time-minute ":" time-second [time-secfrac]
fn partial_time(input: &mut &str) -> ModalResult<Time> {
    trace(
        "partial-time",
        (
            time_hour,
            ':',
            cut_err((time_minute, ':', time_second, opt(time_secfrac))),
        )
            .map(|(hour, _, (minute, _, second, nanosecond))| Time {
                hour,
                minute,
                second,
                nanosecond: nanosecond.unwrap_or_default(),
            }),
    )
    .parse_next(input)
}

// time-offset    = "Z" / time-numoffset
// time-numoffset = ( "+" / "-" ) time-hour ":" time-minute
fn time_offset(input: &mut &str) -> ModalResult<Offset> {
    trace(
        "time-offset",
        alt((
            one_of(('Z', 'z')).value(Offset::Z),
            (one_of(('+', '-')), cut_err((time_hour, ':', time_minute)))
                .map(|(sign, (hours, _, minutes))| {
                    let sign = match sign {
                        '+' => 1,
                        '-' => -1,
                        _ => unreachable!("Parser prevents this"),
                    };
                    sign * (hours as i16 * 60 + minutes as i16)
                })
                .verify(|minutes| ((-24 * 60)..=(24 * 60)).contains(minutes))
                .map(|minutes| Offset::Custom { minutes }),
        ))
        .context(StrContext::Label("time offset")),
    )
    .parse_next(input)
}

// date-fullyear  = 4DIGIT
fn date_fullyear(input: &mut &str) -> ModalResult<u16> {
    unsigned_digits::<4, 4>
        .map(|s: &str| s.parse::<u16>().expect("4DIGIT should match u8"))
        .parse_next(input)
}

// date-month     = 2DIGIT  ; 01-12
fn date_month(input: &mut &str) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (1..=12).contains(&d) {
                Ok(d)
            } else {
                Err(Error::Datetime)
            }
        })
        .parse_next(input)
}

// date-mday      = 2DIGIT  ; 01-28, 01-29, 01-30, 01-31 based on month/year
fn date_mday(input: &mut &str) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (1..=31).contains(&d) {
                Ok(d)
            } else {
                Err(Error::Datetime)
            }
        })
        .parse_next(input)
}

// time-delim     = "T" / %x20 ; T, t, or space
fn time_delim(input: &mut &str) -> ModalResult<char> {
    one_of(TIME_DELIM).parse_next(input)
}

const TIME_DELIM: (u8, u8, u8) = (b'T', b't', b' ');

// time-hour      = 2DIGIT  ; 00-23
fn time_hour(input: &mut &str) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (0..=23).contains(&d) {
                Ok(d)
            } else {
                Err(Error::Datetime)
            }
        })
        .parse_next(input)
}

// time-minute    = 2DIGIT  ; 00-59
fn time_minute(input: &mut &str) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (0..=59).contains(&d) {
                Ok(d)
            } else {
                Err(Error::Datetime)
            }
        })
        .parse_next(input)
}

// time-second    = 2DIGIT  ; 00-58, 00-59, 00-60 based on leap second rules
fn time_second(input: &mut &str) -> ModalResult<u8> {
    unsigned_digits::<2, 2>
        .try_map(|s: &str| {
            let d = s.parse::<u8>().expect("2DIGIT should match u8");
            if (0..=60).contains(&d) {
                Ok(d)
            } else {
                Err(Error::Datetime)
            }
        })
        .parse_next(input)
}

// time-secfrac   = "." 1*DIGIT
fn time_secfrac(input: &mut &str) -> ModalResult<u32> {
    static SCALE: [u32; 10] = [
        0,
        100_000_000,
        10_000_000,
        1_000_000,
        100_000,
        10_000,
        1_000,
        100,
        10,
        1,
    ];
    const INF: usize = usize::MAX;
    preceded('.', unsigned_digits::<1, INF>)
        .try_map(|mut repr: &str| -> Result<u32, Error> {
            let max_digits = SCALE.len() - 1;
            if max_digits < repr.len() {
                // Millisecond precision is required. Further precision of fractional seconds is
                // implementation-specific. If the value contains greater precision than the
                // implementation can support, the additional precision must be truncated, not
                // rounded.
                repr = &repr[0..max_digits];
            }

            let v = repr.parse::<u32>().map_err(|_| Error::Datetime)?;
            let num_digits = repr.len();

            // scale the number accordingly.
            let scale = SCALE.get(num_digits).ok_or(Error::Datetime)?;
            let v = v.checked_mul(*scale).ok_or(Error::Datetime)?;
            Ok(v)
        })
        .parse_next(input)
}

fn unsigned_digits<'i, const MIN: usize, const MAX: usize>(
    input: &mut &'i str,
) -> ModalResult<&'i str> {
    take_while(MIN..=MAX, DIGIT).parse_next(input)
}

// DIGIT = %x30-39 ; 0-9
const DIGIT: RangeInclusive<u8> = b'0'..=b'9';
