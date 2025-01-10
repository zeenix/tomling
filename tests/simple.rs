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
            (
                "dep-from-git",
                [
                    ("git", "https://github.com/zeenix/dep-from-git"),
                    ("branch", "main"),
                ]
                .into_iter()
                .collect::<Value>(),
            ),
            (
                "dep-from-path",
                [("path", "../dep-from-path")]
                    .into_iter()
                    .collect::<Value>(),
            ),
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
    assert!(package.version().unwrap().inherited());
    assert_eq!(
        package.edition().unwrap().uninherited_ref().unwrap(),
        &RustEdition::E2021,
    );
    assert_eq!(package.resolver().unwrap(), ResolverVersion::V2);
    let authors = package.authors().unwrap();
    let mut authors = authors.uninherited().unwrap();
    let alice = authors.next().unwrap();
    assert_eq!(alice.name(), "Alice Great");
    assert_eq!(alice.email(), Some("foo@bar.com"));
    let bob = authors.next().unwrap();
    assert_eq!(bob.name(), "Bob Less");
    assert_eq!(bob.email(), None);

    let serde = manifest.dependencies().unwrap().by_name("serde").unwrap();
    assert_eq!(serde.version().unwrap(), "1.0");
    assert_eq!(
        serde
            .features()
            .map(|f| f.map(|s| s).collect::<Vec<_>>())
            .as_deref(),
        Some(&["std", "derive"][..])
    );

    let regex = manifest.dependencies().unwrap().by_name("regex").unwrap();
    assert_eq!(regex.version().unwrap(), "1.5");
    let dep_from_git = manifest
        .dependencies()
        .unwrap()
        .by_name("dep-from-git")
        .unwrap();
    let git = dep_from_git.source().unwrap().git().unwrap();
    assert_eq!(git.repository(), "https://github.com/zeenix/dep-from-git");
    let commit = git.commit().unwrap();
    assert_eq!(commit.branch().unwrap(), "main");
    assert!(commit.revision().is_none());
    assert!(commit.tag().is_none());

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

#[test]
fn test_escape_sequences_basic_strings() {
    use tomling::parse;

    let toml_str = r#"
        str1 = "This is a backslash: \\"
        str2 = "This is a double quote: \""
        str3 = "This is a backspace: \b"
        str4 = "This is a tab: \t"
        str5 = "This is a newline: \n"
        str6 = "This is a form feed: \f"
        str7 = "This is a carriage return: \r"
        str8 = "This is a unicode character: \u0041"
        str9 = "This is another unicode character: \U00000041"
    "#;

    let parsed_map = parse(toml_str).unwrap();
    assert_eq!(
        parsed_map.get("str1").unwrap().as_str().unwrap(),
        "This is a backslash: \\"
    );
    assert_eq!(
        parsed_map.get("str2").unwrap().as_str().unwrap(),
        "This is a double quote: \""
    );
    assert_eq!(
        parsed_map.get("str3").unwrap().as_str().unwrap(),
        "This is a backspace: \x08"
    );
    assert_eq!(
        parsed_map.get("str4").unwrap().as_str().unwrap(),
        "This is a tab: \t"
    );
    assert_eq!(
        parsed_map.get("str5").unwrap().as_str().unwrap(),
        "This is a newline: \n"
    );
    assert_eq!(
        parsed_map.get("str6").unwrap().as_str().unwrap(),
        "This is a form feed: \x0C"
    );
    assert_eq!(
        parsed_map.get("str7").unwrap().as_str().unwrap(),
        "This is a carriage return: \r"
    );
    assert_eq!(
        parsed_map.get("str8").unwrap().as_str().unwrap(),
        "This is a unicode character: A"
    );
    assert_eq!(
        parsed_map.get("str9").unwrap().as_str().unwrap(),
        "This is another unicode character: A"
    );
}

#[test]
fn test_escape_sequences_multiline_basic_strings() {
    use tomling::parse;

    let toml_str = r#"
        str1 = """This is a backslash: \\"""
        str2 = """This is a double quote: \"""""
        str3 = """This is a backspace: \b"""
        str4 = """This is a tab: \t"""
        str5 = """This is a newline: \n"""
        str6 = """This is a form feed: \f"""
        str7 = """This is a carriage return: \r"""
        str8 = """This is a unicode character: \u0041"""
        str9 = """This is another unicode character: \U00000041"""
    "#;

    let parsed_map = parse(toml_str).unwrap();
    assert_eq!(
        parsed_map.get("str1").unwrap().as_str().unwrap(),
        "This is a backslash: \\"
    );
    assert_eq!(
        parsed_map.get("str2").unwrap().as_str().unwrap(),
        "This is a double quote: \""
    );
    assert_eq!(
        parsed_map.get("str3").unwrap().as_str().unwrap(),
        "This is a backspace: \x08"
    );
    assert_eq!(
        parsed_map.get("str4").unwrap().as_str().unwrap(),
        "This is a tab: \t"
    );
    assert_eq!(
        parsed_map.get("str5").unwrap().as_str().unwrap(),
        "This is a newline: \n"
    );
    assert_eq!(
        parsed_map.get("str6").unwrap().as_str().unwrap(),
        "This is a form feed: \x0C"
    );
    assert_eq!(
        parsed_map.get("str7").unwrap().as_str().unwrap(),
        "This is a carriage return: \r"
    );
    assert_eq!(
        parsed_map.get("str8").unwrap().as_str().unwrap(),
        "This is a unicode character: A"
    );
    assert_eq!(
        parsed_map.get("str9").unwrap().as_str().unwrap(),
        "This is another unicode character: A"
    );
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
dep-from-git = { git = "https://github.com/zeenix/dep-from-git", branch = "main" }
dep-from-path = { path = "../dep-from-path" }

[target.'cfg(unix)'.build-dependencies]
cc = "1.0.3"

[features]
default = ["serde"]

[[bin]]
name = "some-binary"
path = "src/bin/my-binary.rs"

"#;
