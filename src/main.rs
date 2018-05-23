//! The `comrak` binary.

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unused_import_braces)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![allow(unknown_lints, doc_markdown, cyclomatic_complexity)]

// When compiled for the rustc compiler itself we want to make sure that this is
// an unstable crate.
#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]

#![cfg_attr(any(feature = "flamegraphs", feature = "minflame"), feature(alloc_system))]
#![cfg_attr(any(feature = "flamegraphs", feature = "minflame"), feature(plugin, custom_attribute))]
#![cfg_attr(any(feature = "flamegraphs", feature = "minflame"), plugin(flamer))]

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
extern crate flame;
#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
extern crate alloc_system;

extern crate entities;
#[macro_use]
extern crate clap;
extern crate unicode_categories;
extern crate typed_arena;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate twoway;
#[macro_use]
extern crate serde_json;

mod arena_tree;
mod html;
mod rtjson;
mod parser;
mod nodes;
mod ctype;
mod scanners;
mod strings;
mod entity;

use std::io::{Read,Write};
use std::process;
use std::path::Path;
use std::fs::File;
use typed_arena::Arena;

fn render_html(text: &str, opts: &parser::ComrakOptions) -> String {
    let arena = Arena::new();
    let root = parser::parse_document(&arena, text, opts);
    let mut rendered_html = vec![];
    html::format_document(root, &parser::ComrakOptions::default(), &mut rendered_html).ok();
    String::from_utf8(rendered_html).unwrap()
}

#[cfg_attr(feature = "flamegraphs", flame)]
fn render_rtjson(text: &str, opts: &parser::ComrakOptions) -> String {
    let arena = Arena::new();
    let root = parser::parse_document(&arena, text, opts);
    let rendered_rtjson = rtjson::format_document(root);
    rendered_rtjson.0.to_string()
}

// Tests in the spec (v0.26) are of the form:
//
// ```````````````````````````````` example
// <markdown input>
// .
// <expected output>
// ````````````````````````````````
#[derive(Debug)]
struct Spec<'a> {
    spec: &'a str,
    test_n: usize,
}

impl<'a> Spec<'a> {
    pub fn new(spec: &'a str) -> Self {
        Spec{ spec:spec, test_n: 0 }
    }
}

struct TestCase <'a> {
    n: usize,
    input: &'a str,
    expected: &'a str,
}

impl<'a> TestCase<'a> {
    pub fn new(n: usize, input: &'a str, expected: &'a str) -> Self {
        TestCase{n: n, input: input, expected: expected }
    }
}

impl<'a> Iterator for Spec<'a> {
    type Item = TestCase<'a>;

    fn next(&mut self) -> Option<TestCase<'a>> {
        let spec = self.spec;

        let i_start = match self.spec.find("```````````````````````````````` example\n").map(|pos| pos + 41) {
            Some(pos) => pos,
            None => return None,
        };

        let i_end = match self.spec[i_start..].find("\n.\n").map(|pos| (pos + 1) + i_start ) {
            Some(pos) => pos,
            None => return None,
        };

        let e_end = match self.spec[i_end + 2..].find("````````````````````````````````\n").map(|pos| pos + i_end + 2){
            Some(pos) => pos,
            None => return None,
        };

        self.test_n += 1;
        self.spec = &self.spec[e_end + 33 ..];

        Some(TestCase::new(self.test_n, &spec[i_start .. i_end], &spec[i_end + 2 .. e_end]))
    }
}

fn spec_test (args: &Vec<&str>, opts: parser::ComrakOptions) -> Result<(), ()> {
    let mut spec_text = String::new();
    for fs in args {
        spec_text.push_str(&read_file(&fs).replace("→","\t"));
    }

    let (first, last) = if args.is_empty( ) {
        (None, None)
    } else {
        let mut iter = args[0].split("..");
        let first = iter.next().and_then(|s| s.parse().ok());
        let last = match iter.next() {
            Some(s) => s.parse().ok(),
            None => first
        };
        (first, last)
    };

    let formatter = if opts.rtjson {
        render_rtjson
    } else {
        render_html
    };

    let spec = Spec::new(&spec_text[..]);
    let mut tests_failed = 0;
    let mut tests_run = 0;
    let mut fail_report = String::new();

    for test in spec {
        if first.map(|fst| test.n < fst).unwrap_or(false) { continue }
        if last.map(|lst| test.n > lst).unwrap_or(false) { break }

        if test.n % 10 == 1 {
            if test.n % 40 == 1 {
                if test.n > 1 {
                    println!("");
                }
            } else {
                print!(" ");
            }
            print!("[{:3}]", test.n );
        } else if test.n % 10 == 6 {
            print!(" ");
        }
        let rtjson = opts.rtjson.clone();

        let our_rendering = formatter(&test.input, &opts);
        let mut value = serde_json::Value::Null;
        let mut compare = serde_json::Value::Null;
        if rtjson {
            value = match serde_json::from_str(&our_rendering) {
                 Ok(s) => s,
                 Err(e) => {
                     println!("error parsing: {:?}", e);
                     serde_json::Value::Null
                 }
            };
            compare = match serde_json::from_str(&test.expected) {
                 Ok(s) => s,
                 Err(e) => {
                    println!("error parsing: {:?}", e);
                     serde_json::Value::Null
                 }
            };
        }

        if rtjson && value == compare {
            print!(".");
        } else if our_rendering == test.expected {
            print!(".");
        } else {
            if !rtjson {
                fail_report += format!("\nFAIL {}:\n\n---input---\n{}\n\n---wanted---\n{}\n\n---got---\n{}\n",
                                       test.n, test.input, test.expected, our_rendering).as_str();
            } else {
                let expected = compare.to_string();
                let actual = value.to_string();
                fail_report += format!("\nFAIL {}:\n\n---input---\n{}\n\n---wanted---\n{}\n\n---got---\n{}\n",
                                       test.n, test.input, expected, actual).as_str();
            }
            print!("X");
            tests_failed += 1;
        }

        let _ = std::io::stdout().flush();
        tests_run += 1;
    }

    println!("\n{}/{}", tests_run - tests_failed, tests_run );
    print!("{}", fail_report);
    println!("\n {} test succeeded out of {} test run.", tests_run - tests_failed, tests_run );

    if tests_failed == 0 {
        Ok(())
    } else {
        Err(())
    }
}

fn read_file(filename: &str) -> String {
    let path = Path::new(filename);
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file
    };
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(_) => s
    }
}

fn main() {
    let matches = clap::App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            clap::Arg::with_name("file")
                .value_name("FILE")
                .multiple(true)
                .help(
                    "The CommonMark file to parse; or standard input if none passed",
                ),
        )
        .arg(clap::Arg::with_name("rtjson").long("rtjson").help(
            "Parse AST into an RTJSON compliant format",
        ))
        .arg(clap::Arg::with_name("hardbreaks").long("hardbreaks").help(
            "Treat newlines as hard line breaks",
        ))
        .arg(
            clap::Arg::with_name("github-pre-lang")
                .long("github-pre-lang")
                .help("Use GitHub-style <pre lang> for code blocks"),
        )
        .arg(
            clap::Arg::with_name("extension")
                .short("e")
                .long("extension")
                .takes_value(true)
                .number_of_values(1)
                .multiple(true)
                .possible_values(
                    &[
                        "strikethrough",
                        "tagfilter",
                        "table",
                        "autolink",
                        "tasklist",
                        "superscript",
                    ],
                )
                .value_name("EXTENSION")
                .help("Specify an extension name to use"),
        )
        .arg(
            clap::Arg::with_name("format")
                .short("t")
                .long("to")
                .takes_value(true)
                .possible_values(&["html", "commonmark"])
                .default_value("html")
                .value_name("FORMAT")
                .help("Specify output format"),
        )
        .arg(
            clap::Arg::with_name("width")
                .long("width")
                .takes_value(true)
                .value_name("WIDTH")
                .default_value("0")
                .help("Specify wrap width (0 = nowrap)"),
        )
        .arg(
            clap::Arg::with_name("spec")
                .long("spec")
                .takes_value(true)
                .multiple(true)
                .value_name("SPEC")
                .help("Run test from spec file"),
        )
        .get_matches();

    let options = parser::ComrakOptions {
        rtjson: false || matches.is_present("rtjson"),
        hardbreaks: false || matches.is_present("hardbreaks"),
        github_pre_lang: false || matches.is_present("github=pre-lang"),
        width: matches.value_of("width").unwrap_or("0").parse().unwrap_or(
            0,
        ),
        ext_strikethrough: true,
        ext_tagfilter: false,
        ext_table: true,
        ext_autolink: true,
        ext_tasklist: false,
        ext_superscript: false,
        ext_footnotes: false,
        ext_header_ids: None,
        ext_spoilertext: true,
        ext_reddit_quirks: true,
    };

    if matches.is_present("spec") {
        let r = spec_test(&matches.values_of("spec").unwrap().collect::<Vec<_>>(), options);
        flame_dump();
        if r.is_err() {
            process::exit(1);
        }
    }

    process::exit(0);
}

#[cfg(feature = "flamegraphs")]
fn flame_dump() {
    flame::dump_html(&mut File::create("flamegraph.html").unwrap()).unwrap()
}

#[cfg(not(feature = "flamegraphs"))]
fn flame_dump() { }
