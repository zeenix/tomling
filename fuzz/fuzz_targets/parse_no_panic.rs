#![no_main]

use libfuzzer_sys::fuzz_target;
use tomling::parse;

fuzz_target!(|input: &str| {
    let _ = parse(input);
});
