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

#![cfg_attr(feature = "flamegraphs", feature(alloc_system))]
#![cfg_attr(feature = "flamegraphs", feature(plugin, custom_attribute))]
#![cfg_attr(feature = "flamegraphs", plugin(flamer))]

#[cfg(feature = "flamegraphs")]
extern crate flame;
#[cfg(feature = "flamegraphs")]
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
use rtjson::Json;

extern crate libc;
#[cfg(feature = "cpython")]
#[macro_use] extern crate cpython;

#[cfg(feature = "cpython")]
use cpython::*;
use serde_json::Value;

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

#[cfg(feature = "flamegraphs")]
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

#[cfg(not(feature = "flamegraphs"))]
fn add_flame_fns(_py: Python, _m: &PyModule) -> PyResult<()> {
    Ok(())
}

#[cfg(feature = "flamegraphs")]
fn flame_exec_start(py: Python) -> PyResult<PyObject> {
    flame::start("exec");
    Ok(py.None())
}

#[cfg(feature = "flamegraphs")]
fn flame_exec_end(py: Python) -> PyResult<PyObject> {
    flame::end("exec");
    Ok(py.None())
}

#[cfg(feature = "flamegraphs")]
fn flame_convert_start(py: Python) -> PyResult<PyObject> {
    flame::start("convert");
    Ok(py.None())
}

#[cfg(feature = "flamegraphs")]
fn flame_convert_end(py: Python) -> PyResult<PyObject> {
    flame::end("convert");
    Ok(py.None())
}

#[cfg(feature = "flamegraphs")]
fn flame_dumps_start(py: Python) -> PyResult<PyObject> {
    flame::start("dumps");
    Ok(py.None())
}

#[cfg(feature = "flamegraphs")]
fn flame_dumps_end(py: Python) -> PyResult<PyObject> {
    flame::end("dumps");
    Ok(py.None())
}

#[cfg(feature = "flamegraphs")]
fn flame_del_start(py: Python) -> PyResult<PyObject> {
    flame::start("del");
    Ok(py.None())
}

#[cfg(feature = "flamegraphs")]
fn flame_del_end(py: Python) -> PyResult<PyObject> {
    flame::end("del");
    Ok(py.None())
}

#[cfg(feature = "flamegraphs")]
fn flame_write(py: Python) -> PyResult<PyObject> {
    flame::dump_html(&mut ::std::fs::File::create("flamegraph.html").unwrap()).unwrap();
    Ok(py.None())
}

#[cfg(feature = "flamegraphs")]
fn flame_clear(py: Python) -> PyResult<PyObject> {
    flame::clear();
    Ok(py.None())
}

// rust-cpython aware function. All of our python interface could be
// declared in a separate module.
// Note that the py_fn!() macro automatically converts the arguments from
// Python objects to Rust values; and the Rust return value back into a Python object.
#[cfg_attr(feature = "flamegraphs", flame)]
pub fn cm_to_rtjson(cm: String) -> Json {
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
    let rendered_rtjson = rtjson::format_document(root);
    rendered_rtjson
}

/// Convert from a `serde_json::Value` to a `cpython::PyObject`.
/// Code originally inspired from library by Iliana Weller found at
/// https://github.com/ilianaw/rust-cpython-json/blob/master/src/lib.rs
#[cfg_attr(feature = "flamegraphs", flame)]
#[cfg(feature = "cpython")]
pub fn from_json(py: Python, json: Json) -> PyObject {
    macro_rules! obj {
        ($x:ident) => {
            $x.into_py_object(py).into_object()
        }
    }

    // Iterative traversal similar to `format` in rtjson.rs. Pre-traversal
    // enqueues children for processing; post-traversal pops children and
    // converts the node.

    enum Phase { Pre, Post }
    enum Parent<'a> { Array, Map(&'a str) }

    let mut stack = vec![(&json.0, Phase::Pre, Parent::Array)];
    let mut vec_accum = vec![vec![]];
    let mut map_accum = vec![];

    while let Some((json, phase, parent)) = stack.pop() {
        match phase {
            Phase::Pre => {
                stack.push((json, Phase::Post, parent));
                match *json {
                    Value::Array(ref vec) => {
                        vec_accum.push(vec![]);
                        for item in vec.iter().rev() {
                            stack.push((item, Phase::Pre, Parent::Array));
                        }
                    }
                    Value::Object(ref map) => {
                        map_accum.push(vec![]);
                        for (key, value) in map.iter().rev() {
                            stack.push((value, Phase::Pre, Parent::Map(key)))
                        }
                    }
                    _ => ()
                }
            }
            Phase::Post => {
                let pyval = match *json {
                    Value::Number(ref x) => {
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
                    Value::String(ref x) => PyUnicode::new(py, &x).into_object(),
                    Value::Bool(x) => obj!(x),
                    Value::Array(..) => {
                        let elements = vec_accum.pop().expect("py vec accumulator");
                        PyList::new(py, &elements[..]).into_object()
                    }
                    Value::Object(..) => {
                        let elements = map_accum.pop().expect("py map accumulator");
                        let dict = PyDict::new(py);
                        for (key, value) in elements {
                            dict.set_item(py, key, value);
                        }
                        dict.into_object()
                    }
                    Value::Null => py.None(),
                };

                match parent {
                    Parent::Array => {
                        vec_accum.last_mut().expect("py vec accumulator").push(pyval);
                    }
                    Parent::Map(key) => {
                        map_accum.last_mut().expect("py map accumulator").push((key, pyval));
                    }
                }
            }
        }
    }

    assert!(map_accum.is_empty());
    let mut last_accum = vec_accum.pop().expect("last accumulator");
    assert!(vec_accum.is_empty());
    let pyval = last_accum.pop().expect("last json");
    assert!(last_accum.is_empty());
    pyval
}

// logic implemented as a normal rust function
#[cfg_attr(feature = "flamegraphs", flame)]
#[cfg(feature = "cpython")]
fn cm_to_rtjson_py(py: Python, cm: String) -> PyResult<PyObject> {
    let out = cm_to_rtjson(cm);
    let res = from_json(py, out);
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::cm_to_rtjson;

    #[test]
    fn no_stack_smash() {
        // Don't smash the stack on this deeply-nested blockquote
        let big: String = ::std::iter::repeat('>').take(150_000).collect();
        cm_to_rtjson(big);
    }
}

