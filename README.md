<p align="center">
  <a href="https://github.com/zeenix/tomling/actions/workflows/rust.yml">
    <img alt="Build Status" src="https://github.com/zeenix/tomling/actions/workflows/rust.yml/badge.svg">
  </a>
  <a href="https://docs.rs/tomling/">
    <img alt="API Documentation" src="https://docs.rs/tomling/badge.svg">
  </a>
  <a href="https://crates.io/crates/tomling">
    <img alt="crates.io" src="https://img.shields.io/crates/v/tomling">
  </a>
</p>

<p align="center">
  <img alt="Project logo" src="https://raw.githubusercontent.com/zeenix/tomling/fc40ab049d833cb79ee3ab9c441b0eebf05494ef/logo.svg">
</p>

<h1 align="center">tomling</h1>

`tomling` is a simple TOML parser API, that is designed to have minimal dependencies and is `no_std`
compatible. The main target is Cargo manifests (`Cargo.toml` files) and hence why specific
API is provided for that purpose as well.

## Usage

```rust
use tomling::{
    cargo::{BuildDependency, Dependency, Manifest, ResolverVersion, RustEdition},
    Value, parse,
};

//
// Using the `Cargo.toml` specific API:
//

let manifest: Manifest = tomling::from_str(CARGO_TOML).unwrap();

let package = manifest.package().unwrap();
assert_eq!(package.name(), "example");
assert_eq!(package.version(), &"0.1.0".into());
assert_eq!(package.edition().unwrap().uninherited().unwrap(), &RustEdition::E2021);
assert_eq!(package.resolver().unwrap(), ResolverVersion::V2);
let authors = package.authors().unwrap();
let authors = authors.uninherited().unwrap();
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

//
// Using the generic raw `TOML` parsing API:
//
let manifest = parse(CARGO_TOML).unwrap();
let package = manifest.get("package").unwrap().as_table().unwrap();
assert_eq!(package.get("name").unwrap().as_str().unwrap(), "example");
assert_eq!(package.get("version").unwrap().as_str().unwrap(), "0.1.0");
assert_eq!(package.get("edition").unwrap().as_str().unwrap(), "2021");
assert_eq!(package.get("resolver").unwrap().as_str().unwrap(), "2");

let deps = manifest.get("dependencies").unwrap().as_table().unwrap();
let serde = deps.get("serde").unwrap().as_table().unwrap();
assert_eq!(serde.get("version").unwrap().as_str().unwrap(), "1.0");
let serde_features =
    serde.get("features").unwrap().as_array().unwrap().as_slice();
assert_eq!(serde_features, &[Value::from("std"), "derive".into()]);
let regex = deps.get("regex").unwrap().as_str().unwrap();
assert_eq!(regex, "1.5");

const CARGO_TOML: &'static str = r#"
[package]
name = "example"
version = "0.1.0"
edition = "2021"
authors = ["Alice Great <foo@bar.com>", "Bob Less"]
resolver = "2"

[dependencies]
serde = { version = "1.0", features = [
    "std",
    "derive", # and here.
] }
regex = "1.5"

[target.'cfg(unix)'.build-dependencies]
cc = "1.0.3"

[features]
default = ["serde"]

[[bin]]
name = "some-binary"
path = "src/bin/my-binary.rs"

"#;
```

## Dependencies

- `winnow` with `alloc` and `simd` features enabled.
- `serde` (optional) with `alloc` and `derive` features enabled.

## Features

- `serde` - Enables Serde support.
- `cargo-toml` - Enables Cargo manifest specific API. This requires `serde`.
- `simd` - Enables the `simd` feature of `winnow` for SIMD acceleration for parsing.
- `std` - Enables some features, like `std::error::Error` implementation for `Error` type. It also
  enables `std` feature of `winnow` and `serde`.

All features are enabled by default.

## Comparison with `toml` crate

The [`toml`] crate is great but it being based on `toml_edit`, it ends up requiring `indexmap` crate
and its dependencies. `tomling` was created specifically to avoid most of these dependencies by
focusing completely on the parsing of `TOML` documents only.

Having said that, some of the code (especially the low-level parsing code) is inspired (or in some
cases, copied) from the `toml_edit` crate.

## Goals

- Simple parser/deserializer API.
- Minimum dependencies. The only mandatory dependency is `winnow` with only 2 features enabled.
- Primary target: Cargo manifests.

## Non-goals

- Encoder/Serializer API.

## License

[MIT](LICENSE-MIT)

## The Name

The name "tomling" is a portmanteau of "TOML" and "ling" (a suffix meaning "a small thing").
Coincidentally, it also means a "male kitten" in English, with all the stress on the "kitten"
part 😸 and none on the "male" part.

[`toml`]: https://crates.io/crates/toml
