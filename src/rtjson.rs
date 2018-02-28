use nodes::{TableAlignment, NodeValue, ListType, AstNode};
use parser::ComrakOptions;
use serde_json;

/// Formats an AST as HTML, modified by the given options.
pub fn format_document<'a>(root: &'a AstNode<'a>, options: &ComrakOptions) -> serde_json::Value {
    let mut f = RTJsonFormatter::new(options);
    f.format(root).unwrap()
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

    fn format_children<'a>(&mut self, node: &'a AstNode<'a>, content: &mut serde_json::Value) {
        let mut vals = Vec::with_capacity(128);
        for n in node.children() {
            let js = self.format(n).to_owned();
            match js {
                Some(k) => vals.push(k),
                None => (),
            }
        }
        *content = json!(vals);
    }

    fn format<'a>(&mut self, node: &'a AstNode<'a>) -> Option<serde_json::Value> {
        let mut content = &mut json!({});
        self.format_children(node, content);
        let mut json = match self.format_node(node) {
            Some(k) => k,
            None => serde_json::Value::Null
        };
        if json == serde_json::Value::Null {
            return None
        }
        match node.data.borrow().value {
            NodeValue::Document => {
                json["document"] = content.to_owned();
            }
            NodeValue::Table(..) => {
                let mut vals = vec![];
                for val in content.as_array_mut().unwrap() {
                    if val.get("h") != None {
                        json["h"] = val.get("h").unwrap_or(&serde_json::Value::Null).to_owned();
                    } else {
                        vals.push(val.get("c").unwrap_or(&serde_json::Value::Null));
                    }
                }
                json["c"] = json!(vals);
            }
            NodeValue::TableRow(..) => {
                match json.clone().get_mut("h") {
                    Some(_h) => json["h"] = content.to_owned(),
                    None => json["c"] = content.to_owned(),
                }
            }
            NodeValue::Item(..) => {
                if content[0].get("e") != None && content[0].get("e").unwrap() == "list" {
                    json["c"] = json!([json["c"], content.clone()]);
                } else {
                    json["c"] = content.clone();
                }
            }
            _ => {
                if !content.as_array().unwrap().is_empty() {
                    json["c"] = content.to_owned();
                }
            }
        }
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
                    "c": [{
                        "c": [{
                            "e": "text",
                            "t": ""
                        }],
                        "e": "par"
                    }],
                }))
            }
            NodeValue::Heading(ref nch) => {
                Some(json!({
                    "e": "h",
                    "l": nch.level,
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
            NodeValue::HtmlBlock(_) => None,
            NodeValue::ThematicBreak | NodeValue::LineBreak |
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
                   NodeValue::TableCell | NodeValue::Paragraph | NodeValue::BlockQuote => {
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
                   NodeValue::TableCell | NodeValue::Paragraph | NodeValue::BlockQuote => {
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
                }))
            }
            NodeValue::Image(ref nl) => {
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
            NodeValue::FootnoteReference(..) => None
        }
    }
}
