use std::collections::HashMap;

use toml_test_harness::{Decoded, DecodedValue, Decoder, DecoderHarness};
use tomling::{parse, Table, Value};

#[derive(Clone, Copy)]
struct Tomling;

impl Decoder for Tomling {
    fn name(&self) -> &str {
        "tomling"
    }

    fn decode(&self, data: &[u8]) -> Result<Decoded, toml_test_harness::Error> {
        fn inner(data: &[u8]) -> Result<Decoded, String> {
            let s = std::str::from_utf8(data).map_err(|e| e.to_string())?;
            let table = parse(s).map_err(|e| e.to_string())?;
            let table = map_table(&table);
            Ok(Decoded::Table(table))
        }

        inner(data).map_err(toml_test_harness::Error::new)
    }
}

fn map_table(table: &Table<'_>) -> HashMap<String, Decoded> {
    table
        .iter()
        .map(|(key, val)| (key.to_string(), value_to_decoded(val)))
        .collect()
}

fn value_to_decoded(value: &Value<'_>) -> Decoded {
    match value {
        Value::String(s) => Decoded::Value(s.to_string().into()),
        &Value::Integer(i) => Decoded::Value(i.into()),
        &Value::Float(f) => Decoded::Value(f.into()),
        &Value::Boolean(b) => Decoded::Value(b.into()),
        Value::Array(a) => Decoded::Array(a.iter().map(value_to_decoded).collect()),
        Value::Table(t) => Decoded::Table(map_table(t)),
        Value::Datetime(dt) => Decoded::Value(map_date_time(dt)),
    }
}

fn map_date_time(dt: &tomling::Datetime) -> DecodedValue {
    let value = dt.to_string();

    match (dt.date.is_some(), dt.time.is_some(), dt.offset.is_some()) {
        (true, true, true) => DecodedValue::Datetime(value),
        (true, true, false) => DecodedValue::DatetimeLocal(value),
        (true, false, false) => DecodedValue::DateLocal(value),
        (false, true, false) => DecodedValue::TimeLocal(value),
        _ => unreachable!("Unsupported case"),
    }
}

#[test]
fn toml_test_harness() {
    let mut harness = DecoderHarness::new(Tomling);
    harness.version("1.0.0");
    harness
        .ignore([
            "valid/array/array-subtables.toml",
            "valid/array/open-parent-table.toml",
            "valid/array/string-quote-comma-2.toml",
            "valid/array/string-quote-comma.toml",
            "valid/array/table-array-string-backslash.toml",
            "valid/comment/tricky.toml",
            "valid/inline-table/empty.toml",
            "valid/inline-table/key-dotted-1.toml",
            "valid/inline-table/key-dotted-2.toml",
            "valid/inline-table/key-dotted-3.toml",
            "valid/inline-table/key-dotted-4.toml",
            "valid/inline-table/key-dotted-5.toml",
            "valid/inline-table/key-dotted-6.toml",
            "valid/inline-table/key-dotted-7.toml",
            "valid/key/escapes.toml",
            "valid/key/quoted-unicode.toml",
            "valid/key/space.toml",
            "valid/spec/array-of-tables-1.toml",
            "valid/spec/inline-table-0.toml",
            "valid/spec/string-0.toml",
            "valid/spec/string-2.toml",
            "valid/spec/string-3.toml",
            "valid/spec/string-4.toml",
            "valid/spec/string-7.toml",
            "valid/spec/table-0.toml",
            "valid/spec/table-3.toml",
            "valid/spec/table-4.toml",
            "valid/spec/table-5.toml",
            "valid/spec/table-6.toml",
            "valid/string/double-quote-escape.toml",
            "valid/string/ends-in-whitespace-escape.toml",
            "valid/string/escape-tricky.toml",
            "valid/string/escaped-escape.toml",
            "valid/string/escapes.toml",
            "valid/string/multiline-empty.toml",
            "valid/string/multiline-escaped-crlf.toml",
            "valid/string/multiline-quotes.toml",
            "valid/string/multiline.toml",
            "valid/string/nl.toml",
            "valid/string/quoted-unicode.toml",
            "valid/string/raw-multiline.toml",
            "valid/string/start-mb.toml",
            "valid/string/unicode-escape.toml",
            "valid/table/array-implicit-and-explicit-after.toml",
            "valid/table/array-implicit.toml",
            "valid/table/array-nest.toml",
            "valid/table/array-table-array.toml",
            "valid/table/array-within-dotted.toml",
            "valid/table/empty.toml",
            "valid/table/keyword.toml",
            "valid/table/names.toml",
            "valid/table/no-eol.toml",
            "valid/table/sub-empty.toml",
            "valid/table/whitespace.toml",
            "valid/table/without-super.toml",
            "invalid/array/extend-defined-aot.toml",
            "invalid/array/extending-table.toml",
            "invalid/array/tables-1.toml",
            "invalid/array/tables-2.toml",
            "invalid/control/bare-cr.toml",
            "invalid/control/multi-cr.toml",
            "invalid/control/multi-del.toml",
            "invalid/control/multi-lf.toml",
            "invalid/control/multi-null.toml",
            "invalid/control/multi-us.toml",
            "invalid/control/rawmulti-cd.toml",
            "invalid/control/rawmulti-del.toml",
            "invalid/control/rawmulti-lf.toml",
            "invalid/control/rawmulti-null.toml",
            "invalid/control/rawmulti-us.toml",
            "invalid/control/rawstring-cr.toml",
            "invalid/control/rawstring-del.toml",
            "invalid/control/rawstring-lf.toml",
            "invalid/control/rawstring-null.toml",
            "invalid/control/rawstring-us.toml",
            "invalid/control/string-bs.toml",
            "invalid/control/string-cr.toml",
            "invalid/control/string-del.toml",
            "invalid/control/string-lf.toml",
            "invalid/control/string-null.toml",
            "invalid/control/string-us.toml",
            "invalid/inline-table/duplicate-key-1.toml",
            "invalid/inline-table/overwrite-01.toml",
            "invalid/inline-table/overwrite-02.toml",
            "invalid/inline-table/overwrite-03.toml",
            "invalid/inline-table/overwrite-04.toml",
            "invalid/inline-table/overwrite-05.toml",
            "invalid/inline-table/overwrite-09.toml",
            "invalid/key/after-array.toml",
            "invalid/key/after-table.toml",
            "invalid/key/after-value.toml",
            "invalid/key/dotted-redefine-table-1.toml",
            "invalid/key/dotted-redefine-table-2.toml",
            "invalid/key/duplicate-keys-1.toml",
            "invalid/key/duplicate-keys-2.toml",
            "invalid/key/duplicate-keys-3.toml",
            "invalid/key/duplicate-keys-4.toml",
            "invalid/key/newline-2.toml",
            "invalid/key/newline-3.toml",
            "invalid/key/no-eol.toml",
            "invalid/key/special-character.toml",
            "invalid/spec/inline-table-2-0.toml",
            "invalid/spec/inline-table-3-0.toml",
            "invalid/spec/table-9-0.toml",
            "invalid/spec/table-9-1.toml",
            "invalid/string/bad-byte-escape.toml",
            "invalid/string/bad-escape-1.toml",
            "invalid/string/bad-escape-2.toml",
            "invalid/string/bad-escape-3.toml",
            "invalid/string/bad-hex-esc-1.toml",
            "invalid/string/bad-hex-esc-2.toml",
            "invalid/string/bad-hex-esc-3.toml",
            "invalid/string/bad-hex-esc-4.toml",
            "invalid/string/bad-hex-esc-5.toml",
            "invalid/string/bad-multiline.toml",
            "invalid/string/bad-slash-escape.toml",
            "invalid/string/bad-uni-esc-1.toml",
            "invalid/string/bad-uni-esc-2.toml",
            "invalid/string/bad-uni-esc-3.toml",
            "invalid/string/bad-uni-esc-4.toml",
            "invalid/string/bad-uni-esc-5.toml",
            "invalid/string/bad-uni-esc-6.toml",
            "invalid/string/bad-uni-esc-7.toml",
            "invalid/string/basic-byte-escapes.toml",
            "invalid/string/basic-multiline-out-of-range-unicode-escape-1.toml",
            "invalid/string/basic-multiline-out-of-range-unicode-escape-2.toml",
            "invalid/string/basic-multiline-unknown-escape.toml",
            "invalid/string/basic-out-of-range-unicode-escape-1.toml",
            "invalid/string/basic-out-of-range-unicode-escape-2.toml",
            "invalid/string/basic-unknown-escape.toml",
            "invalid/string/multiline-bad-escape-1.toml",
            "invalid/string/multiline-bad-escape-2.toml",
            "invalid/string/multiline-bad-escape-3.toml",
            "invalid/string/multiline-bad-escape-4.toml",
            "invalid/string/multiline-escape-space-1.toml",
            "invalid/string/multiline-escape-space-2.toml",
            "invalid/table/append-to-array-with-dotted-keys.toml",
            "invalid/table/append-with-dotted-keys-1.toml",
            "invalid/table/append-with-dotted-keys-2.toml",
            "invalid/table/array-implicit.toml",
            "invalid/table/duplicate-key-dotted-array.toml",
            "invalid/table/duplicate-key-dotted-table.toml",
            "invalid/table/duplicate-key-dotted-table2.toml",
            "invalid/table/duplicate-key-table.toml",
            "invalid/table/duplicate-table-array.toml",
            "invalid/table/duplicate-table-array2.toml",
            "invalid/table/duplicate.toml",
            "invalid/table/overwrite-array-in-parent.toml",
            "invalid/table/overwrite-bool-with-array.toml",
            "invalid/table/overwrite-with-deep-table.toml",
            "invalid/table/redefine-1.toml",
            "invalid/table/redefine-2.toml",
            "invalid/table/redefine-3.toml",
            "invalid/table/super-twice.toml",
        ])
        .unwrap();
    harness.test();
}
