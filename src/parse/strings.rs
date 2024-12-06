use crate::Value;

use winnow::{
    combinator::{alt, delimited},
    error::ContextError,
    token::take_until,
    PResult, Parser,
};

/// Parses a string value enclosed in quotes
pub(crate) fn parse<'i>(input: &mut &'i str) -> PResult<Value<'i>, ContextError> {
    // TODO:
    // * Handle escape sequences.
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
    delimited('"', take_until(0.., '"'), '"')
        .map(Into::into)
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
        take_until(0.., "\"\"\"").map(|s: &str| {
            // Trim leading newlines.
            s.trim_start_matches('\n')
        }),
        "\"\"\"",
    )
    .map(Into::into)
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
