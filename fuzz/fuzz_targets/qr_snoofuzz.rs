#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate snoomark as sm;
#[macro_use] extern crate cpython;

use cpython::*;
use std::process::Command;
use std::env;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = std::str::from_utf8(data) {
        let gil = Python::acquire_gil();
        let py = gil.python();
        let path = env::current_dir().unwrap();
        path.push("fuzz");
        path.push("script");
        path.push("snoofuzz.py");
        Command::new("python").args(&[path.to_str(), s]).spawn().expect("python command failed");
    }
});