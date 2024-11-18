#[test]
fn simple_cargo_toml() {
    use tomling::{parse, Table, Value};

    let cargo_toml = r#"
        [package]
        name = "example"
        version = "0.1.0"
        edition = "2021"

        # This is a comment.
        [dependencies]
        # a comment.
        serde = { version = "1.0", features = [
            # A comment here.
            "std",
            # A multiline
            # comment here.
            "derive", # and here.
        ] }
        regex = "1.5" # This is also a comment.

        [features]
        default = ["serde"]
    "#;
    let mut map = Table::new();
    map.insert(
        "package",
        Value::Table({
            let mut package = Table::new();
            package.insert("name", Value::String("example"));
            package.insert("version", Value::String("0.1.0"));
            package.insert("edition", Value::String("2021"));
            package
        }),
    );
    map.insert(
        "dependencies",
        Value::Table({
            let mut dependencies = Table::new();
            dependencies.insert(
                "serde",
                Value::Table({
                    let mut serde = Table::new();
                    serde.insert("version", Value::String("1.0"));
                    serde.insert(
                        "features",
                        Value::Array(
                            [Value::String("std"), Value::String("derive")]
                                .into_iter()
                                .collect(),
                        ),
                    );
                    serde
                }),
            );
            dependencies.insert("regex", Value::String("1.5"));
            dependencies
        }),
    );
    map.insert(
        "features",
        Value::Table({
            let mut features = Table::new();
            features.insert(
                "default",
                Value::Array([Value::String("serde")].into_iter().collect()),
            );
            features
        }),
    );

    let parsed_map = parse(cargo_toml).unwrap();
    assert_eq!(parsed_map, map);
}
