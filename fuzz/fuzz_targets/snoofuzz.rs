#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate snoomark as sm;
#[macro_use] extern crate cpython;

use cpython::*;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        sm::py::cm_to_rtjson(py, s.to_string()).expect("parsing failed");
    }
});
