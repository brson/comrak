use nodes::{TableAlignment, NodeValue, ListType, AstNode};
use parser::ComrakOptions;
use serde_json;

// This is a wrapper type that does nothing but ensure that the JSON value is
// destroyed without blowing the stack - deeply nested JSON dictionaries use
// recursion. This is arguably a bug in serde_json.
// FIXME: Remove when https://github.com/serde-rs/json/issues/440 is fixed.
#[derive(Debug)]
pub struct Json(pub serde_json::Value);

impl Drop for Json {
    fn drop(&mut self) {
        // We're going to iteratively peel apart the entire tree, by removing
        // child nodes from their parents and dropping them. The root is a
        // special case since it's not passed by value, so we first have to
        // remove its children and push them onto the stack, then we'll start
        // iterating on the stack and taking apart their children.

        let mut stack = vec![];

        // This first step is pretty distasteful but I don't see offhad a better
        // way to do it: while there are still values in the root dictionary,
        // copy the entire key string, then remove the value by key.
        let root_map = self.0.as_object_mut().expect("document root should be a map");
        let mut key = String::new();
        while !root_map.is_empty() {
            key.clear();
            key.push_str(root_map.keys().next().unwrap());
            stack.push(root_map.remove(&key).unwrap());
        }

        while let Some(val) = stack.pop() {
            match val {
                serde_json::Value::Object(val) => {
                    for (_, child) in val.into_iter() {
                        stack.push(child);
                    }
                }
                serde_json::Value::Array(val) => {
                    for child in val.into_iter() {
                        stack.push(child);
                    }
                }
                _ => (),
            }
        }
    }
}

/// Formats an AST as HTML, modified by the given options.
pub fn format_document<'a>(root: &'a AstNode<'a>, options: &ComrakOptions) -> Json {
    let mut f = RTJsonFormatter::new(options);
    Json(f.format(root).unwrap())
}

struct RTJsonFormatter<'o> {
    options: &'o ComrakOptions,
}

impl<'o> RTJsonFormatter<'o> {
    fn new(options: &'o ComrakOptions) -> Self {
        RTJsonFormatter {
            options: options,
        }
    }

    fn format<'a>(&mut self, root_node: &'a AstNode<'a>) -> Option<serde_json::Value> {

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
                    let mut json = match self.format_node(node) {
                        Some(k) => k,
                        None => serde_json::Value::Null
                    };
                    if json == serde_json::Value::Null {
                        continue
                    }

                    let children = accum.pop().expect("json child nodes");
                    let content = serde_json::Value::Array(children);

                    // Then add the child content to the node
                    match node.data.borrow().value {
                        NodeValue::Document => {
                            json["document"] = content;
                        }
                        NodeValue::Table(..) => {
                            let mut vals = vec![];
                            let mut content = content;
                            for val in content.as_array_mut().unwrap() {
                                if let Some(map) = val.as_object_mut() {
                                    if let Some(c) = map.remove("c") {
                                        vals.push(c);
                                    } else if let Some(h) = map.remove("h") {
                                        json["h"] = h;
                                    } else {
                                        unreachable!();
                                    }
                                }
                            }
                            json["c"] = serde_json::Value::Array(vals);
                        }
                        NodeValue::TableRow(..) => {
                            match json.clone().get_mut("h") {
                                Some(_h) => json["h"] = content,
                                None => json["c"] = content,
                            }
                        }
                        _ => {
                            if !content.as_array().unwrap().is_empty() {
                                json["c"] = content;
                            }
                        }
                    }

                    // Finally, push this rendered node onto it's parent's list of
                    // accumulated children.
                    let last = accum.last_mut().expect("child accum");
                    last.push(json);
                }
            }
        }

        let mut last_accum = accum.pop().expect("last accumulator");
        assert!(accum.is_empty());
        let json = last_accum.pop().expect("last json");
        assert!(last_accum.is_empty());
        Some(json)
    }

    fn format_node<'a>(&mut self, node: &'a AstNode<'a>) -> Option<serde_json::Value> {
        match node.data.borrow().value {
            NodeValue::Document => {
                Some(json!({
                    "document": ""
                }))
            },
            NodeValue::BlockQuote => {
                Some(json!({
                    "e": "blockquote",
                }))
            }
            NodeValue::List(ref nl) => {
                Some(json!({
                    "e": "list",
                    "o": if nl.list_type != ListType::Bullet {true} else {false},
                }))
            }
            NodeValue::Item(..) => {
                Some(json!({
                    "e": "li",
                }))
            }
            NodeValue::Heading(ref nch) => {
                Some(json!({
                    "e": "h",
                    "l": nch.level,
                    "c": json!([{
                        "e":"raw",
                        "t":""
                    }])
                }))
            }
            NodeValue::CodeBlock(ref ncb) => {
                let mut int = Vec::with_capacity(128);
                let code_content = String::from_utf8(ncb.literal.to_owned()).unwrap();
                let max = code_content.split("\n").count() - 1;
                for (i, it) in code_content.split("\n").enumerate() {
                    if i != max {
                        int.push(json!({
                            "e": "raw",
                            "t": it
                        }).clone());
                    }
                }
                Some(json!({
                    "e": "code",
                    "c": int
                }))
            }
            NodeValue::ThematicBreak => {
                Some(json!({
                    "e": "hr"
                }))
            },
            NodeValue::LineBreak => {
                Some(json!({
                    "e": "br"
                }))
            },
            NodeValue::HtmlBlock(ref nhb) => unreachable!(),
            NodeValue:: SoftBreak => None,
            NodeValue::Code(_) | NodeValue::Strong | NodeValue::Emph | NodeValue::Superscript |
            NodeValue::Strikethrough | NodeValue::Underline => unreachable!(),
            NodeValue::Paragraph => {
                Some(json!({
                    "e": "par",
                }))
            }
            NodeValue::Text(ref literal) => {
                match node.parent().unwrap().data.borrow().value {
                    NodeValue::Heading(..) | NodeValue::CodeBlock(..) => {
                       Some(json!({
                           "e":"raw",
                           "t": String::from_utf8(literal.to_owned()).unwrap()
                       }))
                   }
                   NodeValue::TableCell | NodeValue::Paragraph | NodeValue::BlockQuote
                   | NodeValue::SpoilerText => {
                       Some(json!({
                           "e": "text",
                           "t": String::from_utf8(literal.to_owned()).unwrap(),
                       }))
                   }
                   _ => unreachable!(),
                }
            }
            NodeValue::FormattedText(ref literal, ref format_ranges) => {
                match node.parent().unwrap().data.borrow().value {
                    NodeValue::Heading(..) | NodeValue::CodeBlock(..) => {
                       Some(json!({
                           "e":"raw",
                           "t": String::from_utf8(literal.to_owned()).unwrap()
                       }))
                   }
                   NodeValue::TableCell | NodeValue::Paragraph | NodeValue::BlockQuote
                   | NodeValue::SpoilerText => {
                       Some(json!({
                           "e": "text",
                           "t": String::from_utf8(literal.to_owned()).unwrap(),
                           "f": format_ranges
                       }))
                   }
                   _ => unreachable!(),
                }
            }
            NodeValue::FormattedLink(ref nl) => {
                if !&nl.element.is_empty() {
                    Some(json!({
                        "e":"link",
                        "u": String::from_utf8(nl.url.to_owned()).unwrap(),
                        "t": String::from_utf8(nl.caption.to_owned()).unwrap(),
                        "f": &nl.format_range,
                        "a": String::from_utf8(nl.element.to_owned()).unwrap(),
                    }))
                } else {
                    Some(json!({
                        "e":"link",
                        "u": String::from_utf8(nl.url.to_owned()).unwrap(),
                        "t": String::from_utf8(nl.caption.to_owned()).unwrap(),
                        "f":&nl.format_range,
                    }))
                }
            }
            NodeValue::UnformattedLink(ref nl) => {
                if !&nl.element.is_empty() {
                    Some(json!({
                        "e": "link",
                        "u": String::from_utf8(nl.url.to_owned()).unwrap(),
                        "t": String::from_utf8(nl.caption.to_owned()).unwrap(),
                        "a": String::from_utf8(nl.element.to_owned()).unwrap()
                    }))
                } else {
                    Some(json!({
                        "e":"link",
                        "u": String::from_utf8(nl.url.to_owned()).unwrap(),
                        "t": String::from_utf8(nl.caption.to_owned()).unwrap(),
                    }))
                }
            }
            NodeValue::Link(ref nl) => {
                Some(json!({
                    "e":"link",
                    "u": String::from_utf8(nl.url.to_owned()).unwrap(),
                    "t": String::from_utf8(nl.title.to_owned()).unwrap(),
                }))
            }
            NodeValue::RedditLink(ref nl) => {
                Some(json!({
                    "e": String::from_utf8(nl.url.to_owned()).unwrap(),
                    "t": String::from_utf8(nl.title.to_owned()).unwrap(),
                    "l":nl.l,
                }))
            }
            NodeValue::Media(ref nl) => {
                if !&nl.title.is_empty() {
                    Some(json!({
                        "e":  String::from_utf8(nl.e.to_owned()).unwrap(),
                        "id": String::from_utf8(nl.url.to_owned()).unwrap(),
                        "c":  String::from_utf8(nl.title.to_owned()).unwrap(),
                    }))
                } else {
                    Some(json!({
                        "e": String::from_utf8(nl.e.to_owned()).unwrap(),
                        "id": String::from_utf8(nl.url.to_owned()).unwrap(),
                    }))
                }
            }
            NodeValue::Table(..) => {
                Some(json!({
                    "e": "table",
                }))
            }
            NodeValue::TableRow(header) => {
                if header {
                    Some(json!({
                        "h" : []
                    }))
                } else {
                    Some(json!({
                        "c" : []
                    }))
                }
            }
            NodeValue::TableCell => {
                let row = &node.parent().unwrap().data.borrow().value;
                let in_header = match *row {
                    NodeValue::TableRow(header) => header,
                    _ => panic!(),
                };

                let table = &node.parent()
                                 .unwrap()
                                 .parent()
                                 .unwrap()
                                 .data
                                 .borrow()
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
                    Some(json!({
                        "a": match alignments[i] {
                            TableAlignment::Left => "L",
                            TableAlignment::Right => "R",
                            TableAlignment::Center => "C",
                            TableAlignment::None => "",
                        },
                    }))
                } else {
                    Some(json!({
                        "c": []
                    }))
                }
            }
            NodeValue::FootnoteDefinition(..) => None,
            NodeValue::HtmlInline(ref nd) => unreachable!(),
            NodeValue::FootnoteReference(..) => None,
            NodeValue::SpoilerText => {
                Some(json!({
                    "e": "spoilertext",
                    "c": [],
                }))
            }
            NodeValue::Image(..) => unreachable!()
        }
    }
}
