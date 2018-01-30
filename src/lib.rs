//! A 100% [CommonMark](http://commonmark.org/) and [GFM](https://github.github.com/gfm/)
//! compatible Markdown parser.  Source repository is at <https://github.com/kivikakk/comrak>.
//!
//! The design is based on [cmark](https://github.com/github/cmark), so familiarity with that will
//! help.
//!
//! You can use `comrak::markdown_to_html` directly:
//!
//! ```
//! use comrak::{markdown_to_html, ComrakOptions};
//! assert_eq!(markdown_to_html("Hello, **世界**!", &ComrakOptions::default()),
//!            "<p>Hello, <strong>世界</strong>!</p>\n");
//! ```
//!
//! Or you can parse the input into an AST yourself, manipulate it, and then use your desired
//! formatter:
//!
//! ```
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
        trivial_numeric_casts, unstable_features, unused_import_braces)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![allow(unknown_lints, doc_markdown, cyclomatic_complexity)]

#![cfg_attr(rustbuild, feature(staged_api, rustc_private))]
#![cfg_attr(rustbuild, unstable(feature = "rustc_private", issue = "27812"))]

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
#[macro_use]
extern crate serde_json;

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
#[macro_use] extern crate cpython;

use cpython::*;
use serde_json::Value;

// add bindings to the generated python module
// This initializes the Python module and assigns the name `snoomark`,
// which converts Reddit-flavored CommonMark (or legacy Markdown) to RTJSON.
py_module_initializer!(snoomark, initsnoomark, PyInit_snoomark, |py, m| {
    // add bindings to the generated python module
    // This initializes the Python module and assigns the name `snoomark`,
    // which converts Reddit-flavored CommonMark (or legacy Markdown) to RTJSON.
    const DOC_NAME: &'static str = env!("CARGO_PKG_NAME");
    const DOC_VERSION: &'static str = env!("CARGO_PKG_VERSION");
    let doc_string = format!("[{} {}] This module is implemented in Rust.", DOC_NAME, DOC_VERSION);
    try!(m.add(py, "__doc__", doc_string));
    try!(m.add(py, "cm_to_rtjson", py_fn!(py, cm_to_rtjson_py(cm: String))));
    Ok(())
});

// rust-cpython aware function. All of our python interface could be
// declared in a separate module.
// Note that the py_fn!() macro automatically converts the arguments from
// Python objects to Rust values; and the Rust return value back into a Python object.
fn cm_to_rtjson(cm: String) -> Value {
    let arena = Arena::new();

    let options = ComrakOptions {
        rtjson: true,
        hardbreaks: false,
        github_pre_lang: false,
        width: 0,
        ext_strikethrough: true,
        ext_tagfilter: false,
        ext_table: true,
        ext_autolink: false,
        ext_tasklist: false,
        ext_superscript: false,
        ext_footnotes: false,
        ext_header_ids: None
    };

    let root = parse_document(&arena, &cm, &options);
    let rendered_rtjson = rtjson::format_document(root, &ComrakOptions::default());
    rendered_rtjson
}

/// Convert from a `serde_json::Value` to a `cpython::PyObject`.
/// Code originally inspired from library by Iliana Weller found at
/// https://github.com/ilianaw/rust-cpython-json/blob/master/src/lib.rs
pub fn from_json(py: Python, json: Value) -> PyObject {
    macro_rules! obj {
        ($x:ident) => {
            $x.into_py_object(py).into_object()
        }
    }

    match json {
        Value::Number(x) => {
            if let Some(n) = x.as_u64() {
                obj!(n)
            } else if let Some(n) = x.as_i64() {
                obj!(n)
            } else if let Some(n) = x.as_f64() {
                obj!(n)
            } else {
                // We should never get to this point
                unreachable!()
            }
        }
        Value::String(x) => PyUnicode::new(py, &x).into_object(),
        Value::Bool(x) => obj!(x),
        Value::Array(vec) => {
            let mut elements = Vec::new();
            for item in vec {
                elements.push(from_json(py, item));
            }
            PyList::new(py, &elements[..]).into_object()
        }
        Value::Object(map) => {
            let dict = PyDict::new(py);
            for (key, value) in map {
                dict.set_item(py, key, from_json(py, value));
            }
            dict.into_object()
        }
        Value::Null => py.None(),
    }
}

// logic implemented as a normal rust function
fn cm_to_rtjson_py(py: Python, cm: String) -> PyResult<PyObject> {
    let out = cm_to_rtjson(cm);
    let res = from_json(py, out);
    Ok(res)
}
