//! The `comrak` binary.

#![deny(missing_docs, missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unused_import_braces)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![allow(unknown_lints, doc_markdown, cyclomatic_complexity)]

// When compiled for the rustc compiler itself we want to make sure that this is
// an unstable crate.
#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]

#![cfg_attr(feature = "flamegraphs", feature(alloc_system))]
#![cfg_attr(feature = "flamegraphs", feature(plugin, custom_attribute))]
#![cfg_attr(feature = "flamegraphs", plugin(flamer))]

#[cfg(feature = "flamegraphs")]
extern crate flame;
#[cfg(feature = "flamegraphs")]
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
extern crate memchr;

mod arena_tree;
mod html;
mod rtjson;
mod parser;
mod nodes;
mod ctype;
mod scanners;
mod strings;
mod entity;

use std::process;


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

    process::exit(0);
}
