#[test]
fn simple_cargo_toml() {
    use tomling::{parse, TomlMap, Value};

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
    let mut map = TomlMap::new();
    map.insert(
        "package",
        Value::Table({
            let mut package = TomlMap::new();
            package.insert("name", Value::String("example"));
            package.insert("version", Value::String("0.1.0"));
            package.insert("edition", Value::String("2021"));
            package
        }),
    );
    map.insert(
        "dependencies",
        Value::Table({
            let mut dependencies = TomlMap::new();
            dependencies.insert(
                "serde",
                Value::Table({
                    let mut serde = TomlMap::new();
                    serde.insert("version", Value::String("1.0"));
                    serde.insert(
                        "features",
                        Value::Array(vec![Value::String("std"), Value::String("derive")]),
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
            let mut features = TomlMap::new();
            features.insert("default", Value::Array(vec![Value::String("serde")]));
            features
        }),
    );

    let parsed_map = parse(cargo_toml).unwrap();
    assert_eq!(parsed_map, map);
}
