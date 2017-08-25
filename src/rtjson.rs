use ctype::isspace;
use nodes::{TableAlignment, NodeValue, ListType, AstNode};
use parser::ComrakOptions;

/// Formats an AST as HTML, modified by the given options.
pub fn format_document<'a>(root: &'a AstNode<'a>, options: &ComrakOptions) -> String {
    let mut f = RTJsonFormatter::new(options);
    f.format(root, false);
    f.s
}

struct RTJsonFormatter<'o> {
    s: String,
    options: &'o ComrakOptions,
}

impl<'o> RTJsonFormatter<'o> {
    fn new(options: &'o ComrakOptions) -> Self {
        RTJsonFormatter {
            s: String::with_capacity(1024),
            options: options,
        }
    }

    fn escape(&mut self, buffer: &str) -> String {
        lazy_static! {
            static ref NEEDS_ESCAPED: [bool; 256] = {
                let mut sc = [false; 256];
                for &c in &['"', '&', '<', '>'] {
                    sc[c as usize] = true;
                }
                sc
            };
        }

        let src = buffer.as_bytes();
        let size = src.len();
        let mut i = 0;
        let mut text = String::with_capacity(1024);

        while i < size {
            let org = i;
            while i < size && !NEEDS_ESCAPED[src[i] as usize] {
                i += 1;
            }

            if i > org {
                text += &buffer[org..i];
            }

            if i >= size {
                break;
            }

            match src[i] as char {
                '"' => text += "&quot;",
                '&' => text += "&amp;",
                '<' => text += "&lt;",
                '>' => text += "&gt;",
                _ => unreachable!(),
            }

            i += 1;
        }
        text
    }

    fn escape_href(&mut self, buffer: &str) -> String {
        lazy_static! {
            static ref HREF_SAFE: [bool; 256] = {
                let mut a = [false; 256];
                for &c in b"-_.+!*'(),%#@?=;:/,+&$abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".iter() {
                    a[c as usize] = true;
                }
                a
            };
        }

        let src = buffer.as_bytes();
        let size = src.len();
        let mut i = 0;
        let mut text = String::with_capacity(1024);

        while i < size {
            let org = i;
            while i < size && HREF_SAFE[src[i] as usize] {
                i += 1;
            }

            if i > org {
                text += &buffer[org..i];
            }

            if i >= size {
                break;
            }

            match src[i] as char {
                '&' => text += "&amp;",
                '\'' => text += "&#x27;",
                _ => text += &format!("%{:02X}", src[i]),
            }

            i += 1;
        }
        text
    }

    fn append_comma<'a>(&mut self, node: &'a AstNode<'a>) {
        if node.next_sibling().is_some() {
            self.s += ",";
        }
    }

    fn format_children<'a>(&mut self, node: &'a AstNode<'a>, plain: bool) {
        for n in node.children() {
            self.format(n, plain);
        }
    }

    fn format<'a>(&mut self, node: &'a AstNode<'a>, plain: bool) {
        if plain {
            match node.data.borrow().value {
                NodeValue::Text(ref literal) |
                NodeValue::Code(ref literal) => self.s += self.escape(literal).as_str(),
                _ => (),
            }
            self.format_children(node, true);
        } else {
            let new_plain = self.format_node(node, true);
            self.format_children(node, new_plain);
            self.format_node(node, false);
        }
    }

    fn format_node<'a>(&mut self, node: &'a AstNode<'a>, entering: bool) -> bool {
        match node.data.borrow().value {
            NodeValue::Document => {
                if entering {
                    self.s += r#"{"document":["#;
                } else {
                    self.s += "]}";
                }
            },
            NodeValue::BlockQuote => {
                if entering {
                    self.s += r#"{"e":"blockquote","c":["#;
                } else {
                    self.s += "]}";
                }
            }
            NodeValue::List(ref nl) => {
                if entering {
                    if nl.list_type == ListType::Bullet {
                        self.s += r#"{"e":"list","o":false,"c":["#;
                    } else {
                        self.s += r#"{"e":"list","o":true,"c":["#;
                    }
                } else {
                    self.s += "]}";
                }
            }
            NodeValue::Item(..) => {
                if entering {
                    self.s += r#"{"e":"li","c":["#;
                } else {
                    self.s += "]}";
                }
            }
            NodeValue::Heading(ref nch) => {
                if entering {
                    self.s += &format!(r#"{{"e":"h","l":{},"c":["#, nch.level);
                } else {
                    self.s += "]}";
                }
            }
            NodeValue::CodeBlock(ref ncb) => {
                if entering {
                    if ncb.info.is_empty() {
                        self.s += r#"{"e":"code","c":["#;
                    } else {
                        let mut first_tag = 0;
                        while first_tag < ncb.info.len() &&
                              !isspace(ncb.info.as_bytes()[first_tag]) {
                            first_tag += 1;
                        }

                        self.s += format!(r#"{{"e":"code","l":"{}","c":["#, self.escape(&ncb.info[..first_tag])).as_str();
                    }

                    let max = ncb.literal.split("\n").count() - 1;
                    for (i, it) in ncb.literal.split("\n").enumerate() {
                        self.s += format!(r#"{{"e":"raw","t":"{}"}}"#, self.escape(it)).as_str();
                        if i != max {
                            self.s += ",";
                        }
                    }
                    self.s += "]}";
                }
            }
            NodeValue::HtmlBlock(_) => unreachable!(),
            NodeValue::ThematicBreak | NodeValue::LineBreak |
            NodeValue:: SoftBreak => {
                if entering {
                    self.s += r#"{"e":"br"}"#
                }
            }
            NodeValue::Code(_) => {
                if entering {
                    self.s += r#"{"e":"error code"}"#;
                }
            }
            NodeValue::Underline => {
                if entering {
                    self.s += r#"{"e":"error underline"}"#;
                }
            }
            NodeValue::Strong | NodeValue::Emph | NodeValue::Superscript |
            NodeValue::Strikethrough => unreachable!(),
            NodeValue::Paragraph => {
                if entering {
                    self.s += r#"{"e":"par","c":["#;
                } else  {
                    self.s += "]}";
                }
            }
            NodeValue::Text(ref literal) => {
                if entering {
                    match node.parent().unwrap().data.borrow().value {
                        NodeValue::Link(_) => self.s += self.escape(literal).as_str(),
                        NodeValue::Image(_) => self.s += self.escape(literal).as_str(),
                        NodeValue::Text(ref literal) |
                        NodeValue::Code(ref literal) => self.s += self.escape(literal).as_str(),
                        NodeValue::LineBreak | NodeValue::SoftBreak | NodeValue::ThematicBreak => self.s += r#"{"e":"br"},"#,
                        NodeValue::Heading(_) | NodeValue::CodeBlock(_) => {
                            self.s += format!(r#"{{"e":"raw","t":"{}"}}"#, self.escape(literal)).as_str();
                        }
                        NodeValue::BlockQuote  | NodeValue::Paragraph => {
                            self.s += format!(r#"{{"e":"text","t":"{}"}}"#, self.escape(literal)).as_str();
                        }
                        NodeValue::TableCell  => {
                            let row = &node.parent().unwrap().parent().unwrap().data.borrow().value;
                            let in_header = match *row {
                                NodeValue::TableRow(header) => header,
                                _ => panic!(),
                            };
                            if in_header {
                                self.s += format!( r#""e":"text","t":"{}"}}"#, self.escape(literal)).as_str();
                            } else {
                                self.s += format!( r#"{{"e":"text","t":"{}"}}"#, self.escape(literal)).as_str();
                            }
                        }
                        NodeValue::Document | NodeValue::Strong | NodeValue::Emph |
                        NodeValue::Underline | NodeValue::Superscript |
                        NodeValue::Strikethrough => unreachable!(),
                        NodeValue::List(_) | NodeValue::Item(_) | NodeValue::HtmlBlock(_) |
                        NodeValue::Table(_) | NodeValue::TableRow(_) => unreachable!(),
                        NodeValue::FormattedText(_, _) | NodeValue::UnformattedLink(_, _) => unreachable!(),
                        NodeValue::FormattedLink(_,_,_) => unreachable!(),
                    }
                }
            }
            NodeValue::FormattedText(ref literal, ref format_ranges) => {
                if entering {
                    match node.parent().unwrap().data.borrow().value {
                        NodeValue::TableCell  => {
                            let row = &node.parent().unwrap().parent().unwrap().data.borrow().value;
                            let in_header = match *row {
                                NodeValue::TableRow(header) => header,
                                _ => panic!(),
                            };
                            if in_header {
                                self.s += format!( r#""e":"text","t":"{}","f":{:?}}}"#, self.escape(literal), format_ranges).as_str();
                            } else {
                                self.s += format!( r#"{{"e":"text","t":"{}","f":{:?}}}"#, self.escape(literal), format_ranges).as_str();
                            }
                        },
                        NodeValue::Link(_) => self.s += self.escape(literal).as_str(),
                        NodeValue::Image(_) => self.s += self.escape(literal).as_str(),
                        NodeValue::Text(ref literal) | 
                        NodeValue::Code(ref literal) => self.s += self.escape(literal).as_str(),
                        NodeValue::LineBreak | NodeValue::SoftBreak | NodeValue::ThematicBreak => self.s += r#"{"e":"br"},"#,
                        NodeValue::Heading(_) | NodeValue::CodeBlock(_) => {
                            self.s += format!(r#"{{"e":"raw","t":"{}"}}"#, self.escape(literal)).as_str();
                        },
                        NodeValue::Paragraph  => {
                            self.s += format!(r#"{{"e":"text","t":"{}","f":{:?}}}"#, self.escape(literal), format_ranges).as_str();
                        }
                        NodeValue::Document | NodeValue::Strong | NodeValue::Emph|
                        NodeValue::Underline | NodeValue::Superscript |
                        NodeValue::Strikethrough | NodeValue::BlockQuote => unreachable!(),
                        NodeValue::List(_) | NodeValue::Item(_) | NodeValue::HtmlBlock(_) |
                        NodeValue::Table(_) | NodeValue::TableRow(_) => unreachable!(),
                        NodeValue::FormattedText(_, _) | NodeValue::UnformattedLink(_, _) => unreachable!(),
                        NodeValue::FormattedLink(_,_,_) => unreachable!(),
                    }
                }
            }
            NodeValue::FormattedLink(ref url, ref literal, ref format_ranges) => {
                if entering {
                    self.s += format!(r#"{{"e":"link","u":"{}","t":"{}","f":{:?}}}"#, self.escape_href(url), self.escape(literal), format_ranges).as_str();
                }
            }
            NodeValue::UnformattedLink(ref url, ref literal) => {
                if entering {
                    self.s += format!(r#"{{"e":"link","u":"{}","t":"{}"}}"#, self.escape_href(url), self.escape(literal)).as_str();
                }
            }
            NodeValue::Link(ref nl) => {
                if entering {
                    self.s += format!(r#"{{"e":"link","u":"{}","t":"{}"}}"#, self.escape_href(&nl.url), self.escape(&nl.title)).as_str();
                }
            }
            NodeValue::Link(ref nl) => {
                if entering {
                    self.s += format!(r#"{{"e":"link","u":"{}","t":"{}"}}"#, self.escape_href(&nl.url), self.escape(&nl.title)).as_str();
                    self.append_comma(node);
                }
            }
            NodeValue::Image(ref nl) => {
                if entering {
                    self.s += format!(r#"{{"e":"link","u":"{}","t":""#, self.escape_href(&nl.url)).as_str();
                } else {
                    self.s += r#""}"#;
                    self.append_comma(node);
                }
            }
            NodeValue::Table(..) => {
                if entering {
                    self.s += r#"{"e":"table","#;
                } else {
                    if !node.last_child()
                            .unwrap()
                            .same_node(node.first_child().unwrap()) {
                        self.s += "]";
                    }
                    self.s += "}";
                }
            }
            NodeValue::TableRow(header) => {
                if entering {
                    if header {
                        self.s += r#""h":["#;
                    }
                    self.s += "[";
                } else {
                    self.s += "]";
                    if node.next_sibling().is_some() && !header {
                        self.s += ",";
                    }
                    if header {
                        self.s += "],";
                        self.s += r#""c":["#;
                    }
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

                if entering {

                    let mut start = node.parent().unwrap().first_child().unwrap();
                    let mut i = 0;
                    while !start.same_node(node) {
                        i += 1;
                        start = start.next_sibling().unwrap();
                    }

                    if in_header {
                        self.s += "{";

                        match alignments[i] {
                            TableAlignment::Left => self.s += r#""a":"L","#,
                            TableAlignment::Right => self.s += r#""a":"R","#,
                            TableAlignment::Center => self.s += r#""a":"C","#,
                            TableAlignment::None => (),
                        }
                    } else {
                        self.s += r#"{"c":["#;
                    }

                } else if !in_header {
                    self.s += "]}";
                }
            }
        }
        if !entering {
            self.append_comma(node);
        }
        false
    }
}
