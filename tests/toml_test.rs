use std::collections::HashMap;

use toml_test_harness::{Decoded, Decoder, DecoderHarness};
use tomling::{parse, Table, Value};

#[derive(Clone, Copy)]
struct Tomling;

impl Decoder for Tomling {
    fn name(&self) -> &str {
        "tomling"
    }

    fn decode(&self, data: &[u8]) -> Result<Decoded, toml_test_harness::Error> {
        fn inner(data: &[u8]) -> Result<Decoded, Box<dyn std::error::Error>> {
            let s = std::str::from_utf8(data)?;
            let table = parse(s)?;
            let table = map_table(&table);
            Ok(Decoded::Table(table))
        }

        inner(data).map_err(toml_test_harness::Error::new)
    }
}

fn map_table(table: &Table<'_>) -> HashMap<String, Decoded> {
    table
        .iter()
        .map(|(key, val)| (key.to_owned(), value_to_decoded(val)))
        .collect()
}

fn value_to_decoded(value: &Value<'_>) -> Decoded {
    match value {
        &Value::String(s) => Decoded::Value(s.into()),
        &Value::Integer(i) => Decoded::Value(i.into()),
        &Value::Float(f) => Decoded::Value(f.into()),
        &Value::Boolean(b) => Decoded::Value(b.into()),
        Value::Array(a) => Decoded::Array(a.iter().map(value_to_decoded).collect()),
        Value::Table(t) => Decoded::Table(map_table(t)),
    }
}

#[test]
fn toml_test_harness() {
    let mut harness = DecoderHarness::new(Tomling);
    harness.version("1.0.0");
    harness
        .ignore([
            "/invalid/array/extend*",
            "/invalid/array/tables-*",
            "/invalid/control/bare-cr.toml",
            "/invalid/control/comment-*",
            "/invalid/control/multi-*",
            "/invalid/control/rawmulti-*",
            "/invalid/control/rawstring-*",
            "/invalid/control/string-*",
            "/invalid/float/leading-zero.toml",
            "/invalid/inline-table/duplicate-key-1.toml",
            "/invalid/inline-table/overwrite-01.toml",
            "/invalid/inline-table/overwrite-02.toml",
            "/invalid/inline-table/overwrite-03.toml",
            "/invalid/inline-table/overwrite-04.toml",
            "/invalid/inline-table/overwrite-05.toml",
            "/invalid/inline-table/overwrite-09.toml",
            "/invalid/integer/leading-zero-1.toml",
            "/invalid/integer/leading-zero-2.toml",
            "/invalid/key/after-*",
            "/invalid/key/dotted-redefine-table-*",
            "/invalid/key/duplicate-keys-*",
            "/invalid/key/newline-2.toml",
            "/invalid/key/newline-3.toml",
            "/invalid/key/no-eol.toml",
            "/invalid/key/special-character.toml",
            "/invalid/spec/inline-table-*",
            "/invalid/spec/table-9-*",
            "/invalid/string/bad-byte-escape.toml",
            "/invalid/string/bad-escape-*",
            "/invalid/string/bad-hex-esc-*",
            "/invalid/string/bad-multiline.toml",
            "/invalid/string/bad-slash-escape.toml",
            "/invalid/string/bad-uni-esc-*",
            "/invalid/string/basic-byte-escapes.toml",
            "/invalid/string/basic-multiline-out-of-range-unicode-escape-*",
            "/invalid/string/basic-multiline-unknown-escape.toml",
            "/invalid/string/basic-out-of-range-unicode-escape-*",
            "/invalid/string/basic-unknown-escape.toml",
            "/invalid/string/multiline-bad-escape-*",
            "/invalid/string/multiline-escape-space-*",
            "/invalid/table/append-to-array-with-dotted-keys.toml",
            "/invalid/table/append-with-dotted-keys-*",
            "/invalid/table/array-implicit.toml",
            "/invalid/table/duplicate*",
            "/invalid/table/overwrite-*",
            "/invalid/table/redefine-*",
            "/invalid/table/super-twice.toml",
            "/valid/array/array-subtables.toml",
            "/valid/array/array.toml",
            "/valid/array/mixed-string-table.toml",
            "/valid/array/nested-double.toml",
            "/valid/array/open-parent-table.toml",
            "/valid/array/string-quote-comma*",
            "/valid/array/string-with-comma*",
            "/valid/array/table-array-string-backslash.toml",
            "/valid/comment/after-literal-no-ws.toml",
            "/valid/comment/everywhere.toml",
            "/valid/comment/noeol.toml",
            "/valid/comment/tricky.toml",
            "/valid/datetime",
            "/valid/empty-file.toml",
            "/valid/example.toml",
            "/valid/float",
            "/valid/inline-table/empty.toml",
            "/valid/inline-table/key-dotted-*",
            "/valid/integer",
            "/valid/key/escapes.toml",
            "/valid/key/quoted-unicode.toml",
            "/valid/key/space.toml",
            "/valid/spec-example-1-compact.toml",
            "/valid/spec-example-1.toml",
            "/valid/spec/array-0.toml",
            "/valid/spec/array-1.toml",
            "/valid/spec/array-of-tables-1.toml",
            "/valid/spec/float-*",
            "/valid/spec/inline-table-0.toml",
            "/valid/spec/integer-*",
            "/valid/spec/local-*",
            "/valid/spec/offset-date-time-*",
            "/valid/spec/string-0.toml",
            "/valid/spec/string-2.toml",
            "/valid/spec/string-3.toml",
            "/valid/spec/string-4.toml",
            "/valid/spec/string-7.toml",
            "/valid/spec/table-0.toml",
            "/valid/spec/table-3.toml",
            "/valid/spec/table-4.toml",
            "/valid/spec/table-5.toml",
            "/valid/spec/table-6.toml",
            "/valid/spec/table-7.toml",
            "/valid/string/double-quote-escape.toml",
            "/valid/string/ends-in-whitespace-escape.toml",
            "/valid/string/escape-tricky.toml",
            "/valid/string/escaped-escape.toml",
            "/valid/string/escapes.toml",
            "/valid/string/multiline-empty.toml",
            "/valid/string/multiline-escaped-crlf.toml",
            "/valid/string/multiline-quotes.toml",
            "/valid/string/multiline.toml",
            "/valid/string/nl.toml",
            "/valid/string/quoted-unicode.toml",
            "/valid/string/raw-multiline.toml",
            "/valid/string/start-mb.toml",
            "/valid/string/unicode-escape.toml",
            "/valid/table/array-implicit*",
            "/valid/table/array-nest.toml",
            "/valid/table/array-table-array.toml",
            "/valid/table/array-within-dotted.toml",
            "/valid/table/empty.toml",
            "/valid/table/keyword.toml",
            "/valid/table/names.toml",
            "/valid/table/no-eol.toml",
            "/valid/table/sub-empty.toml",
            "/valid/table/whitespace.toml",
            "/valid/table/without-super.toml",
        ])
        .unwrap();
    harness.test();
}
