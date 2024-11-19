#[test]
fn simple_cargo_toml() {
    use tomling::{parse, Table, Value};

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

    let parsed_map = parse(CARGO_TOML).unwrap();
    assert_eq!(parsed_map, map);
}

#[cfg(feature = "serde")]
#[test]
fn simple_cargo_toml_serde() {
    // Make use of serde derives
    use serde::Deserialize;
    use tomling::from_str;

    #[derive(Debug, PartialEq, Deserialize)]
    struct Package<'a> {
        name: &'a str,
        version: &'a str,
        edition: &'a str,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct Dependencies<'a> {
        #[serde(borrow)]
        serde: Serde<'a>,
        regex: Regex,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct Serde<'a> {
        version: &'a str,
        features: Vec<&'a str>,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct Regex(String);

    #[derive(Debug, PartialEq, Deserialize)]
    struct Features<'a> {
        #[serde(borrow)]
        default: Vec<&'a str>,
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct CargoToml<'a> {
        #[serde(borrow)]
        package: Package<'a>,
        dependencies: Dependencies<'a>,
        features: Features<'a>,
    }

    let table = from_str::<CargoToml>(CARGO_TOML).unwrap();
    assert_eq!(
        table,
        CargoToml {
            package: Package {
                name: "example",
                version: "0.1.0",
                edition: "2021",
            },
            dependencies: Dependencies {
                serde: Serde {
                    version: "1.0",
                    features: vec!["std", "derive"],
                },
                regex: Regex("1.5".to_string()),
            },
            features: Features {
                default: vec!["serde"],
            },
        }
    );
}

const CARGO_TOML: &'static str = r#"
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
