#[test]
fn simple_cargo_toml() {
    let cargo_toml = r#"
        [package]
        name = "example"
        version = "0.1.0"
        edition = "2021"

        [dependencies]
        serde = { version = "1.0", features = ["derive"] }
        regex = "1.5"
        
        [features]
        default = ["serde"]
    "#;

    match tomly::parse(cargo_toml) {
        Ok(map) => {
            for (key, value) in map {
                println!("{key}: {value:?}");
            }
        }
        Err(err) => eprintln!("Failed to parse TOML: {err:?}"),
    }
}
