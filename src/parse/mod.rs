mod ignored;
mod numbers;
mod strings;

use crate::{Array, Error, ParseError, Table, Value};

use alloc::{borrow::Cow, vec, vec::Vec};
use ignored::{parse_comment_newline, parse_whitespace_n_comments};
use winnow::{
    ascii::{multispace1, space0},
    combinator::{alt, cut_err, delimited, opt, peek, preceded, repeat, separated, separated_pair},
    error::ContextError,
    token::take_while,
    PResult, Parser,
};

/// Parse a TOML document.
pub fn parse(input: &str) -> Result<Table<'_>, Error> {
    let key_value = parse_key_value.map(|(keys, value)| (None, keys, value));
    let table_header = parse_table_header
        .map(|(header, is_array)| (Some((header, is_array)), Vec::new(), Table::new().into()));
    let whitespace = multispace1.map(|_| (None, Vec::new(), Table::new().into()));
    let comment_line = parse_comment_newline.map(|_| (None, Vec::new(), Table::new().into()));
    let line_parser = alt((table_header, key_value, whitespace, comment_line));

    repeat(1.., line_parser)
        .fold(
            || (None, Table::new()),
            |(mut current_table, mut map), (header, keys, value)| {
                if let Some((header, is_array)) = header {
                    if is_array {
                        // Handle array of tables ([[table]])
                        let key = header.last().expect("Header should not be empty").clone();
                        let entry = map
                            .entry(key.clone())
                            .or_insert_with(|| Array::new().into());
                        if let Value::Array(array) = entry {
                            // Append a new empty table to the array
                            let new_table = Table::new();
                            array.push(new_table.into());

                            // Update current_table to reference the new table
                            current_table = Some(vec![key]);
                        }
                    } else {
                        // Handle regular table ([table]) with dotted keys
                        current_table = Some(header);
                    }
                } else if !keys.is_empty() {
                    if let Some(ref table) = current_table {
                        if let Some(Value::Array(array)) = map.get_mut(&table[0]) {
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
fn parse_table_header<'i>(input: &mut &'i str) -> PResult<(Vec<Cow<'i, str>>, bool), ContextError> {
    alt((
        delimited("[[", parse_dotted_key, "]]").map(|keys| (keys, true)), // Array of tables
        delimited('[', parse_dotted_key, ']').map(|keys| (keys, false)),  // Regular table
    ))
    .parse_next(input)
}

/// Parses a single key-value pair
fn parse_key_value<'i>(
    input: &mut &'i str,
) -> PResult<(Vec<Cow<'i, str>>, Value<'i>), ContextError> {
    separated_pair(parse_dotted_key, '=', parse_value).parse_next(input)
}

/// Parses a dotted or single key
fn parse_dotted_key<'i>(input: &mut &'i str) -> PResult<Vec<Cow<'i, str>>, ContextError> {
    separated(1.., parse_key, '.').parse_next(input)
}

/// Parses a key (alphanumeric or underscores)
fn parse_key<'i>(input: &mut &'i str) -> PResult<Cow<'i, str>, ContextError> {
    // We don't use `parse_string` here beecause that also accept multiline strings and we don't
    // want that here.
    let string_key = alt((strings::parse_basic, strings::parse_literal)).map(|s| match s {
        Value::String(s) => s,
        _ => unreachable!(),
    });
    delimited(
        space0,
        alt((
            string_key,
            take_while(1.., |c: char| c.is_alphanumeric() || c == '_' || c == '-').map(Into::into),
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
            strings::parse,
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

/// Parses an integer value
fn parse_integer<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    numbers::integer(input).map(Into::into)
}

/// Parses a float value
fn parse_float<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    numbers::float(input).map(Into::into)
}

/// Parses a boolean value
fn parse_boolean<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    numbers::boolean(input).map(Into::into)
}

/// Parses an array of values
fn parse_array<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited('[', cut_err(parse_multiline_array_values), cut_err(']'))
        .map(Into::into)
        .parse_next(input)
}

fn parse_multiline_array_values<'i>(input: &mut &'i str) -> PResult<Array<'i>, ContextError> {
    if peek(opt(']')).parse_next(input)?.is_some() {
        // Optimize for empty arrays, avoiding `value` from being expected to fail
        return Ok(Array::new());
    }

    let array: Array<'i> = separated(0.., parse_multiline_array_value, ',').parse_next(input)?;

    if !array.is_empty() {
        // Ignore trailing comma, if present.
        opt(',').void().parse_next(input)?;
    }

    parse_whitespace_n_comments.void().parse_next(input)?;

    Ok(array)
}

fn parse_multiline_array_value<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    preceded(parse_whitespace_n_comments, parse_value).parse_next(input)
}

/// Parses an inline table
fn parse_inline_table<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited(
        '{',
        separated(0.., separated_pair(parse_key, '=', parse_value), ','),
        '}',
    )
    .map(|pairs: Vec<(Cow<'i, str>, Value<'i>)>| pairs.into_iter().collect())
    .parse_next(input)
}

/// Inserts a value into a nested map using a dotted key
fn insert_nested_key<'a>(map: &mut Table<'a>, keys: &[Cow<'a, str>], value: Value<'a>) {
    if let Some((first, rest)) = keys.split_first() {
        if rest.is_empty() {
            map.insert(first.clone(), value);
        } else {
            let entry = map
                .entry(first.clone())
                .or_insert_with(|| Table::new().into());

            if let Value::Table(ref mut nested_map) = entry {
                insert_nested_key(nested_map, rest, value);
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn issue_8() {
        use std::{
            thread::{sleep, spawn},
            time::Duration,
        };

        // Reproducer for #8: parsing of a deeply nested array took an **extremely** long time.
        let handle = spawn(|| super::parse("a=[[[[[[[[[[[[[[[[[[[[[[[[[[[").unwrap_err());
        sleep(Duration::from_millis(10));
        if !handle.is_finished() {
            panic!("parsing took way too long.");
        }
    }
}
