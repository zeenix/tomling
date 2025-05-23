#[test]
fn zbus() {
    use tomling::{parse, Value};

    let parsed_map = parse(CARGO_TOML).unwrap();

    let package = parsed_map.get("package").unwrap().as_table().unwrap();
    assert_eq!(package.get("name").unwrap().as_str().unwrap(), "zbus");
    assert_eq!(package.get("version").unwrap().as_str().unwrap(), "5.1.1");
    assert!(package
        .get("edition")
        .unwrap()
        .as_table()
        .unwrap()
        .get("workspace")
        .unwrap()
        .as_bool()
        .unwrap());

    // Let's check the dependencies, especially the complicated ones.
    let dependencies = parsed_map.get("dependencies").unwrap().as_table().unwrap();

    // Serde
    let serde = dependencies.get("serde").unwrap().as_table().unwrap();
    assert!(serde.get("version").is_none());
    assert!(serde.get("workspace").unwrap().as_bool().unwrap());
    assert_eq!(
        serde.get("features").unwrap(),
        &["derive"].into_iter().collect::<Value>()
    );
    // Tokio
    let tokio = dependencies.get("tokio").unwrap().as_table().unwrap();
    assert_eq!(tokio.get("version").unwrap().as_str().unwrap(), "1.37.0");
    assert!(tokio.get("optional").unwrap().as_bool().unwrap());
    assert_eq!(
        tokio.get("features").unwrap(),
        &["rt", "net", "time", "fs", "io-util", "process", "sync", "tracing",]
            .into_iter()
            .collect::<Value>()
    );

    // cfg-using dependencies
    let target = parsed_map.get("target").unwrap().as_table().unwrap();
    // Nix
    let nix = target
        .get("cfg(unix)")
        .and_then(|v| {
            v.as_table()?
                .get("dependencies")?
                .as_table()?
                .get("nix")?
                .as_table()
        })
        .unwrap();
    assert_eq!(nix.get("version").unwrap().as_str().unwrap(), "0.29");
    assert!(!nix.get("default-features").unwrap().as_bool().unwrap());
    assert_eq!(
        nix.get("features").unwrap(),
        &["socket", "uio", "user"].into_iter().collect::<Value>()
    );
    // async-recursion
    let version = target
        .get("cfg(any(target_os = \"macos\", windows))")
        .and_then(|v| {
            v.as_table()?
                .get("dependencies")?
                .as_table()?
                .get("async-recursion")?
                .as_str()
        })
        .unwrap();
    assert_eq!(version, "1.1.1");

    // Now array of tables
    let bench = parsed_map
        .get("bench")
        .and_then(|v| v.as_array()?[0].as_table())
        .unwrap();
    assert_eq!(bench.get("name").unwrap().as_str().unwrap(), "benchmarks");
    assert!(!bench.get("harness").unwrap().as_bool().unwrap());

    // Finally, the examples
    let examples = parsed_map.get("example").unwrap().as_array().unwrap();
    let names = ["screen-brightness", "screen-brightness2"];
    let paths = [
        "examples/screen-brightness.rs",
        "examples/screen-brightness2.rs",
    ];
    for (i, example) in examples.iter().enumerate() {
        let example = example.as_table().unwrap();
        assert_eq!(example.get("name").unwrap().as_str().unwrap(), names[i]);
        assert_eq!(example.get("path").unwrap().as_str().unwrap(), paths[i]);
        assert_eq!(
            example.get("required-features").unwrap(),
            &["blocking-api"].into_iter().collect::<Value>()
        );
    }

    // Now the workspace Cargo.toml.
    let parsed_map = parse(WORKSPACE_CARGO_TOML).unwrap();
    let workspace = parsed_map.get("workspace").unwrap().as_table().unwrap();

    let package = workspace.get("package").unwrap().as_table().unwrap();
    assert!(package.get("version").is_none());
    assert_eq!(package.get("edition").unwrap().as_str().unwrap(), "2021");
}

#[cfg(feature = "cargo-toml")]
#[test]
fn zbus_serde() {
    use tomling::{
        cargo::{LibraryType, Manifest, ResolverVersion, RustEdition},
        Value,
    };

    let manifest: Manifest = tomling::from_str(CARGO_TOML).unwrap();

    let package = manifest.package().unwrap();
    assert_eq!(package.name(), "zbus");
    assert_eq!(package.version().unwrap(), "5.1.1".into());
    assert!(package.edition().unwrap().inherited());

    let serde = manifest.dependencies().unwrap().by_name("serde").unwrap();
    assert!(serde.version().is_none());
    assert_eq!(serde.workspace(), Some(true));
    assert_eq!(
        serde
            .features()
            .map(|f| f.map(|s| s).collect::<Vec<_>>())
            .as_deref(),
        Some(&["derive"][..]),
    );
    assert_eq!(serde.package(), Some("serde"));

    let tokio = manifest.dependencies().unwrap().by_name("tokio").unwrap();
    assert_eq!(tokio.version().unwrap(), "1.37.0");
    assert!(tokio.optional().unwrap());
    assert_eq!(
        tokio
            .features()
            .map(|f| f.map(|s| s).collect::<Vec<_>>())
            .as_deref(),
        Some(&["rt", "net", "time", "fs", "io-util", "process", "sync", "tracing"][..])
    );

    // The library section.
    let lib = manifest.library().unwrap();
    assert!(!lib.bench().unwrap());
    assert_eq!(
        lib.library_type().unwrap(),
        &[LibraryType::Cdylib, LibraryType::Rlib]
    );

    // The benchmarks.
    let bench = manifest.benches().unwrap().first().unwrap();
    assert_eq!(bench.name(), "benchmarks");
    assert!(!bench.harness().unwrap());

    // Now the workspace Cargo.toml.
    let manifest: Manifest = tomling::from_str(WORKSPACE_CARGO_TOML).unwrap();
    let workspace = manifest.workspace().unwrap();
    assert_eq!(
        workspace.members().unwrap().map(|s| s).collect::<Vec<_>>(),
        &[
            "zbus",
            "zvariant",
            "zbus_names",
            "zvariant_derive",
            "zvariant_utils",
            "zbus_macros",
            "zbus_xml",
            "zbus_xmlgen",
        ]
    );
    assert_eq!(workspace.resolver().unwrap(), ResolverVersion::V2);
    let unexpected_cfgs = match workspace.lints().unwrap().get("rust") {
        Some(Value::Table(rust)) => match rust.get("unexpected_cfgs") {
            Some(Value::Table(unexpected_cfgs)) => unexpected_cfgs,
            _ => panic!(),
        },
        _ => panic!(),
    };
    let package = workspace.package().unwrap();
    assert_eq!(package.edition().unwrap(), RustEdition::E2021);
    assert_eq!(
        unexpected_cfgs.get("level").unwrap().as_str().unwrap(),
        "warn"
    );
    assert_eq!(
        unexpected_cfgs.get("check-cfg").unwrap(),
        &["cfg(tokio_unstable)"].into_iter().collect::<Value>()
    );
    // TODO: Check the `profile` section after we add API for that.
}

const CARGO_TOML: &str = r#"
    [package]
    name = "zbus"
    version = "5.1.1"
    authors.workspace = true
    edition.workspace = true
    rust-version.workspace = true

    description.workspace = true
    repository.workspace = true
    keywords.workspace = true
    license.workspace = true
    categories.workspace = true
    readme.workspace = true

    [features]
    default = ["async-io", "blocking-api"]
    uuid = ["zvariant/uuid"]
    url = ["zvariant/url"]
    time = ["zvariant/time"]
    chrono = ["zvariant/chrono"]
    heapless = ["zvariant/heapless"]
    # Enables ser/de of `Option<T>` as an array of 0 or 1 elements.
    option-as-array = ["zvariant/option-as-array"]
    camino = ["zvariant/camino"]
    # Enables API that is only needed for bus implementations (enables `p2p`).
    bus-impl = ["p2p"]
    # Enables API that is only needed for peer-to-peer (p2p) connections.
    p2p = ["dep:rand"]
    async-io = [
        "dep:async-io",
        "async-executor",
        "async-task",
        "async-lock",
        "async-fs",
        # FIXME: We only currently only need this for unix but Cargo doesn't provide a way to enable
        # features for only specific target OS: https://github.com/rust-lang/cargo/issues/1197.
        "async-process",
        "blocking",
        "futures-util/io",
    ]
    tokio = ["dep:tokio"]
    vsock = ["dep:vsock", "dep:async-io"]
    tokio-vsock = ["dep:tokio-vsock", "tokio"]
    # Enable blocking API (default).
    blocking-api = ["zbus_macros/blocking-api"]
    # Enable `serde_bytes` feature of `zvariant`.
    serde_bytes = ["zvariant/serde_bytes"]

    [dependencies]
    zbus_macros = { path = "../zbus_macros", version = "=5.1.1" }
    zvariant = { path = "../zvariant", version = "5.0.0", default-features = false, features = [
        "enumflags2",
    ] }
    zbus_names = { path = "../zbus_names", version = "4.0" }
    serde = { workspace = true, features = ["derive"], package = "serde" }
    serde_repr = "0.1.19"
    enumflags2 = { version = "0.7.9", features = ["serde"] }
    futures-core = "0.3.30"
    futures-util = { version = "0.3.30", default-features = false, features = [
        "std",
    ] }
    async-broadcast = "0.7.0"
    hex = "0.4.3"
    ordered-stream = "0.2"
    rand = { version = "0.8.5", optional = true }
    event-listener = "5.3.0"
    static_assertions = "1.1.0"
    async-trait = "0.1.80"
    xdg-home = "1.1.0"
    tracing = "0.1.40"
    winnow = "0.6"

    # Optional and target-specific dependencies.

    async-io = { version = "2.3.2", optional = true }
    async-lock = { version = "3.3.0", optional = true }
    async-executor = { version = "1.11.0", optional = true }
    blocking = { version = "1.6.0", optional = true }
    async-task = { version = "4.7.1", optional = true }
    async-fs = { version = "2.1.2", optional = true }
    async-process = { version = "2.2.2", optional = true }
    tokio = { version = "1.37.0", optional = true, features = [
        "rt",
        "net",
        "time",
        "fs",
        "io-util",
        # FIXME: We should only enable this feature for unix. See comment above regarding `async-process`
        # on why we can't.
        "process",
        "sync",
        "tracing",
    ] }
    vsock = { version = "0.5.0", optional = true }
    tokio-vsock = { version = "0.6", optional = true }

    [target.'cfg(windows)'.dependencies]
    windows-sys = { version = "0.59", features = [
    "Win32_Foundation",
    "Win32_Security_Authorization",
    "Win32_System_Memory",
    "Win32_Networking",
    "Win32_Networking_WinSock",
    "Win32_NetworkManagement",
    "Win32_NetworkManagement_IpHelper",
    "Win32_System_IO",
    "Win32_System_Threading",
    ] }
    uds_windows = "1.1.0"

    [target.'cfg(unix)'.dependencies]
    nix = { version = "0.29", default-features = false, features = [
        "socket",
        "uio",
        "user",
    ] }

    [target.'cfg(any(target_os = "macos", windows))'.dependencies]
    async-recursion = "1.1.1"

    [dev-dependencies]
    zbus_xml = { path = "../zbus_xml", version = "5.0.0" }
    doc-comment = "0.3.3"
    futures-util = "0.3.30" # activate default features
    ntest = "0.9.2"
    test-log = { version = "0.2.16", features = [
    "trace",
    ], default-features = false }
    tokio = { version = "1.37.0", features = [
        "macros",
        "rt-multi-thread",
        "fs",
        "io-util",
        "net",
        "sync",
        "time",
        "test-util",
    ] }
    tracing-subscriber = { version = "0.3.18", features = [
        "env-filter",
        "fmt",
        "ansi",
    ], default-features = false }
    tempfile = "3.10.1"
    criterion = "0.5.1"

    [package.metadata.docs.rs]
    all-features = true
    targets = ["x86_64-unknown-linux-gnu"]

    [lints]
    workspace = true

    [lib]
    bench = false
    # Note: zbus' Cargo.toml doesn't have a `crate-type` specified.
    crate-type = ["cdylib", "rlib"]

    [[bench]]
    name = "benchmarks"
    harness = false

    [[example]]
    name = "screen-brightness"
    path = "examples/screen-brightness.rs"
    required-features = ["blocking-api"]

    # No such example in zbus' Cargo.toml but we want a case of > 1 entry in an array of tables.
    [[example]]
    name = "screen-brightness2"
    path = "examples/screen-brightness2.rs"
    required-features = ["blocking-api"]
"#;

const WORKSPACE_CARGO_TOML: &str = r#"
[workspace]
members = [
    "zbus",
    "zvariant",
    "zbus_names",
    "zvariant_derive",
    "zvariant_utils",
    "zbus_macros",
    "zbus_xml",
    "zbus_xmlgen",
]
resolver = "2"

[workspace.package]
authors = ["Zeeshan Ali Khan <zeeshanak@gnome.org>"]
edition = "2021"
rust-version = "1.80"

description = "API for D-Bus communication"
repository = "https://github.com/dbus2/zbus/"
keywords = ["D-Bus", "DBus", "IPC"]
license = "MIT"
categories = ["os::unix-apis"]
readme = "README.md"

[workspace.dependencies]
serde = { version = "1.0.200" }

[workspace.lints.rust]
# This optional cfg flag is used by tokio code to set task name
unexpected_cfgs = { level = "warn", check-cfg = ['cfg(tokio_unstable)'] }

[profile.bench]
debug = true
strip = "none"
"#;
