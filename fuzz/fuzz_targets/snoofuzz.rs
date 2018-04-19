#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate snoomark as sm;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        sm::cm_to_rtjson(s.to_string());
    }
});
