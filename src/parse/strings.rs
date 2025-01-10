use crate::Value;

use winnow::{
    combinator::{alt, delimited, preceded},
    error::ContextError,
    token::{take_until, take_while},
    PResult, Parser,
};

/// Parses a string value enclosed in quotes
pub(crate) fn parse<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    alt((
        parse_multiline_basic,
        parse_basic,
        parse_multiline_literal,
        parse_literal,
    ))
    .parse_next(input)
}

/// Parses a basic string value enclosed in quotes.
pub(crate) fn parse_basic<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited('"', parse_basic_inner, '"')
        .map(Into::into)
        .parse_next(input)
}

fn parse_basic_inner<'i>(input: &mut &'i str) -> PResult<&'i str, ContextError> {
    take_while(0.., |c| c != '\\' && c != '"')
        .and_then(alt((
            preceded('\\', parse_escape_sequence),
            take_while(0.., |c| c != '\\' && c != '"'),
        )))
        .parse_next(input)
}

fn parse_escape_sequence<'i>(input: &mut &'i str) -> PResult<&'i str, ContextError> {
    alt((
        "\\\"".value("\""),
        "\\\\".value("\\"),
        "\\b".value("\x08"),
        "\\t".value("\t"),
        "\\n".value("\n"),
        "\\f".value("\x0C"),
        "\\r".value("\r"),
        preceded("\\u", take_while(4, |c: char| c.is_ascii_hexdigit())),
        preceded("\\U", take_while(8, |c: char| c.is_ascii_hexdigit())),
    ))
    .parse_next(input)
}

/// Parses a literal string value enclosed in single quotes.
pub(crate) fn parse_literal<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited('\'', take_until(0.., '\''), '\'')
        .map(Into::into)
        .parse_next(input)
}

/// Parses a multiline basic string value enclosed in triple quotes.
pub(crate) fn parse_multiline_basic<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited(
        "\"\"\"",
        parse_multiline_basic_inner.map(|s: &str| {
            // Trim leading newlines.
            s.trim_start_matches('\n')
        }),
        "\"\"\"",
    )
    .map(Into::into)
    .parse_next(input)
}

fn parse_multiline_basic_inner<'i>(input: &mut &'i str) -> PResult<&'i str, ContextError> {
    take_while(0.., |c| c != '\\' && c != '"')
        .and_then(alt((
            preceded('\\', parse_escape_sequence),
            take_while(0.., |c| c != '\\' && c != '"'),
        )))
        .parse_next(input)
}

/// Parses a literal multiline string value enclosed in triple single quotes (`'''`).
pub(crate) fn parse_multiline_literal<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    delimited(
        "'''",
        take_until(0.., "'''").map(|s: &str| s.trim_start_matches('\n')), // Trim leading newlines
        "'''",
    )
    .map(Into::into)
    .parse_next(input)
}
