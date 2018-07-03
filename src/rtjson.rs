#![cfg(feature = "cpython")]

extern crate cpython;
use nodes::{TableAlignment, NodeValue, ListType, AstNode};
use self::cpython::*;
use std::str;
use borrow_unchecked::*;

/// Formats an AST as RTJSON
#[cfg_attr(any(feature = "flamegraphs", feature = "minflame"), flame)]
pub fn format_document<'a>(py: Python, root: &'a AstNode<'a>) -> PyResult<PyObject> {
    let f = RTJsonFormatter;
    f.format(py, root)
}

struct RTJsonFormatter;

impl RTJsonFormatter {
    fn format<'a>(&self, py: Python, root_node: &'a AstNode<'a>) -> PyResult<PyObject> {
        // This is another iterative traversal of the AST, with
        // pre-child-traversal, and post-child-traversal phases.
        //
        // For every node this is collecting the child node json into
        // the `accum` vec-of-vecs, moving that json to the node's own
        // json, then pushing its json onto `accum`.
        //
        // More specifically, during pre-order traversal, we push a new child
        // accumulator onto `accum` for the children to add their contents to;
        // during post-order-traversal, we pop the vec of children, and push the
        // node's own content.

        enum Phase { Pre, Post }

        let mut stack = vec![(root_node, Phase::Pre)];
        let mut accum = vec![vec![]];

        while let Some((node, phase)) = stack.pop() {
            match phase {
                Phase::Pre => {
                    // Add a new accumulator for the children of this node
                    accum.push(vec![]);

                    // Push the node back on to the stack to accumulate the
                    // results of traversing the children.
                    stack.push((node, Phase::Post));

                    // Push the children onto the stack, in reverse order, so
                    // they are processed in order.
                    stack.extend(node.reverse_children().map(|cn| (cn, Phase::Pre)));
                }
                Phase::Post => {
                    // Then format the node, without the child content in place
                    let mut json = self.format_node(py, node)?;

                    let children = accum.pop().expect("py map accumulator");
                    let content =  PyList::new(py, &children[..]);

                    // Then add the child content to the node
                    match node.data.borrow_().value {
                        NodeValue::Document => {
                            json.set_item(py, "document", content).unwrap();
                        }
                        NodeValue::Table(..) => {
                            let mut vals = vec![];
                            let mut content = content;
                            for val in content.iter(py) {
                                let map = match val.cast_into::<PyDict>(py) {
                                    Ok(m) => m,
                                    Err(_) => unreachable!(),
                                };
                                if let Some(c) = map.get_item(py, "c") {
                                    vals.push(c);
                                } else if let Some(h) = map.get_item(py, "h") {
                                    json.set_item(py, "h", h).unwrap();
                                } else {
                                    unreachable!();
                                };
                            }
                            json.set_item(py, "c", &vals[..]).unwrap();
                        }
                        NodeValue::TableRow(..) => {
                            match json.get_item(py, "h") {
                                Some(_h) => json.set_item(py, "h", content)?,
                                None => json.set_item(py, "c", content)?,
                            }
                        }
                        _ => {
                            if content.len(py) != 0 {
                                json.set_item(py, "c", content).unwrap();
                            }
                        }
                    }

                    // Finally, push this rendered node onto it's parent's list of
                    // accumulated children.
                    let last = accum.last_mut().expect("child accum");
                    last.push(json.into_object());
                }
            }
        }

        let mut last_accum = accum.pop().expect("last accumulator");
        assert!(accum.is_empty());
        let json_l = last_accum.pop().expect("last json");
        assert!(last_accum.is_empty());
        Ok(json_l)
    }

    fn format_node<'a>(&self, py: Python, node: &'a AstNode<'a>) -> PyResult<PyDict> {
        macro_rules! obj {
            ($x:ident) => {
                $x.into_py_object(py).into_object()
            }
        }

        match node.data.borrow_().value {
            NodeValue::Document => {
                let dict = PyDict::new(py);
                dict.set_item(py, "document", "")?;
                Ok(dict)
            },
            NodeValue::BlockQuote => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "blockquote")?;
                Ok(dict)
            }
            NodeValue::List(ref nl) => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "list")?;
                dict.set_item(py, "o", if nl.list_type != ListType::Bullet {true} else {false})?;
                Ok(dict)
            }
            NodeValue::Item(..) => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "li")?;
                Ok(dict)
            }
            NodeValue::Heading(ref nch) => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "h")?;
                dict.set_item(py, "l", nch.level)?;

                let raw = PyDict::new(py);
                raw.set_item(py, "e", "raw")?;
                raw.set_item(py, "t", "")?;

                let raw_list = PyList::new(py, &[raw.into_object()]);
                dict.set_item(py, "c", raw_list)?;
                Ok(dict)
            }
            NodeValue::CodeBlock(ref ncb) => {
                let mut int = PyList::new(py, &[]);
                let code_content = unsafe { str::from_utf8_unchecked(&ncb.literal) };
                let max = code_content.split("\n").count() - 1;
                for (i, it) in code_content.split("\n").enumerate() {
                    if i != max {
                        let dice = PyDict::new(py);
                        dice.set_item(py, "e", "raw")?;
                        dice.set_item(py, "t", it)?;
                        int.insert_item(py, i, dice.into_object());
                    }
                }
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "code")?;
                dict.set_item(py, "c", int)?;
                Ok(dict)
            }
            NodeValue::ThematicBreak => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "hr")?;
                Ok(dict)
            },
            NodeValue::LineBreak => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "br")?;
                Ok(dict)
            },
            NodeValue::HtmlBlock(..) => unreachable!(),
            NodeValue:: SoftBreak => Ok(PyDict::new(py)),
            NodeValue::Code(_) | NodeValue::Strong | NodeValue::Emph | NodeValue::Superscript |
            NodeValue::Strikethrough => unreachable!(),
            NodeValue::Paragraph => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "par")?;
                Ok(dict)
            }
            NodeValue::Text(ref literal) => {
                match node.parent().unwrap().data.borrow_().value {
                    NodeValue::Heading(..) | NodeValue::CodeBlock(..) => {
                        let dict = PyDict::new(py);
                        dict.set_item(py, "e", "raw")?;
                        dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&literal) })?;
                        Ok(dict)
                   }
                   NodeValue::TableCell | NodeValue::Paragraph | NodeValue::BlockQuote
                   | NodeValue::SpoilerText => {
                        let dict = PyDict::new(py);
                        dict.set_item(py, "e", "text")?;
                        dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&literal) })?;
                        Ok(dict)
                   }
                   _ => unreachable!(),
                }
            }
            NodeValue::FormattedText(ref literal, ref format_ranges) => {
                match node.parent().unwrap().data.borrow_().value {
                    NodeValue::Heading(..) | NodeValue::CodeBlock(..) => {
                        let dict = PyDict::new(py);
                        dict.set_item(py, "e", "raw")?;
                        dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&literal) })?;
                        Ok(dict)
                   }
                   NodeValue::TableCell | NodeValue::Paragraph | NodeValue::BlockQuote
                   | NodeValue::SpoilerText => {
                        let dict = PyDict::new(py);
                        let mut elements = PyList::new(py, &[]);
                        for val in format_ranges.iter() {
                            elements.insert_item(py, elements.len(py), obj!(val));
                        }

                        dict.set_item(py, "e", "text")?;
                        dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&literal) })?;
                        dict.set_item(py, "f", elements)?;
                        Ok(dict)
                   }
                   _ => unreachable!(),
                }
            }
            NodeValue::FormattedLink(ref nl) => {
                if !&nl.element.is_empty() {
                    let dict = PyDict::new(py);
                    let mut elements = PyList::new(py, &[]);
                    for val in nl.format_range.iter() {
                        elements.insert_item(py, elements.len(py), obj!(val));
                    }

                    dict.set_item(py, "e", "link")?;
                    dict.set_item(py, "u", unsafe { str::from_utf8_unchecked(&nl.url) })?;
                    dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&nl.caption) })?;
                    dict.set_item(py, "f", elements)?;
                    dict.set_item(py, "a", unsafe { str::from_utf8_unchecked(&nl.element) })?;
                    Ok(dict)
                } else {
                    let dict = PyDict::new(py);
                    let mut elements = PyList::new(py, &[]);
                    for val in nl.format_range.iter() {
                        elements.insert_item(py, elements.len(py), obj!(val));
                    }

                    dict.set_item(py, "e", "link")?;
                    dict.set_item(py, "u", unsafe { str::from_utf8_unchecked(&nl.url) })?;
                    dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&nl.caption) })?;
                    dict.set_item(py, "f", elements)?;
                    Ok(dict)
                }
            }
            NodeValue::UnformattedLink(ref nl) => {
                if !&nl.element.is_empty() {
                    let dict = PyDict::new(py);
                    dict.set_item(py, "e", "link")?;
                    dict.set_item(py, "u", unsafe { str::from_utf8_unchecked(&nl.url) })?;
                    dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&nl.caption) })?;
                    dict.set_item(py, "a", unsafe { str::from_utf8_unchecked(&nl.element) })?;
                    Ok(dict)
                } else {
                    let dict = PyDict::new(py);
                    dict.set_item(py, "e", "link")?;
                    dict.set_item(py, "u", unsafe { str::from_utf8_unchecked(&nl.url) })?;
                    dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&nl.caption) })?;
                    Ok(dict)
                }
            }
            NodeValue::Link(ref nl) => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "link")?;
                dict.set_item(py, "u", unsafe { str::from_utf8_unchecked(&nl.url) })?;
                dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&nl.title)})?;
                Ok(dict)
            }
            NodeValue::RedditLink(ref nl) => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", unsafe { str::from_utf8_unchecked(&nl.prefix) })?;
                dict.set_item(py, "t", unsafe { str::from_utf8_unchecked(&nl.name) })?;
                dict.set_item(py, "l", nl.l)?;
                Ok(dict)
            }
            NodeValue::Media(ref nl) => {
                if !&nl.title.is_empty() {
                    let dict = PyDict::new(py);
                    dict.set_item(py, "e",  unsafe { str::from_utf8_unchecked(&nl.e) })?;
                    dict.set_item(py, "id", unsafe { str::from_utf8_unchecked(&nl.url) })?;
                    dict.set_item(py, "c",  unsafe { str::from_utf8_unchecked(&nl.title) })?;
                    Ok(dict)
                } else {
                    let dict = PyDict::new(py);
                    dict.set_item(py, "e", unsafe { str::from_utf8_unchecked(&nl.e) })?;
                    dict.set_item(py, "id", unsafe { str::from_utf8_unchecked(&nl.url) })?;
                    Ok(dict)
                }
            }
            NodeValue::Table(..) => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "table")?;
                Ok(dict)
            }
            NodeValue::TableRow(header) => {
                if header {
                    let dict = PyDict::new(py);
                    dict.set_item(py, "h", PyList::new(py, &[]))?;
                    Ok(dict)
                } else {
                    let dict = PyDict::new(py);
                    dict.set_item(py, "c", PyList::new(py, &[]))?;
                    Ok(dict)
                }
            }
            NodeValue::TableCell => {
                let row = &node.parent().unwrap().data.borrow_().value;
                let in_header = match *row {
                    NodeValue::TableRow(header) => header,
                    _ => panic!(),
                };

                let table = &node.parent()
                                 .unwrap()
                                 .parent()
                                 .unwrap()
                                 .data
                                 .borrow_()
                                 .value;
                let alignments = match *table {
                    NodeValue::Table(ref alignments) => alignments,
                    _ => panic!(),
                };

                let mut i = 0;
                let mut start = node.parent().unwrap().first_child().unwrap();
                while !start.same_node(node) {
                    i += 1;
                    start = start.next_sibling().unwrap();
                }

                if in_header {
                    let dict = PyDict::new(py);
                    dict.set_item(
                        py,
                        "a",
                        match alignments[i] {
                            TableAlignment::Left => "L",
                            TableAlignment::Right => "R",
                            TableAlignment::Center => "C",
                            TableAlignment::None => "",
                        }
                    )?;
                    Ok(dict)
                } else {
                    let dict = PyDict::new(py);
                    dict.set_item(py, "c", PyList::new(py, &[]))?;
                    Ok(dict)
                }
            }
            NodeValue::FootnoteDefinition(..) => unreachable!(),
            NodeValue::HtmlInline(..) => unreachable!(),
            NodeValue::FootnoteReference(..) => unreachable!(),
            NodeValue::SpoilerText => {
                let dict = PyDict::new(py);
                dict.set_item(py, "e", "spoilertext")?;
                dict.set_item(py, "c", PyList::new(py, &[]))?;
                Ok(dict)
            }
            NodeValue::Image(..) => unreachable!()
        }
    }
}
