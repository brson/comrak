#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate comrak;

use comrak::ComrakOptions;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let opts = ComrakOptions::default();
        comrak::markdown_to_html(s, &opts);
    }
});
