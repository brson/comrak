//! A 100% [CommonMark](http://commonmark.org/) and [GFM](https://github.github.com/gfm/)
//! compatible Markdown parser.  Source repository is at <https://github.com/kivikakk/comrak>.
//!
//! The design is based on [cmark](https://github.com/github/cmark), so familiarity with that will
//! help.
//!
//! You can use `comrak::markdown_to_html` directly:
//!
//! ```ignore
//! use comrak::{markdown_to_html, ComrakOptions};
//! assert_eq!(markdown_to_html("Hello, **世界**!", &ComrakOptions::default()),
//!            "<p>Hello, <strong>世界</strong>!</p>\n");
//! ```
//!
//! Or you can parse the input into an AST yourself, manipulate it, and then use your desired
//! formatter:
//!
//! ```ignore
//! extern crate comrak;
//! extern crate typed_arena;
//! use typed_arena::Arena;
//! use comrak::{parse_document, format_html, ComrakOptions};
//! use comrak::nodes::{AstNode, NodeValue};
//!
//! # fn main() {
//! // The returned nodes are created in the supplied Arena, and are bound by its lifetime.
//! let arena = Arena::new();
//!
//! let root = parse_document(
//!     &arena,
//!     "This is my input.\n\n1. Also my input.\n2. Certainly my input.\n",
//!     &ComrakOptions::default());
//!
//! fn iter_nodes<'a, F>(node: &'a AstNode<'a>, f: &F)
//!     where F : Fn(&'a AstNode<'a>) {
//!     f(node);
//!     for c in node.children() {
//!         iter_nodes(c, f);
//!     }
//! }
//!
//! iter_nodes(root, &|node| {
//!     match &mut node.data.borrow_mut().value {
//!         &mut NodeValue::Text(ref mut text) => {
//!             let orig = std::mem::replace(text, vec![]);
//!             *text = String::from_utf8(orig).unwrap().replace("my", "your").as_bytes().to_vec();
//!         }
//!         _ => (),
//!     }
//! });
//!
//! let mut html = vec![];
//! format_html(root, &ComrakOptions::default(), &mut html).unwrap();
//!
//! assert_eq!(
//!     String::from_utf8(html).unwrap(),
//!     "<p>This is your input.</p>\n\
//!      <ol>\n\
//!      <li>Also your input.</li>\n\
//!      <li>Certainly your input.</li>\n\
//!      </ol>\n");
//! # }
//! ```

#![deny(missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unused_import_braces)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![allow(unknown_lints, doc_markdown, cyclomatic_complexity)]

#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]

#![cfg_attr(any(feature = "flamegraphs", feature = "minflame"), feature(alloc_system))]
#![cfg_attr(any(feature = "flamegraphs", feature = "minflame"), feature(plugin, custom_attribute))]
#![cfg_attr(any(feature = "flamegraphs", feature = "minflame"), plugin(flamer))]

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
extern crate flame;
#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
extern crate alloc_system;

extern crate unicode_categories;
extern crate typed_arena;
extern crate regex;
extern crate entities;
#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;
extern crate twoway;
extern crate memchr;

mod arena_tree;
mod parser;
mod scanners;
mod html;
mod rtjson;
mod ctype;
mod nodes;
mod entity;
mod strings;

pub use parser::{parse_document, ComrakOptions};
use typed_arena::Arena;

extern crate libc;
#[cfg(feature = "cpython")]
#[macro_use] extern crate cpython;

#[cfg(feature = "cpython")]
use cpython::*;

// add bindings to the generated python module
// This initializes the Python module and assigns the name `snoomark`,
// which converts Reddit-flavored CommonMark (or legacy Markdown) to RTJSON.
#[cfg(feature = "cpython")]
py_module_initializer!(snoomark, initsnoomark, PyInit_snoomark, |py, m| {
    // add bindings to the generated python module
    // This initializes the Python module and assigns the name `snoomark`,
    // which converts Reddit-flavored CommonMark (or legacy Markdown) to RTJSON.
    const DOC_NAME: &'static str = env!("CARGO_PKG_NAME");
    const DOC_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let doc_string = format!("[{} {}] This module is implemented in Rust.", DOC_NAME, DOC_VERSION);
    try!(m.add(py, "__doc__", doc_string));
    try!(m.add(py, "cm_to_rtjson", py_fn!(py, cm_to_rtjson_py(cm: String))));
    add_flame_fns(py, m)?;
    Ok(())
});

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn add_flame_fns(py: Python, m: &PyModule) -> PyResult<()> {
    try!(m.add(py, "flame_exec_start", py_fn!(py, flame_exec_start())));
    try!(m.add(py, "flame_exec_end", py_fn!(py, flame_exec_end())));
    try!(m.add(py, "flame_convert_start", py_fn!(py, flame_convert_start())));
    try!(m.add(py, "flame_convert_end", py_fn!(py, flame_convert_end())));
    try!(m.add(py, "flame_dumps_start", py_fn!(py, flame_dumps_start())));
    try!(m.add(py, "flame_dumps_end", py_fn!(py, flame_dumps_end())));
    try!(m.add(py, "flame_del_start", py_fn!(py, flame_del_start())));
    try!(m.add(py, "flame_del_end", py_fn!(py, flame_del_end())));
    try!(m.add(py, "flame_write", py_fn!(py, flame_write())));
    try!(m.add(py, "flame_clear", py_fn!(py, flame_clear())));
    Ok(())
}

#[cfg(not(any(feature = "flamegraphs", feature = "minflame")))]
fn add_flame_fns(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_exec_start(py: Python) -> PyResult<PyObject> {
    flame::start("exec");
    Ok(py.None())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_exec_end(py: Python) -> PyResult<PyObject> {
    flame::end("exec");
    Ok(py.None())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_convert_start(py: Python) -> PyResult<PyObject> {
    flame::start("convert");
    Ok(py.None())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_convert_end(py: Python) -> PyResult<PyObject> {
    flame::end("convert");
    Ok(py.None())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_dumps_start(py: Python) -> PyResult<PyObject> {
    flame::start("dumps");
    Ok(py.None())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_dumps_end(py: Python) -> PyResult<PyObject> {
    flame::end("dumps");
    Ok(py.None())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_del_start(py: Python) -> PyResult<PyObject> {
    flame::start("del");
    Ok(py.None())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_del_end(py: Python) -> PyResult<PyObject> {
    flame::end("del");
    Ok(py.None())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_write(py: Python) -> PyResult<PyObject> {
    flame::dump_html(&mut ::std::fs::File::create("flamegraph.html").unwrap()).unwrap();
    Ok(py.None())
}

#[cfg(any(feature = "flamegraphs", feature = "minflame"))]
fn flame_clear(py: Python) -> PyResult<PyObject> {
    flame::clear();
    Ok(py.None())
}

// rust-cpython aware function. All of our python interface could be
// declared in a separate module.
// Note that the py_fn!() macro automatically converts the arguments from
// Python objects to Rust values; and the Rust return value back into a Python object.
#[cfg(feature = "cpython")]
#[cfg_attr(any(feature = "flamegraphs", feature = "minflame"), flame)]
pub fn cm_to_rtjson(py: Python, cm: String) -> PyResult<PyObject> {
    let arena = Arena::new();

    let options = ComrakOptions {
        rtjson: true,
        hardbreaks: false,
        github_pre_lang: false,
        width: 0,
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

    let root = parse_document(&arena, &cm, &options);
    rtjson::format_document(py, root)
}

// logic implemented as a normal rust function
#[cfg_attr(any(feature = "flamegraphs", feature = "minflame"), flame)]
#[cfg(feature = "cpython")]
fn cm_to_rtjson_py(py: Python, cm: String) -> PyResult<PyObject> {
    if let Some(obj) = quick_render(py, &cm) {
        return Ok(obj);
    }
    cm_to_rtjson(py, cm)
}

// An optimization for simple documents that use minimal markup, rendering
// directly from the source with minimal allocation and copying. It does one
// initial scan, very fast, looking for bytes that indicate markup syntax, and
// bailing to the slowpath. If it takes the fast path it does 3 more complete
// scans of the text, and up to two copies of the text. Besides the necessary
// allocations of Python objects, it only does 1 allocation, and that is cached
// thread-local.
//
// Note that in the unlikely case this function encounters an error constructing
// python objects it just returns `None`, forcing the caller onto the full
// rendering path - the errors are not propagated.
#[cfg_attr(any(feature = "flamegraphs", feature = "minflame"), flame)]
fn quick_render(py: Python, cm: &str) -> Option<PyObject> {
    use std::iter;
    use ctype::isspace;
    use std::cell::RefCell;

    // Quickly decide whether the document requires full markdown processing, by
    // scanning for characters that may indicate markdown syntax.

    // Larger documents are less likely to be candidates
    const MAX_DOC_SIZE: usize = 640;

    // Conditions for rejecting the document. Closures for lazy execution,
    // should be inlined.

    let too_big = || cm.len() > MAX_DOC_SIZE;
    let empty = || cm.is_empty();
    // Scan 1 - Fast. This is the scan that will catch most of the cases the
    // fast-path can't handle, and it's very fast, so we'll usually bail
    // quickly.
    let has_syntax = || strings::contains_forbidden_chars(cm);
    // Scan 2 - www autolinking. This one's a bummer.
    // This scan could possibly be incorporated into the line-splitting below
    // using memchr2, but not sure it's worth it - it's pretty fast as-is. Using
    // the twoway crate, but fastest with assembly, which we don't turn on.
    // TODO: reimplement the twoway fastpath with simd intrinsics to accelerate this.
    let has_www = || twoway::find_str(cm, "www.").is_some();

    if too_big() || empty() || has_syntax() || has_www() {
        return None;
    }

    // This document can (probably) be processed quickly. Process every line
    // building up paragraphs, and handling special cases.

    let doc_contents = PyList::new(py, &[]);

    // A stored slice of the first line of the paragraph, to avoid copying
    // it into `para_accum` in the case where the paragraph is one line,
    // and to track whether we're parsing the first line of a paragraph
    // or subsequent lines.
    let mut first_line = None;
    // The accumulated contents of the current paragaph. This is a thread
    // local so it can be reused. This could also be a stack-allocated array
    // of bytes, but that code is much uglier.
    thread_local! {
        static PARA_ACCUM: RefCell<String> = RefCell::new(String::new());
    }

    PARA_ACCUM.with(|p| {
        let mut para_accum = p.borrow_mut();
        para_accum.clear();

        // Iterate over lines collecting paragraphs. The chained iterator here adds
        // a blank line to force the last paragraph to close. This is using
        // a custom line-splitting iterator that is faster than the one in std.
        // Scan 3
        for mut line in strings::fast_lines(cm).chain(iter::repeat("").take(1)) {

            // If this is a blank line then output a paragraph of the accumulated text
            if line.is_empty() || line.bytes().all(|b| isspace(b)) {
                let pypara;
                match (first_line, !para_accum.is_empty()) {
                    (None, false) => {
                        // This is a blank line following other blank lines.
                        // It's nothing.
                        continue
                    }
                    (Some(first_line), _) => {
                        // Scan 4 - internally PyString will run is_ascii over
                        // the string to decide whether to create a
                        // bytestring or a unicode string.
                        // TODO: accelerate is_ascii with simd
                        // Copy 1 (directly from the source)
                        pypara = PyString::new(py, first_line);
                    }
                    (None, true) => {
                        // Scan 4
                        // Copy 2 (from the paragraph accumulation buffer)
                        pypara = PyString::new(py, &para_accum);
                    }
                }
                para_accum.clear();
                first_line = None;

                let text = PyDict::new(py);
                text.set_item(py, "e", PyBytes::new(py, b"text")).ok()?;
                text.set_item(py, "t", pypara).ok()?;

                let para_contents = PyList::new(py, &[text.into_object()]).into_object();

                let para = PyDict::new(py);
                para.set_item(py, "e", PyBytes::new(py, b"par")).ok()?;
                para.set_item(py, "c", para_contents).ok()?;

                doc_contents.insert_item(py, doc_contents.len(py), para.into_object());

                continue
            }

            // It wasn't a blank line. We have to first check for
            // a few cases that we can't handle - failures of speculative
            // optimization, then store the line for later paragraph
            // rendering.

            let hardbreak = if line.len() > 2 {
                debug_assert!(!line.bytes().all(|b| isspace(b)));
                line.bytes().rev().take(2).all(|b| isspace(b))
            } else {
                false
            };

            if hardbreak {
                // Speculation failure
                return None;
            }

            let (leading_space, leading_space_bytes) = {
                let mut leading_space = 0;
                let mut leading_space_bytes = 0;
                for ch in line.bytes() {
                    if ch == b' ' {
                        leading_space += 1;
                        leading_space_bytes += 1;
                    } else if ch == b'\t' {
                        // Adding four here probably isn't technically correct -
                        // it should probably round to the nearest tab stop -
                        // but for our purposes all that matters is whether the
                        // leading space is greater or equal to 4.
                        leading_space += 4;
                        leading_space_bytes += 1;
                    } else {
                        break;
                    }
                }
                (leading_space, leading_space_bytes)
            };

            let trailing_space_bytes = {
                line.bytes().rev().take_while(|ch| isspace(*ch)).count()
            };

            // Trim line
            let line = &line[leading_space_bytes..(line.len() - trailing_space_bytes)];

            match (first_line, !para_accum.is_empty()) {
                (None, false) => {
                    // This is the first line of a new paragraph

                    if unsupported_block(line, leading_space, true) {
                        // Speculation failure
                        return None;
                    }

                    first_line = Some(line);
                }
                (Some(ref fl), false) => {
                    // This is the second line of a paragraph

                    if unsupported_block(line, leading_space, false) {
                        // Speculation failure
                        return None;
                    }

                    // Allocation 1
                    para_accum.reserve(MAX_DOC_SIZE);
                    // Copy 1
                    para_accum.push_str(fl);
                    para_accum.push_str(" ");
                    para_accum.push_str(line);

                    first_line = None;
                }
                (None, true) => {
                    // Remaining lines of a paragrah

                    if unsupported_block(line, leading_space, false) {
                        // Speculation failure
                        return None;
                    }

                    // Copy 1
                    para_accum.push_str(" ");
                    para_accum.push_str(line);

                    first_line = None;
                }
                (Some(..), true) => unreachable!()
            }
        }

        debug_assert!(para_accum.is_empty());
        debug_assert!(first_line.is_none());

        let doc = PyDict::new(py);
        doc.set_item(py, "document", doc_contents.into_object()).ok()?;

        Some(doc.into_object())
    })
}

// Some less-common block types that aren't detected by the original
// syntax scan will cause us to bail: code blocks and ordered lists.
#[inline]
fn unsupported_block(line: &str, leading_space: usize, opening_line: bool) -> bool {
    use ctype::isspace;

    if opening_line && leading_space >= 4 {
        // Code block
        return true;
    }
    if !opening_line && leading_space >= 4 {
        // Not an ordered list
        return false;
    }

    let line = line.as_bytes();

    // The rest of this is scanning for ordered list syntax.
    // As a reddit quirk, ordered lists must start with "1",
    // which makes the testing here simple. Just look for
    // "1. ", "1.\t", "1) ", "1)\t".
    if line.len() < 2 {
        // Not enough bytes to be an ordered list
        return false;
    }

    if line[0] != b'1' {
        return false;
    }
    if line[1] != b'.' && line[1] != b')' {
        return false;
    }

    if line.len() > 2 && !isspace(line[2]) {
        return false;
    }

    return true;
}
