use core::ops::RangeInclusive;

use winnow::{
    combinator::{alt, cut_err, opt, peek, preceded, repeat, trace},
    dispatch,
    error::{StrContext, StrContextValue},
    token::{one_of, rest, take},
    ModalResult, Parser,
};

// ;; Boolean

// boolean = true / false
pub(crate) fn boolean(input: &mut &str) -> ModalResult<bool> {
    trace("boolean", alt((true_, false_))).parse_next(input)
}

fn true_(input: &mut &str) -> ModalResult<bool> {
    (peek(TRUE), cut_err(TRUE)).value(true).parse_next(input)
}

fn false_(input: &mut &str) -> ModalResult<bool> {
    (peek(FALSE), cut_err(FALSE)).value(false).parse_next(input)
}
const TRUE: &str = "true";
const FALSE: &str = "false";

// ;; Integer

// integer = dec-int / hex-int / oct-int / bin-int
pub(crate) fn integer(input: &mut &str) -> ModalResult<i64> {
    trace("integer",
    dispatch! {peek(opt::<_, &str, _, _>(take(2usize)));
        Some("0x") => cut_err(hex_int.try_map(|s| i64::from_str_radix(&s.replace('_', ""), 16))),
        Some("0o") => cut_err(oct_int.try_map(|s| i64::from_str_radix(&s.replace('_', ""), 8))),
        Some("0b") => cut_err(bin_int.try_map(|s| i64::from_str_radix(&s.replace('_', ""), 2))),
        _ => dec_int.and_then(cut_err(rest
            .try_map(|s: &str| s.replace('_', "").parse())))
    })
    .parse_next(input)
}

// dec-int = [ minus / plus ] unsigned-dec-int
// unsigned-dec-int = DIGIT / digit1-9 1*( DIGIT / underscore DIGIT )
fn dec_int<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    trace(
        "dec-int",
        (
            opt(one_of((b'+', b'-'))),
            alt((
                (
                    one_of(DIGIT1_9),
                    repeat(
                        0..,
                        alt((
                            digit.void(),
                            (
                                one_of(b'_'),
                                cut_err(digit).context(StrContext::Expected(
                                    StrContextValue::Description("digit"),
                                )),
                            )
                                .void(),
                        )),
                    )
                    .map(|()| ()),
                )
                    .void(),
                digit.void(),
            )),
        )
            .take()
            .context(StrContext::Label("integer")),
    )
    .parse_next(input)
}
const DIGIT1_9: RangeInclusive<u8> = b'1'..=b'9';

// hex-prefix = %x30.78               ; 0x
// hex-int = hex-prefix HEXDIG *( HEXDIG / underscore HEXDIG )
fn hex_int<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    trace(
        "hex-int",
        preceded(
            HEX_PREFIX,
            cut_err((
                hexdig,
                repeat(
                    0..,
                    alt((
                        hexdig.void(),
                        (
                            one_of('_'),
                            cut_err(hexdig).context(StrContext::Expected(
                                StrContextValue::Description("digit"),
                            )),
                        )
                            .void(),
                    )),
                )
                .map(|()| ()),
            ))
            .take(),
        )
        .context(StrContext::Label("hexadecimal integer")),
    )
    .parse_next(input)
}
const HEX_PREFIX: &str = "0x";

// oct-prefix = %x30.6F               ; 0o
// oct-int = oct-prefix digit0-7 *( digit0-7 / underscore digit0-7 )
fn oct_int<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    trace(
        "oct-int",
        preceded(
            OCT_PREFIX,
            cut_err((
                one_of(DIGIT0_7),
                repeat(
                    0..,
                    alt((
                        one_of(DIGIT0_7).void(),
                        (
                            one_of(b'_'),
                            cut_err(one_of(DIGIT0_7)).context(StrContext::Expected(
                                StrContextValue::Description("digit"),
                            )),
                        )
                            .void(),
                    )),
                )
                .map(|()| ()),
            ))
            .take(),
        )
        .context(StrContext::Label("octal integer")),
    )
    .parse_next(input)
}
const OCT_PREFIX: &str = "0o";
const DIGIT0_7: RangeInclusive<u8> = b'0'..=b'7';

// bin-prefix = %x30.62               ; 0b
// bin-int = bin-prefix digit0-1 *( digit0-1 / underscore digit0-1 )
fn bin_int<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    trace(
        "bin-int",
        preceded(
            BIN_PREFIX,
            cut_err((
                one_of(DIGIT0_1),
                repeat(
                    0..,
                    alt((
                        one_of(DIGIT0_1).void(),
                        (
                            one_of(b'_'),
                            cut_err(one_of(DIGIT0_1)).context(StrContext::Expected(
                                StrContextValue::Description("digit"),
                            )),
                        )
                            .void(),
                    )),
                )
                .map(|()| ()),
            ))
            .take(),
        )
        .context(StrContext::Label("binary integer")),
    )
    .parse_next(input)
}
const BIN_PREFIX: &str = "0b";
const DIGIT0_1: RangeInclusive<u8> = b'0'..=b'1';

// ;; Float

// float = float-int-part ( exp / frac [ exp ] )
// float =/ special-float
// float-int-part = dec-int
pub(crate) fn float(input: &mut &str) -> ModalResult<f64> {
    trace(
        "float",
        alt((
            float_.and_then(cut_err(
                rest.try_map(|s: &str| s.replace('_', "").parse())
                    .verify(|f: &f64| *f != f64::INFINITY),
            )),
            special_float,
        ))
        .context(StrContext::Label("floating-point number")),
    )
    .parse_next(input)
}

fn float_<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    (
        dec_int,
        alt((exp.void(), (frac.void(), opt(exp.void())).void())),
    )
        .take()
        .parse_next(input)
}

// frac = decimal-point zero-prefixable-int
// decimal-point = %x2E               ; .
fn frac<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    (
        '.',
        cut_err(zero_prefixable_int)
            .context(StrContext::Expected(StrContextValue::Description("digit"))),
    )
        .take()
        .parse_next(input)
}

// zero-prefixable-int = DIGIT *( DIGIT / underscore DIGIT )
fn zero_prefixable_int<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    (
        digit,
        repeat(
            0..,
            alt((
                digit.void(),
                (
                    one_of(b'_'),
                    cut_err(digit)
                        .context(StrContext::Expected(StrContextValue::Description("digit"))),
                )
                    .void(),
            )),
        )
        .map(|()| ()),
    )
        .take()
        .parse_next(input)
}

// exp = "e" float-exp-part
// float-exp-part = [ minus / plus ] zero-prefixable-int
fn exp<'i>(input: &mut &'i str) -> ModalResult<&'i str> {
    (
        one_of((b'e', b'E')),
        opt(one_of([b'+', b'-'])),
        cut_err(zero_prefixable_int),
    )
        .take()
        .parse_next(input)
}

// special-float = [ minus / plus ] ( inf / nan )
fn special_float(input: &mut &str) -> ModalResult<f64> {
    (opt(one_of((b'+', b'-'))), alt((inf, nan)))
        .map(|(s, f)| match s {
            Some('+') | None => f,
            Some('-') => -f,
            _ => unreachable!("one_of should prevent this"),
        })
        .parse_next(input)
}
// inf = %x69.6e.66  ; inf
fn inf(input: &mut &str) -> ModalResult<f64> {
    INF.value(f64::INFINITY).parse_next(input)
}
const INF: &str = "inf";
// nan = %x6e.61.6e  ; nan
fn nan(input: &mut &str) -> ModalResult<f64> {
    NAN.value(f64::NAN).parse_next(input)
}
const NAN: &str = "nan";

// DIGIT = %x30-39 ; 0-9
fn digit(input: &mut &str) -> ModalResult<char> {
    one_of(DIGIT).parse_next(input)
}
const DIGIT: RangeInclusive<u8> = b'0'..=b'9';

// HEXDIG = DIGIT / "A" / "B" / "C" / "D" / "E" / "F"
fn hexdig(input: &mut &str) -> ModalResult<char> {
    one_of(HEXDIG).parse_next(input)
}
const HEXDIG: (RangeInclusive<u8>, RangeInclusive<u8>, RangeInclusive<u8>) =
    (DIGIT, b'A'..=b'F', b'a'..=b'f');
