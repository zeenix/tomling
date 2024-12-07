#[test]
fn tokio() {
    use tomling::parse;

    let parsed_map = parse(CARGO_TOML).unwrap();

    // Too much to check for everything. Let's check some keys and values.
    let package = parsed_map.get("package").unwrap().as_table().unwrap();
    assert_eq!(package.get("name").unwrap().as_str().unwrap(), "tokio");
    assert_eq!(package.get("version").unwrap().as_str().unwrap(), "1.41.1");
    assert_eq!(package.get("edition").unwrap().as_str().unwrap(), "2021");

    // Let's check the dependencies, especially the complicated ones.
    let dependencies = parsed_map.get("dependencies").unwrap().as_table().unwrap();

    // bytes
    let bytes = dependencies.get("bytes").unwrap().as_table().unwrap();
    assert_eq!(bytes.get("version").unwrap().as_str().unwrap(), "1.0.0");
    assert_eq!(bytes.get("optional").unwrap().as_bool().unwrap(), true);

    let dev_deps = parsed_map
        .get("dev-dependencies")
        .unwrap()
        .as_table()
        .unwrap();
    let tokio_test = dev_deps.get("tokio-test").unwrap().as_table().unwrap();
    assert_eq!(
        tokio_test.get("version").unwrap().as_str().unwrap(),
        "0.4.0"
    );
    assert_eq!(
        tokio_test.get("path").unwrap().as_str().unwrap(),
        "../tokio-test"
    );

    // cfg-using dependencies
    let target = parsed_map.get("target").unwrap().as_table().unwrap();
    // wasm-bindgen-test
    let version = target
        .get("cfg(all(target_family = \"wasm\", not(target_os = \"wasi\")))")
        .and_then(|c| {
            c.as_table()?
                .get("dev-dependencies")?
                .as_table()?
                .get("wasm-bindgen-test")?
                .as_str()
        })
        .unwrap();
    assert_eq!(version, "0.3.0");

    // rand
    let version = target
        .get("cfg(not(all(target_family = \"wasm\", target_os = \"unknown\")))")
        .and_then(|c| {
            c.as_table()?
                .get("dev-dependencies")?
                .as_table()?
                .get("rand")?
                .as_str()
        })
        .unwrap();
    assert_eq!(version, "0.8.0");
}

#[cfg(feature = "cargo-toml")]
#[test]
fn tokio_serde() {
    use tomling::cargo::{Dependency, DevDependency, Manifest, RustEdition};

    let manifest: Manifest = tomling::from_str(CARGO_TOML).unwrap();
    assert_eq!(manifest.package().name(), "tokio");
    assert_eq!(manifest.package().version(), "1.41.1");
    assert_eq!(manifest.package().edition().unwrap(), RustEdition::E2021);

    let bytes = match manifest.dependencies().unwrap().by_name("bytes").unwrap() {
        Dependency::Full(bytes) => bytes,
        _ => panic!(),
    };
    assert_eq!(bytes.version(), "1.0.0");
    assert_eq!(bytes.optional(), Some(true));

    let socket2 = match manifest
        .targets()
        .unwrap()
        .by_name("cfg(not(target_family = \"wasm\"))")
        .unwrap()
        .dependencies()
        .unwrap()
        .by_name("socket2")
        .unwrap()
    {
        Dependency::Full(s) => s,
        _ => panic!(),
    };
    assert_eq!(socket2.version(), "0.5.5");
    assert_eq!(socket2.optional(), Some(true));
    assert_eq!(socket2.features(), Some(&["all"][..]));

    let tokio_test = manifest
        .dev_dependencies()
        .unwrap()
        .by_name("tokio-test")
        .unwrap();
    let tokio_test = match tokio_test {
        DevDependency::Full(t) => t,
        _ => panic!(),
    };
    assert_eq!(tokio_test.version(), "0.4.0");
    assert_eq!(tokio_test.features(), None);

    let windows_sys = manifest
        .targets()
        .unwrap()
        .by_name("cfg(windows)")
        .unwrap()
        .dev_dependencies()
        .unwrap()
        .by_name("windows-sys")
        .unwrap();
    let windows_sys = match windows_sys {
        DevDependency::Full(w) => w,
        _ => panic!(),
    };
    assert_eq!(windows_sys.version(), "0.52");
    assert_eq!(
        windows_sys.features(),
        Some(&["Win32_Foundation", "Win32_Security_Authorization"][..])
    );
}

const CARGO_TOML: &str = r#"
[package]
name = "tokio"
# When releasing to crates.io:
# - Remove path dependencies
# - Update doc url
#   - README.md
# - Update CHANGELOG.md.
# - Create "v1.x.y" git tag.
version = "1.41.1"
edition = "2021"
rust-version = "1.70"
authors = ["Tokio Contributors <team@tokio.rs>"]
license = "MIT"
readme = "README.md"
repository = "https://github.com/tokio-rs/tokio"
homepage = "https://tokio.rs"
description = """
An event-driven, non-blocking I/O platform for writing asynchronous I/O
backed applications.
"""
categories = ["asynchronous", "network-programming"]
keywords = ["io", "async", "non-blocking", "futures"]

[features]
# Include nothing by default
default = []

# enable everything
full = [
  "fs",
  "io-util",
  "io-std",
  "macros",
  "net",
  "parking_lot",
  "process",
  "rt",
  "rt-multi-thread",
  "signal",
  "sync",
  "time",
]

fs = []
io-util = ["bytes"]
# stdin, stdout, stderr
io-std = []
macros = ["tokio-macros"]
net = [
  "libc",
  "mio/os-poll",
  "mio/os-ext",
  "mio/net",
  "socket2",
  "windows-sys/Win32_Foundation",
  "windows-sys/Win32_Security",
  "windows-sys/Win32_Storage_FileSystem",
  "windows-sys/Win32_System_Pipes",
  "windows-sys/Win32_System_SystemServices",
]
process = [
  "bytes",
  "libc",
  "mio/os-poll",
  "mio/os-ext",
  "mio/net",
  "signal-hook-registry",
  "windows-sys/Win32_Foundation",
  "windows-sys/Win32_System_Threading",
  "windows-sys/Win32_System_WindowsProgramming",
]
# Includes basic task execution capabilities
rt = []
rt-multi-thread = ["rt"]
signal = [
  "libc",
  "mio/os-poll",
  "mio/net",
  "mio/os-ext",
  "signal-hook-registry",
  "windows-sys/Win32_Foundation",
  "windows-sys/Win32_System_Console",
]
sync = []
test-util = ["rt", "sync", "time"]
time = []

[dependencies]
tokio-macros = { version = "~2.4.0", path = "../tokio-macros", optional = true }

pin-project-lite = "0.2.11"

# Everything else is optional...
bytes = { version = "1.0.0", optional = true }
mio = { version = "1.0.1", optional = true, default-features = false }
parking_lot = { version = "0.12.0", optional = true }

[target.'cfg(not(target_family = "wasm"))'.dependencies]
socket2 = { version = "0.5.5", optional = true, features = ["all"] }

# Currently unstable. The API exposed by these features may be broken at any time.
# Requires `--cfg tokio_unstable` to enable.
[target.'cfg(tokio_unstable)'.dependencies]
tracing = { version = "0.1.29", default-features = false, features = ["std"], optional = true } # Not in full

# Currently unstable. The API exposed by these features may be broken at any time.
# Requires `--cfg tokio_unstable` to enable.
[target.'cfg(tokio_taskdump)'.dependencies]
backtrace = { version = "0.3.58" }

[target.'cfg(unix)'.dependencies]
libc = { version = "0.2.149", optional = true }
signal-hook-registry = { version = "1.1.1", optional = true }

[target.'cfg(unix)'.dev-dependencies]
libc = { version = "0.2.149" }
nix = { version = "0.29.0", default-features = false, features = ["aio", "fs", "socket"] }

[target.'cfg(windows)'.dependencies.windows-sys]
version = "0.52"
optional = true

[target.'cfg(windows)'.dev-dependencies.windows-sys]
version = "0.52"
features = [
  "Win32_Foundation",
  "Win32_Security_Authorization",
]

[dev-dependencies]
tokio-test = { version = "0.4.0", path = "../tokio-test" }
tokio-stream = { version = "0.1", path = "../tokio-stream" }
futures = { version = "0.3.0", features = ["async-await"] }
mockall = "0.11.1"
async-stream = "0.3"

[target.'cfg(not(target_family = "wasm"))'.dev-dependencies]
socket2 = "0.5.5"
tempfile = "3.1.0"
proptest = "1"

[target.'cfg(not(all(target_family = "wasm", target_os = "unknown")))'.dev-dependencies]
rand = "0.8.0"

[target.'cfg(all(target_family = "wasm", not(target_os = "wasi")))'.dev-dependencies]
wasm-bindgen-test = "0.3.0"

[target.'cfg(target_os = "freebsd")'.dev-dependencies]
mio-aio = { version = "0.9.0", features = ["tokio"] }

[target.'cfg(loom)'.dev-dependencies]
loom = { version = "0.7", features = ["futures", "checkpoint"] }

[package.metadata.docs.rs]
all-features = true
# enable unstable features in the documentation
rustdoc-args = ["--cfg", "docsrs", "--cfg", "tokio_unstable", "--cfg", "tokio_taskdump"]
# it's necessary to _also_ pass `--cfg tokio_unstable` and `--cfg tokio_taskdump`
# to rustc, or else dependencies will not be enabled, and the docs build will fail.
rustc-args = ["--cfg", "tokio_unstable", "--cfg", "tokio_taskdump"]

[package.metadata.playground]
features = ["full", "test-util"]

[package.metadata.cargo_check_external_types]
# The following are types that are allowed to be exposed in Tokio's public API.
# The standard library is allowed by default.
allowed_external_types = [
  "bytes::buf::buf_impl::Buf",
  "bytes::buf::buf_mut::BufMut",
  "tokio_macros::*",
]
"#;
