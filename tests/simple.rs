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
            package.insert("resolver", Value::String("2"));
            package.insert(
                "authors",
                Value::Array(
                    [
                        Value::String("Alice Great <foo@bar.com>"),
                        Value::String("Bob Less"),
                    ]
                    .into_iter()
                    .collect(),
                ),
            );
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
        "target",
        Value::Table({
            let mut target = Table::new();
            target.insert(
                "cfg(unix)",
                Value::Table({
                    let mut cfg_unix = Table::new();
                    cfg_unix.insert(
                        "build-dependencies",
                        Value::Table({
                            let mut build_dependencies = Table::new();
                            build_dependencies.insert("cc", Value::String("1.0.3"));

                            build_dependencies
                        }),
                    );

                    cfg_unix
                }),
            );

            target
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
    map.insert(
        "bin",
        Value::Array(
            [Value::Table({
                let mut bin = Table::new();
                bin.insert("name", Value::String("some-binary"));
                bin.insert("path", Value::String("src/bin/my-binary.rs"));
                bin
            })]
            .into_iter()
            .collect(),
        ),
    );

    let parsed_map = parse(CARGO_TOML).unwrap();
    assert_eq!(parsed_map, map);
}

#[cfg(feature = "cargo-toml")]
#[test]
fn simple_cargo_toml_serde() {
    use tomling::cargo::{BuildDependency, Dependency, Manifest, ResolverVersion, RustEdition};

    let manifest: Manifest = tomling::from_str(CARGO_TOML).unwrap();

    assert_eq!(manifest.package().name(), "example");
    assert_eq!(manifest.package().version(), "0.1.0");
    assert_eq!(manifest.package().edition().unwrap(), RustEdition::E2021);
    assert_eq!(manifest.package().resolver().unwrap(), ResolverVersion::V2);
    let authors = manifest.package().authors().unwrap();
    let alice = &authors[0];
    assert_eq!(alice.name(), "Alice Great");
    assert_eq!(alice.email(), Some("foo@bar.com"));
    let bob = &authors[1];
    assert_eq!(bob.name(), "Bob Less");
    assert_eq!(bob.email(), None);

    let serde = match manifest.dependencies().unwrap().by_name("serde").unwrap() {
        Dependency::Full(serde) => serde,
        _ => panic!(),
    };
    assert_eq!(serde.version(), "1.0");
    assert_eq!(serde.features(), Some(&["std", "derive"][..]));

    let regex = match manifest.dependencies().unwrap().by_name("regex").unwrap() {
        Dependency::VersionOnly(regex) => *regex,
        _ => panic!(),
    };
    assert_eq!(regex, "1.5");

    let cc = match manifest
        .targets()
        .unwrap()
        .by_name("cfg(unix)")
        .unwrap()
        .build_dependencies()
        .unwrap()
        .by_name("cc")
        .unwrap()
    {
        BuildDependency::VersionOnly(cc) => *cc,
        _ => panic!(),
    };
    assert_eq!(cc, "1.0.3");

    let default = manifest.features().unwrap().by_name("default").unwrap();
    assert_eq!(default, &["serde"]);

    let binary = &manifest.binaries().unwrap()[0];
    assert_eq!(binary.name(), "some-binary");
    assert_eq!(binary.path(), Some("src/bin/my-binary.rs"));
}

const CARGO_TOML: &str = r#"
[package]
name = "example"
version = "0.1.0"
edition = "2021"
authors = ["Alice Great <foo@bar.com>", "Bob Less"]
resolver = "2"

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

[target.'cfg(unix)'.build-dependencies]
cc = "1.0.3"

[features]
default = ["serde"]

[[bin]]
name = "some-binary"
path = "src/bin/my-binary.rs"

"#;
