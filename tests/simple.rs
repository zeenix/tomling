#[test]
fn simple_cargo_toml() {
    use tomling::{parse, Table, Value};

    let mut map = Table::new();
    map.insert(
        "package".into(),
        [
            ("name", "example".into()),
            (
                "version",
                [("workspace", Value::from(true))]
                    .into_iter()
                    .collect::<Value>(),
            ),
            ("edition", "2021".into()),
            ("resolver", "2".into()),
            (
                "authors",
                ["Alice Great <foo@bar.com>", "Bob Less"]
                    .into_iter()
                    .collect::<Value>(),
            ),
        ]
        .into_iter()
        .collect(),
    );
    map.insert(
        "dependencies".into(),
        [
            (
                "serde",
                [
                    ("version", "1.0".into()),
                    ("features", ["std", "derive"].into_iter().collect::<Value>()),
                ]
                .into_iter()
                .collect::<Value>(),
            ),
            ("regex", "1.5".into()),
        ]
        .into_iter()
        .collect(),
    );
    map.insert(
        "target".into(),
        [(
            "cfg(unix)",
            [(
                "build-dependencies",
                [("cc", "1.0.3")].into_iter().collect::<Value>(),
            )]
            .into_iter()
            .collect::<Value>(),
        )]
        .into_iter()
        .collect(),
    );
    map.insert(
        "features".into(),
        [("default", ["serde"].into_iter().collect::<Value>())]
            .into_iter()
            .collect(),
    );
    map.insert(
        "bin".into(),
        [[
            ("name", Value::from("some-binary")),
            ("path", "src/bin/my-binary.rs".into()),
        ]
        .into_iter()
        .collect::<Value>()]
        .into_iter()
        .collect(),
    );

    let parsed_map = parse(CARGO_TOML).unwrap();
    assert_eq!(parsed_map, map);
}

#[cfg(feature = "cargo-toml")]
#[test]
fn simple_cargo_toml_serde() {
    use tomling::cargo::{Manifest, ResolverVersion, RustEdition};

    let manifest: Manifest = tomling::from_str(CARGO_TOML).unwrap();

    let package = manifest.package().unwrap();
    assert_eq!(package.name(), "example");
    assert!(package.version().inherited());
    assert_eq!(
        package.edition().unwrap().uninherited().unwrap(),
        &RustEdition::E2021,
    );
    assert_eq!(package.resolver().unwrap(), ResolverVersion::V2);
    let authors = package.authors().unwrap();
    let authors = authors.uninherited().unwrap();
    let alice = &authors[0];
    assert_eq!(alice.name(), "Alice Great");
    assert_eq!(alice.email(), Some("foo@bar.com"));
    let bob = &authors[1];
    assert_eq!(bob.name(), "Bob Less");
    assert_eq!(bob.email(), None);

    let serde = manifest.dependencies().unwrap().by_name("serde").unwrap();
    assert_eq!(serde.version().unwrap(), "1.0");
    assert_eq!(serde.features(), Some(&["std", "derive"][..]));

    let regex = manifest.dependencies().unwrap().by_name("regex").unwrap();
    assert_eq!(regex.version().unwrap(), "1.5");

    let cc = manifest
        .targets()
        .unwrap()
        .by_name("cfg(unix)")
        .unwrap()
        .build_dependencies()
        .unwrap()
        .by_name("cc")
        .unwrap();
    assert_eq!(cc.version().unwrap(), "1.0.3");

    let default = manifest.features().unwrap().by_name("default").unwrap();
    assert_eq!(default, &["serde"]);

    let binary = &manifest.binaries().unwrap()[0];
    assert_eq!(binary.name(), "some-binary");
    assert_eq!(binary.path(), Some("src/bin/my-binary.rs"));
}

const CARGO_TOML: &str = r#"
[package]
name = "example"
version.workspace = true
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
