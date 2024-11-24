mod numbers;

use crate::{Array, Error, ParseError, Table, Value};

use alloc::{vec, vec::Vec};
use winnow::{
    ascii::{multispace1, space0},
    combinator::{alt, delimited, opt, repeat, separated, separated_pair, terminated},
    error::ContextError,
    token::{take_until, take_while},
    PResult, Parser,
};

/// Parse a TOML document.
pub fn parse(input: &str) -> Result<Table<'_>, Error> {
    let key_value = parse_key_value.map(|(keys, value)| (None, keys, value));
    let table_header = parse_table_header.map(|(header, is_array)| {
        (
            Some((header, is_array)),
            Vec::new(),
            Value::Table(Table::new()),
        )
    });
    let whitespace = multispace1.map(|_| (None, Vec::new(), Value::Table(Table::new())));
    let comment = parse_comments.map(|_| (None, Vec::new(), Value::Table(Table::new())));
    let line_parser = alt((table_header, key_value, whitespace, comment));

    repeat(1.., line_parser)
        .fold(
            || (None, Table::new()),
            |(mut current_table, mut map), (header, keys, value)| {
                if let Some((header, is_array)) = header {
                    if is_array {
                        // Handle array of tables ([[table]])
                        let key = *header.last().expect("Header should not be empty");
                        let entry = map.entry(key).or_insert_with(|| Value::Array(Array::new()));
                        if let Value::Array(array) = entry {
                            // Append a new empty table to the array
                            let new_table = Table::new();
                            array.push(Value::Table(new_table));

                            // Update current_table to reference the new table
                            current_table = Some(vec![key]);
                        }
                    } else {
                        // Handle regular table ([table]) with dotted keys
                        current_table = Some(header);
                    }
                } else if !keys.is_empty() {
                    if let Some(ref table) = current_table {
                        if let Some(Value::Array(array)) = map.get_mut(table[0]) {
                            // Insert into the most recent table in the array
                            if let Some(Value::Table(last_table)) = array.last_mut() {
                                insert_nested_key(last_table, &keys, value);
                            }
                        } else {
                            // Insert into a regular table
                            let mut full_key = table.clone();
                            full_key.extend(keys);
                            insert_nested_key(&mut map, &full_key, value);
                        }
                    } else {
                        // Global key-value pair
                        insert_nested_key(&mut map, &keys, value);
                    }
                }
                (current_table, map)
            },
        )
        .map(|(_, map)| map)
        .parse(input)
        .map_err(|e| ParseError::new(e.into_inner()))
        .map_err(Error::Parse)
}

/// Parses a table header (e.g., `[dependencies]`)
fn parse_table_header<'i>(input: &mut &'i str) -> PResult<(Vec<&'i str>, bool), ContextError> {
    alt((
        delimited("[[", parse_dotted_key, "]]").map(|keys| (keys, true)), // Array of tables
        delimited('[', parse_dotted_key, ']').map(|keys| (keys, false)),  // Regular table
    ))
    .parse_next(input)
}

/// Parses comments.
fn parse_comments(input: &mut &'_ str) -> PResult<(), ContextError> {
    delimited(
        '#',
        take_while(
            0..,
            // > Control characters other than tab (U+0000 to U+0008, U+000A to U+001F, U+007F) are
            // > not permitted in comments.
            |c| !matches!(c, '\0'..='\u{08}' | '\u{0a}'..='\u{1f}' | '\u{7f}'),
        ),
        alt(("\r\n", "\n")),
    )
    .void()
    .parse_next(input)
}

/// Parses a single key-value pair
fn parse_key_value<'i>(input: &mut &'i str) -> PResult<(Vec<&'i str>, Value<'i>), ContextError> {
    separated_pair(parse_dotted_key, '=', parse_value).parse_next(input)
}

/// Parses a dotted or single key
fn parse_dotted_key<'i>(input: &mut &'i str) -> PResult<Vec<&'i str>, ContextError> {
    separated(1.., parse_key, '.').parse_next(input)
}

/// Parses a key (alphanumeric or underscores)
fn parse_key<'i>(input: &mut &'i str) -> PResult<&'i str, ContextError> {
    // We don't use `parse_string` here beecause in the future that will also accept multiline
    // strings and we don't want that here.
    let string_key = alt((parse_basic_string, parse_literal_string)).map(|s| match s {
        Value::String(s) => s,
        _ => unreachable!(),
    });
    delimited(
        space0,
        alt((
            string_key,
            take_while(1.., |c: char| c.is_alphanumeric() || c == '_' || c == '-'),
        )),
        space0,
    )
    .parse_next(input)
}

/// Parses a value (string, integer, float, boolean, array, or table)
fn parse_value<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited(
        space0,
        // FIXME: Use `dispatch!` to make it more efficient.
        alt((
            parse_string,
            parse_float,
            parse_integer,
            parse_boolean,
            parse_array,
            parse_inline_table,
        )),
        space0,
    )
    .parse_next(input)
}

/// Parses a string value enclosed in quotes
fn parse_string<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    // TODO:
    // * Handle multiline basic and literal strings.
    // * Handle escape sequences.
    alt((
        parse_multiline_basic_string,
        parse_basic_string,
        parse_multiline_literal_string,
        parse_literal_string,
    ))
    .parse_next(input)
}

/// Parses a basic string value enclosed in quotes.
fn parse_basic_string<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited('"', take_until(0.., '"'), '"')
        .map(Value::String)
        .parse_next(input)
}

/// Parses a literal string value enclosed in single quotes.
fn parse_literal_string<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited('\'', take_until(0.., '\''), '\'')
        .map(Value::String)
        .parse_next(input)
}

/// Parses a multiline basic string value enclosed in triple quotes.
fn parse_multiline_basic_string<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited(
        "\"\"\"",
        take_until(0.., "\"\"\"").map(|s: &str| {
            // Trim leading newlines.
            s.trim_start_matches('\n')
        }),
        "\"\"\"",
    )
    .map(Value::String)
    .parse_next(input)
}

/// Parses a literal multiline string value enclosed in triple single quotes (`'''`).
fn parse_multiline_literal_string<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited(
        "'''",
        take_until(0.., "'''").map(|s: &str| s.trim_start_matches('\n')), // Trim leading newlines
        "'''",
    )
    .map(Value::String)
    .parse_next(input)
}

/// Parses an integer value
fn parse_integer<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    numbers::integer(input).map(Value::Integer)
}

/// Parses a float value
fn parse_float<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    numbers::float(input).map(Value::Float)
}

/// Parses a boolean value
fn parse_boolean<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    numbers::boolean(input).map(Value::Boolean)
}

/// Parses an array of values
fn parse_array<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited(
        '[',
        (repeat(0.., parse_multiline_array_values), opt(parse_value)),
        ']',
    )
    .map(|(values, value): (Vec<_>, _)| {
        let mut values: Array<'i> = values.into_iter().flatten().collect();
        if let Some(value) = value {
            values.push(value);
        }
        Value::Array(values)
    })
    .parse_next(input)
}

fn parse_multiline_array_values<'i>(
    input: &mut &'i str,
) -> PResult<Option<Value<'i>>, ContextError> {
    alt((
        multispace1.map(|_| None),
        parse_comments.map(|_| None),
        terminated(parse_value, ',').map(Some),
    ))
    .parse_next(input)
}

/// Parses an inline table
fn parse_inline_table<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited(
        '{',
        separated(0.., separated_pair(parse_key, '=', parse_value), ','),
        '}',
    )
    .map(|pairs: Vec<(&'i str, Value<'i>)>| Value::Table(pairs.into_iter().collect()))
    .parse_next(input)
}

/// Inserts a value into a nested map using a dotted key
fn insert_nested_key<'a>(map: &mut Table<'a>, keys: &[&'a str], value: Value<'a>) {
    if let Some((first, rest)) = keys.split_first() {
        if rest.is_empty() {
            map.insert(first, value);
        } else {
            let entry = map
                .entry(first)
                .or_insert_with(|| Value::Table(Table::new()));

            if let Value::Table(ref mut nested_map) = entry {
                insert_nested_key(nested_map, rest, value);
            }
        }
    }
}
