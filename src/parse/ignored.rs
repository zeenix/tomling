use winnow::{
    ascii::space0,
    combinator::{alt, empty, eof, fail, opt, peek, preceded},
    dispatch,
    stream::Stream as _,
    token::{any, one_of, take_while},
    PResult, Parser,
};

/// Parse a comment, w/o the trailing newline.
pub(crate) fn parse_comment(input: &mut &str) -> PResult<()> {
    preceded(
        '#',
        take_while(
            0..,
            // > Control characters other than tab (U+0000 to U+0008, U+000A to U+001F, U+007F) are
            // > not permitted in comments.
            |c| !matches!(c, '\0'..='\u{08}' | '\u{0a}'..='\u{1f}' | '\u{7f}'),
        ),
    )
    .void()
    .parse_next(input)
}

/// Parses a comment and newline (unless at EOF).
pub(crate) fn parse_comment_newline(input: &mut &str) -> PResult<()> {
    (parse_comment, alt((newline, eof.void())))
        .void()
        .parse_next(input)
}

/// Parse all whitespace (including newlines) and comments.
pub(crate) fn parse_whitespace_n_comments(input: &mut &str) -> PResult<()> {
    let mut start = input.checkpoint();
    loop {
        let _ = space0.parse_next(input)?;

        let next_token = opt(peek(any)).parse_next(input)?;
        match next_token {
            Some('#') => (parse_comment, newline).void().parse_next(input)?,
            Some('\n') => (newline).void().parse_next(input)?,
            Some('\r') => (newline).void().parse_next(input)?,
            _ => break,
        }

        let end = input.checkpoint();
        if start == end {
            break;
        }
        start = end;
    }

    Ok(())
}

/// Parse a newline.
pub(crate) fn newline(input: &mut &str) -> PResult<()> {
    dispatch! {any;
        '\n' => empty,
        '\r' => one_of('\n').void(),
        _ => fail,
    }
    .parse_next(input)
}
