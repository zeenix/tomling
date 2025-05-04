use std::collections::HashMap;

use toml_test_harness::{DecodedScalar, DecodedValue, Decoder, DecoderHarness};
use tomling::{parse, Table, Value};

#[derive(Clone, Copy)]
struct Tomling;

impl Decoder for Tomling {
    fn name(&self) -> &str {
        "tomling"
    }

    fn decode(&self, data: &[u8]) -> Result<DecodedValue, toml_test_harness::Error> {
        fn inner(data: &[u8]) -> Result<DecodedValue, String> {
            let s = std::str::from_utf8(data).map_err(|e| e.to_string())?;
            let table = parse(s).map_err(|e| e.to_string())?;
            let table = map_table(&table);
            Ok(DecodedValue::Table(table))
        }

        inner(data).map_err(toml_test_harness::Error::new)
    }
}

fn map_table(table: &Table<'_>) -> HashMap<String, DecodedValue> {
    table
        .iter()
        .map(|(key, val)| (key.to_string(), value_to_decoded(val)))
        .collect()
}

fn value_to_decoded(value: &Value<'_>) -> DecodedValue {
    match value {
        Value::String(s) => DecodedValue::Scalar(s.to_string().into()),
        &Value::Integer(i) => DecodedValue::Scalar(i.into()),
        &Value::Float(f) => DecodedValue::Scalar(f.into()),
        &Value::Boolean(b) => DecodedValue::Scalar(b.into()),
        Value::Array(a) => DecodedValue::Array(a.iter().map(value_to_decoded).collect()),
        Value::Table(t) => DecodedValue::Table(map_table(t)),
        Value::Datetime(dt) => DecodedValue::Scalar(map_date_time(dt)),
    }
}

fn map_date_time(dt: &tomling::Datetime) -> DecodedScalar {
    let value = dt.to_string();

    match (dt.date.is_some(), dt.time.is_some(), dt.offset.is_some()) {
        (true, true, true) => DecodedScalar::Datetime(value),
        (true, true, false) => DecodedScalar::DatetimeLocal(value),
        (true, false, false) => DecodedScalar::DateLocal(value),
        (false, true, false) => DecodedScalar::TimeLocal(value),
        _ => unreachable!("Unsupported case"),
    }
}

#[test]
fn toml_test_harness() {
    let mut harness = DecoderHarness::new(Tomling);
    harness.version("1.0.0");
    harness
        .ignore([
            "valid/multibyte.toml",
            "valid/array/array-subtables.toml",
            "valid/array/string-quote-comma.toml",
            "valid/array/string-quote-comma-2.toml",
            "valid/array/table-array-string-backslash.toml",
            "valid/array/open-parent-table.toml",
            "valid/comment/tricky.toml",
            "valid/inline-table/empty.toml",
            "valid/inline-table/key-dotted-1.toml",
            "valid/inline-table/key-dotted-4.toml",
            "valid/inline-table/key-dotted-2.toml",
            "valid/inline-table/key-dotted-3.toml",
            "valid/inline-table/key-dotted-5.toml",
            "valid/inline-table/key-dotted-7.toml",
            "valid/inline-table/key-dotted-6.toml",
            "valid/key/escapes.toml",
            "valid/key/quoted-unicode.toml",
            "valid/key/space.toml",
            "valid/spec-1.0.0/inline-table-0.toml",
            "valid/key/quoted-dots.toml",
            "valid/spec-1.0.0/array-of-tables-1.toml",
            "valid/spec-1.0.0/string-0.toml",
            "valid/spec-1.0.0/string-4.toml",
            "valid/spec-1.0.0/table-0.toml",
            "valid/spec-1.0.0/string-7.toml",
            "valid/spec-1.0.0/string-2.toml",
            "valid/string/basic-escape-01.toml",
            "valid/spec-1.0.0/string-3.toml",
            "valid/spec-1.0.0/table-6.toml",
            "valid/spec-1.0.0/table-5.toml",
            "valid/string/basic-escape-03.toml",
            "valid/string/basic-escape-02.toml",
            "valid/string/escape-tricky.toml",
            "valid/spec-1.0.0/table-4.toml",
            "valid/string/ends-in-whitespace-escape.toml",
            "valid/string/escaped-escape.toml",
            "valid/string/multiline-quotes.toml",
            "valid/spec-1.0.0/table-3.toml",
            "valid/string/escapes.toml",
            "valid/string/multiline-escaped-crlf.toml",
            "valid/string/multiline-empty.toml",
            "valid/string/multibyte-escape.toml",
            "valid/string/raw-multiline.toml",
            "valid/string/nl.toml",
            "valid/string/quoted-unicode.toml",
            "valid/table/array-implicit.toml",
            "valid/table/empty.toml",
            "valid/table/no-eol.toml",
            "valid/table/keyword.toml",
            "valid/table/array-implicit-and-explicit-after.toml",
            "valid/string/start-mb.toml",
            "valid/table/sub-empty.toml",
            "valid/table/whitespace.toml",
            "valid/table/array-within-dotted.toml",
            "valid/string/multiline.toml",
            "valid/table/array-table-array.toml",
            "valid/table/without-super.toml",
            "valid/table/names.toml",
            "invalid/array/extend-defined-aot.toml",
            "valid/string/unicode-escape.toml",
            "valid/table/array-nest.toml",
            "invalid/array/extending-table.toml",
            "invalid/array/tables-01.toml",
            "invalid/control/bare-cr.toml",
            "invalid/control/multi-cr.toml",
            "invalid/control/multi-del.toml",
            "invalid/control/multi-us.toml",
            "invalid/control/multi-null.toml",
            "invalid/control/multi-lf.toml",
            "invalid/control/rawmulti-lf.toml",
            "invalid/control/rawmulti-null.toml",
            "invalid/control/rawmulti-us.toml",
            "invalid/control/rawstring-del.toml",
            "invalid/control/rawstring-cr.toml",
            "invalid/control/rawstring-lf.toml",
            "invalid/control/rawmulti-cr.toml",
            "invalid/control/rawstring-null.toml",
            "invalid/array/tables-02.toml",
            "invalid/control/rawstring-us.toml",
            "invalid/control/string-bs.toml",
            "invalid/control/string-cr.toml",
            "invalid/control/string-del.toml",
            "invalid/control/rawmulti-del.toml",
            "invalid/control/string-null.toml",
            "invalid/control/string-us.toml",
            "invalid/control/string-lf.toml",
            "invalid/inline-table/duplicate-key-01.toml",
            "invalid/inline-table/overwrite-02.toml",
            "invalid/inline-table/overwrite-01.toml",
            "invalid/inline-table/overwrite-05.toml",
            "invalid/inline-table/overwrite-04.toml",
            "invalid/inline-table/overwrite-03.toml",
            "invalid/inline-table/overwrite-09.toml",
            "invalid/key/after-array.toml",
            "invalid/key/after-table.toml",
            "invalid/key/duplicate-keys-01.toml",
            "invalid/key/after-value.toml",
            "invalid/key/duplicate-keys-03.toml",
            "invalid/key/duplicate-keys-04.toml",
            "invalid/key/duplicate-keys-02.toml",
            "invalid/key/dotted-redefine-table-01.toml",
            "invalid/key/duplicate-keys-07.toml",
            "invalid/key/newline-02.toml",
            "invalid/key/duplicate-keys-05.toml",
            "invalid/key/duplicate-keys-06.toml",
            "invalid/key/newline-03.toml",
            "invalid/key/dotted-redefine-table-02.toml",
            "invalid/key/no-eol-01.toml",
            "invalid/key/no-eol-02.toml",
            "invalid/key/special-character.toml",
            "invalid/key/no-eol-06.toml",
            "invalid/key/no-eol-04.toml",
            "invalid/key/no-eol-03.toml",
            "invalid/key/no-eol-07.toml",
            "invalid/string/bad-byte-escape.toml",
            "invalid/string/bad-escape-01.toml",
            "invalid/string/bad-escape-03.toml",
            "invalid/string/bad-escape-04.toml",
            "invalid/string/bad-escape-05.toml",
            "invalid/string/bad-escape-02.toml",
            "invalid/string/bad-hex-esc-02.toml",
            "invalid/string/bad-hex-esc-03.toml",
            "invalid/string/bad-hex-esc-01.toml",
            "invalid/string/bad-hex-esc-05.toml",
            "invalid/string/bad-hex-esc-04.toml",
            "invalid/string/bad-hex-esc-1.toml",
            "invalid/string/bad-hex-esc-2.toml",
            "invalid/string/bad-hex-esc-4.toml",
            "invalid/string/bad-hex-esc-3.toml",
            "invalid/string/bad-uni-esc-01.toml",
            "invalid/string/bad-multiline.toml",
            "invalid/string/bad-slash-escape.toml",
            "invalid/string/bad-uni-esc-03.toml",
            "invalid/string/bad-uni-esc-02.toml",
            "invalid/spec-1.0.0/inline-table-3-0.toml",
            "invalid/string/bad-uni-esc-04.toml",
            "invalid/string/bad-uni-esc-06.toml",
            "invalid/string/bad-uni-esc-05.toml",
            "invalid/spec-1.0.0/inline-table-2-0.toml",
            "invalid/string/bad-uni-esc-07.toml",
            "invalid/string/bad-uni-esc-1.toml",
            "invalid/string/bad-uni-esc-2.toml",
            "invalid/string/bad-uni-esc-5.toml",
            "invalid/string/bad-uni-esc-4.toml",
            "invalid/string/bad-uni-esc-3.toml",
            "invalid/string/bad-uni-esc-6.toml",
            "invalid/string/bad-uni-esc-7.toml",
            "invalid/string/bad-uni-esc-ml-1.toml",
            "invalid/string/bad-uni-esc-ml-2.toml",
            "invalid/string/bad-uni-esc-ml-3.toml",
            "invalid/string/bad-uni-esc-ml-4.toml",
            "invalid/string/basic-byte-escapes.toml",
            "invalid/string/basic-multiline-out-of-range-unicode-escape-01.toml",
            "invalid/string/bad-uni-esc-ml-7.toml",
            "invalid/string/bad-uni-esc-ml-5.toml",
            "invalid/string/bad-uni-esc-ml-6.toml",
            "invalid/string/basic-multiline-out-of-range-unicode-escape-02.toml",
            "invalid/string/basic-multiline-unknown-escape.toml",
            "invalid/string/basic-out-of-range-unicode-escape-01.toml",
            "invalid/string/basic-out-of-range-unicode-escape-02.toml",
            "invalid/string/basic-unknown-escape.toml",
            "invalid/string/multiline-bad-escape-01.toml",
            "invalid/string/multiline-escape-space-01.toml",
            "invalid/string/multiline-bad-escape-04.toml",
            "invalid/string/multiline-escape-space-02.toml",
            "invalid/string/multiline-bad-escape-02.toml",
            "invalid/string/multiline-bad-escape-03.toml",
            "invalid/spec-1.0.0/table-9-0.toml",
            "invalid/spec-1.0.0/table-9-1.toml",
            "invalid/string/no-close-09.toml",
            "invalid/string/no-close-10.toml",
            "invalid/table/duplicate-key-06.toml",
            "invalid/table/append-with-dotted-keys-06.toml",
            "invalid/table/duplicate-key-07.toml",
            "invalid/table/append-with-dotted-keys-03.toml",
            "invalid/table/append-with-dotted-keys-05.toml",
            "invalid/table/duplicate-key-04.toml",
            "invalid/table/duplicate-key-02.toml",
            "invalid/table/append-with-dotted-keys-04.toml",
            "invalid/table/duplicate-key-03.toml",
            "invalid/table/duplicate-key-05.toml",
            "invalid/table/duplicate-key-10.toml",
            "invalid/table/duplicate-key-01.toml",
            "invalid/table/append-with-dotted-keys-02.toml",
            "invalid/table/duplicate-key-09.toml",
            "invalid/table/newline-02.toml",
            "invalid/table/array-implicit.toml",
            "invalid/table/super-twice.toml",
            "invalid/table/overwrite-bool-with-array.toml",
            "invalid/table/append-with-dotted-keys-01.toml",
            "invalid/table/overwrite-with-deep-table.toml",
            "invalid/table/duplicate-key-08.toml",
            "invalid/table/redefine-03.toml",
            "invalid/table/redefine-01.toml",
            "invalid/table/overwrite-array-in-parent.toml",
            "invalid/table/redefine-02.toml",
        ])
        .unwrap();
    harness.test();
}
