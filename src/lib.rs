use std::collections::HashMap;
use winnow::{
    ascii::{multispace0, multispace1},
    combinator::{alt, delimited, repeat, separated, separated_pair},
    error::InputError,
    stream::AsChar,
    token::take_while,
    PResult, Parser,
};

/// Parse a TOML document.
pub fn parse<'i>(mut input: &'i str) -> PResult<TomlMap<'i>, InputError<&'i str>> {
    let key_value = parse_key_value.map(|(keys, value)| (None, keys, value));
    let table_header =
        parse_table_header.map(|header| (Some(header), Vec::new(), Value::Table(HashMap::new())));
    let whitespace = multispace1.map(|_| (None, Vec::new(), Value::Table(HashMap::new())));
    let line_parser = alt((table_header, key_value, whitespace));

    repeat(1.., line_parser)
        .fold(
            || (None, HashMap::new()),
            |(current_table, mut map), (header, keys, value)| {
                if header.is_some() {
                    (header, map)
                } else if !keys.is_empty() {
                    if let Some(ref table) = current_table {
                        let mut full_key = table.clone();
                        full_key.extend(keys);
                        insert_nested_key(&mut map, &full_key, value);
                    } else {
                        insert_nested_key(&mut map, &keys, value);
                    }

                    (current_table, map)
                } else {
                    (current_table, map)
                }
            },
        )
        .map(|(_, map)| map)
        .parse_next(&mut input)
}

/// A TOML value.
#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    String(&'a str),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Array(Vec<Value<'a>>),
    Table(HashMap<&'a str, Value<'a>>),
}

pub type TomlMap<'a> = HashMap<&'a str, Value<'a>>;

/// Parses a table header (e.g., `[dependencies]`)
fn parse_table_header<'i>(input: &mut &'i str) -> PResult<Vec<&'i str>, InputError<&'i str>> {
    delimited('[', parse_dotted_key, ']').parse_next(input)
}

/// Parses a single key-value pair
fn parse_key_value<'i>(
    input: &mut &'i str,
) -> PResult<(Vec<&'i str>, Value<'i>), InputError<&'i str>> {
    separated_pair(parse_dotted_key, '=', parse_value).parse_next(input)
}

/// Parses a dotted or single key
fn parse_dotted_key<'i>(input: &mut &'i str) -> PResult<Vec<&'i str>, InputError<&'i str>> {
    separated(1.., parse_key, '.').parse_next(input)
}

/// Parses a key (alphanumeric or underscores)
fn parse_key<'i>(input: &mut &'i str) -> PResult<&'i str, InputError<&'i str>> {
    delimited(
        multispace0,
        take_while(1.., |c: char| c.is_alphanumeric() || c == '_'),
        multispace0,
    )
    .parse_next(input)
}

/// Parses a value (string, integer, float, boolean, array, or table)
fn parse_value<'i>(input: &mut &'i str) -> PResult<Value<'i>, InputError<&'i str>> {
    (
        multispace0,
        // FIXME: Use `dispatch!` to make it more efficient.
        alt((
            parse_string,
            parse_float,
            parse_boolean,
            parse_array,
            parse_inline_table,
        )),
        multispace0,
    )
        .map(|(_, value, _)| value)
        .parse_next(input)
}

/// Parses a string value enclosed in quotes
fn parse_string<'i>(input: &mut &'i str) -> PResult<Value<'i>, InputError<&'i str>> {
    delimited('"', take_while(0.., |c| c != '"'), '"')
        .map(Value::String)
        .parse_next(input)
}

/// Parses an integer value
fn parse_integer<'i>(input: &mut &'i str) -> PResult<Value<'i>, InputError<&'i str>> {
    take_while(1.., |c: char| c.is_ascii_digit())
        .map(|s: &str| Value::Integer(s.parse().unwrap()))
        .parse_next(input)
}

/// Parses a float value
fn parse_float<'i>(input: &mut &'i str) -> PResult<Value<'i>, InputError<&'i str>> {
    alt((
        separated_pair(
            take_while(1.., AsChar::is_dec_digit),
            '.',
            take_while(1.., AsChar::is_dec_digit),
        )
        .map(|(int_part, frac_part): (&str, &str)| {
            Value::Float(format!("{}.{}", int_part, frac_part).parse().unwrap())
        }),
        parse_integer,
    ))
    .parse_next(input)
}

/// Parses a boolean value
fn parse_boolean<'i>(input: &mut &'i str) -> PResult<Value<'i>, InputError<&'i str>> {
    alt((
        "true".map(|_| Value::Boolean(true)),
        "false".map(|_| Value::Boolean(false)),
    ))
    .parse_next(input)
}

/// Parses an array of values
fn parse_array<'i>(input: &mut &'i str) -> PResult<Value<'i>, InputError<&'i str>> {
    delimited(
        ('[', multispace0),
        separated(0.., parse_value, ','),
        (multispace0, ']'),
    )
    .map(Value::Array)
    .parse_next(input)
}

/// Parses an inline table
fn parse_inline_table<'i>(input: &mut &'i str) -> PResult<Value<'i>, InputError<&'i str>> {
    delimited(
        ('{', multispace0),
        separated(0.., separated_pair(parse_key, '=', parse_value), ','),
        (multispace0, '}'),
    )
    .map(|pairs: Vec<(&'i str, Value<'i>)>| Value::Table(pairs.into_iter().collect()))
    .parse_next(input)
}

/// Inserts a value into a nested map using a dotted key
fn insert_nested_key<'a>(map: &mut TomlMap<'a>, keys: &[&'a str], value: Value<'a>) {
    if let Some((first, rest)) = keys.split_first() {
        if rest.is_empty() {
            map.insert(first, value);
        } else {
            let entry = map
                .entry(first)
                .or_insert_with(|| Value::Table(HashMap::new()));

            if let Value::Table(ref mut nested_map) = entry {
                insert_nested_key(nested_map, rest, value);
            }
        }
    }
}
