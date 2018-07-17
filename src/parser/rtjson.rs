//! RTJSON-specific AST processing

use super::Parser;
use super::inlines;
use nodes::{AstNode, NodeValue, NodeFormatLink};
use std::collections::HashMap;
use std::str;

// The 'official' node nesting limit (the number we've arbitrarily and
// informally decided) in redesign is 32 levels. That's probably best defined as
// the number of levels of JSON that a client needs to be able to deserialize
// and render. For fear of off-by-N errors between snoomark and clients, we
// actually limit nesting to 30 levels.
const NESTED_NODE_LIMIT: u32 = 30;

impl<'a, 'o> Parser<'a, 'o> {
    #[cfg_attr(feature = "flamegraphs", flame)]
    fn reset_rtjson_node(
        &mut self,
        unformatted_text: &mut Vec<u8>,
        format_ranges: &mut Vec<[u16; 3]>,
        range_idx: &mut u16,
    ) {
        unformatted_text.clear();
        format_ranges.clear();
        *range_idx = 0;
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    fn insert_format(&mut self, current_format: &mut HashMap<u16, u16>, val: u16) {
        *current_format.entry(val).or_insert(0) += 1;
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    fn remove_format(&mut self, current_format: &mut HashMap<u16, u16>, val: u16) {
        *current_format.entry(val).or_insert(1) -= 1;
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    fn consolidate_format(&mut self, format_ranges: &mut Vec<[u16; 3]>) {
        // TODO[shaq|Nov-27-2017]: This errors out on post longer than
        // 65K, can we possibly fix that or make a better exit happen
        let mut prev_end = format_ranges[0][1] + format_ranges[0][2];
        let mut prev_style = format_ranges[0][0];
        let mut new_format: Vec<[u16; 3]> = Vec::with_capacity(format_ranges.len());
        let mut buffer_range = format_ranges[0];
        buffer_range[2] = 0;
        for (i, range) in format_ranges.iter().enumerate() {
            if i == 0 {
                buffer_range[2] += range[2];
                continue;
            }
            let curr_style = range[0];
            let curr_beg = range[1];
            if !(prev_style == range[0] && prev_end == curr_beg) {
                new_format.push(buffer_range);
                buffer_range = *range;
            } else {
                buffer_range[2] += range[2];
            }
            prev_end = curr_beg + range[2];
            prev_style = curr_style;
        }
        new_format.push(buffer_range);
        *format_ranges = new_format;
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    fn output_format_range(
            &mut self,
            unformatted_text: &mut Vec<u8>,
            current_format: &mut HashMap<u16, u16>,
            format_ranges: &mut Vec<[u16; 3]>,
            range_idx: &mut u16,
            text: &[u8],
    ) {
        let mut sum: u16 = 0;
        for (key, val) in current_format.iter() {
            if *val > 0 {
                sum += *key;
            }
        }

        let range_length = str::from_utf8(text).expect("utf8").chars().count() as u16;
        if sum > 0 {
            let new_range = [sum, *range_idx, range_length];
            format_ranges.push(new_range);
        }
        unformatted_text.extend_from_slice(text);
        *range_idx += range_length;
    }

    #[cfg_attr(any(feature = "flamegraphs", feature = "minflame"), flame)]
    pub fn postprocess_rtjson_ast(
        &mut self,
        root_node: &'a AstNode<'a>,
        unformatted_text: &mut Vec<u8>,
        current_format: &mut HashMap<u16, u16>,
        format_ranges: &mut Vec<[u16; 3]>,
        range_idx: &mut u16,
    ) {
        // This function does its work in a loop to avoid recursion. It has work
        // to do on each node both before and after visiting children, so each
        // node gets two turns on the work-stack.
        enum Phase { Pre, Post };

        let mut stack = vec![(root_node, Phase::Pre)];

        // Tracking the nesting level. In this traversal we don't actually try
        // to denest because there's too much tree manipulation going on to
        // easily track exactly how deep any node is going to end up without
        // doing a bunch of extra work in the post-traversal phase. So this
        // traversal is just making a conservative guess as to whether the tree
        // is too deep.
        let mut nested_level = 0;
        let mut may_need_denesting = false;

        while let Some((node, phase)) = stack.pop() {
            match phase {
                Phase::Pre => {
                    let skip = self.postprocess_rtjson_ast_pre(
                        node,
                        unformatted_text,
                        current_format,
                        format_ranges,
                        range_idx
                    );

                    if skip { continue }

                    // Push the current working-node back onto the stack for
                    // post-traversal processing, then the children onto the
                    // stack in pre-traversal mode, in reverse order so that the
                    // first child is processed first,
                    stack.push((node, Phase::Post));
                    stack.extend(node.reverse_children().map(|cn| (cn, Phase::Pre)));

                    if self.count_as_nested(node) {
                        nested_level += 1;

                        may_need_denesting |= nested_level > NESTED_NODE_LIMIT;
                    }
                }
                Phase::Post => {
                    if self.count_as_nested(node) {
                        nested_level -= 1;
                    }

                    self.postprocess_rtjson_ast_post(
                        node,
                        unformatted_text,
                        current_format,
                        format_ranges,
                        range_idx
                    );
                }
            }
        }

        if may_need_denesting {
            self.denest(root_node);
        }

        assert_eq!(nested_level, 0);
    }

    // The pre-traversal phase of rtjson postprocessing
    //
    // If this function returns true than further processing
    // of this subtree should be aborted.
    fn postprocess_rtjson_ast_pre(
        &mut self,
        node: &'a AstNode<'a>,
        unformatted_text: &mut Vec<u8>,
        current_format: &mut HashMap<u16, u16>,
        format_ranges: &mut Vec<[u16; 3]>,
        range_idx: &mut u16,
    ) -> bool {
        match node.data.borrow_mut().value {
            NodeValue::Text(ref text) => {
                self.output_format_range(
                    unformatted_text,
                    current_format,
                    format_ranges,
                    range_idx,
                    text
                );
            },
            NodeValue::SoftBreak => {
                self.output_format_range(
                    unformatted_text,
                    current_format,
                    format_ranges,
                    range_idx,
                    &[32],
                );
                node.detach();
            }
            NodeValue::Link(..)
            | NodeValue::UnformattedLink(..)
            | NodeValue::RedditLink(..)
            | NodeValue::SpoilerText
            | NodeValue::LineBreak => {
                if !unformatted_text.is_empty() {

                    let text_node = if format_ranges.is_empty() {
                        NodeValue::Text(
                            unformatted_text.to_vec(),
                        )
                    } else {
                        self.consolidate_format(format_ranges);
                        NodeValue::FormattedText(
                            unformatted_text.to_vec(),
                            format_ranges.to_owned()
                        )
                    };

                    let inline_text_node = inlines::make_inline(
                        self.arena,
                        text_node
                    );
                    node.insert_before(inline_text_node);
                    self.reset_rtjson_node(unformatted_text, format_ranges, range_idx);
                }
            },
            NodeValue::Media(..) => {
                // Image syntax in rtjson is actually a media element, with the
                // 'text' being either "img", "vid", or "gif". Any other
                // childeren beneath the image results in something unsupported.
                // Media nodes are generated by the RTE, but can't be generated
                // by hand. Images are normally inline elements, and the RTE
                // generates them inside paragraphs, so this code pulls the
                // image (media) node out of the paragraph so the RTJSON
                // formatter will put it at the document level (assuming that
                // the RTE wrote the images in paragraphs at the document
                // level).
                //
                // Furthermore, though, the CM AST allows arbitrary inline elements
                // as children of image nodes (for what reason I don't understand
                // - HTML can't render arbitrary elements inside images sensibly
                // AFAICT).
                //
                // So we're going to do some sanity checks, first that the
                // parent is a paragraph, and _its_ parent is the document; then
                // that the only child is a text node containing a valid media
                // type. If none of that is true then we're just going to drop
                // the node on the floor; but also replace it with text that
                // vaguelly represents the node's textual contents, similar to
                // the way the HTML renderer render's Image nodes' children as
                // "plain" text.

                // Just throw away any preceding text in this paragraph for now.
                self.reset_rtjson_node(unformatted_text, format_ranges, range_idx);

                let parents_are_cool = if let Some(parent) = node.parent() {
                    match parent.data.borrow().value {
                        NodeValue::Paragraph => {
                            if let Some(grandparent) = parent.parent() {
                                match grandparent.data.borrow().value {
                                    NodeValue::Document => true,
                                    _ => false,
                                }
                            } else {
                                // Weird case.
                                //
                                // This shouldn't come up in markdown submitted
                                // by the RTE, but is possible syntactically.
                                // Because below we detach the _parent_ paragraph
                                // from the AST on first encounter of an Image,
                                // it's possible for us to get back here and be
                                // working on a detached subtree, in which case
                                // anything we do is not going to show up in
                                // the output.
                                //
                                // Just return false for now.
                                false
                            }
                        }
                        _ => false,
                    }
                } else {
                    unreachable!("images should have parents")
                };

                let one_child = node.children().count() == 1;
                let valid_media_type = match node.first_child() {
                    Some(n) => {
                        match n.data.borrow().value {
                            NodeValue::Text(ref t) => {
                                static MEDIA_TYPES: &[&[u8]] = &[b"img", b"video", b"gif"];
                                MEDIA_TYPES.iter().any(|m| *m == t.as_slice())
                            }
                            _ => false,
                        }
                    }
                    None => false,
                };

                let everything_is_cool =
                    parents_are_cool &&
                    one_child &&
                    valid_media_type;

                if everything_is_cool {
                    let parent_paragraph = node.parent().unwrap();
                    parent_paragraph.insert_before(node);
                    parent_paragraph.detach();
                } else {
                    // Traverse the children and render them as 'plain' text,
                    // replace the image node with a text node.
                    let mut stack = vec![];
                    for n in node.reverse_children() {
                        stack.push(n);
                    }
                    let mut accum = vec![];
                    while let Some(n) = stack.pop() {
                        match n.data.borrow().value {
                            NodeValue::Text(ref literal)
                            | NodeValue::Code(ref literal)
                            | NodeValue::HtmlInline(ref literal) => {
                                accum.extend(literal)
                            }
                            NodeValue::LineBreak | NodeValue::SoftBreak => {
                                accum.extend(b" ");
                            }
                            _ => (),
                        }
                        for n in n.reverse_children() {
                            stack.push(n);
                        }
                    }
                    let new = inlines::make_inline(self.arena, NodeValue::Text(accum));
                    node.insert_before(new);
                    node.detach();
                    return true;
                }
            }
            NodeValue::Image(..) => {
                unreachable!("rtjson preduces media elements, not image");
            }
            NodeValue::Code(ref literal) => {
                let range_length = str::from_utf8(literal).expect("utf8").chars().count() as u16;
                let new_range = [64, *range_idx, range_length];
                format_ranges.push(new_range);
                unformatted_text.extend_from_slice(literal);
                *range_idx += str::from_utf8(literal).expect("utf8").chars().count() as u16
            },
            NodeValue::HtmlBlock(ref nhb) => {
                self.output_format_range(
                    unformatted_text,
                    current_format,
                    format_ranges,
                    range_idx,
                    &nhb.literal,
                );

                let text_node = if format_ranges.is_empty() {
                    NodeValue::Text(
                        unformatted_text.to_vec(),
                    )
                } else {
                    self.consolidate_format(format_ranges);
                    NodeValue::FormattedText(
                        unformatted_text.to_vec(),
                        format_ranges.to_owned()
                    )
                };
                let par_inl = inlines::make_inline(
                    self.arena,
                    NodeValue::Paragraph
                );
                let inline_text_node = inlines::make_inline(
                    self.arena,
                    text_node
                );
                par_inl.append(inline_text_node);

                node.insert_before(par_inl);
                self.reset_rtjson_node(unformatted_text, format_ranges, range_idx);
                node.detach();
            }
            NodeValue::Strong => self.insert_format(current_format, 1),
            NodeValue::Emph => self.insert_format(current_format, 2),
            NodeValue::Strikethrough => self.insert_format(current_format, 8),
            NodeValue::Superscript => self.insert_format(current_format, 32),
            _ => ()
        }

        false
    }

    // The post-traversal phase of rtjson postprocessing
    fn postprocess_rtjson_ast_post(
        &mut self,
        node: &'a AstNode<'a>,
        unformatted_text: &mut Vec<u8>,
        current_format: &mut HashMap<u16, u16>,
        format_ranges: &mut Vec<[u16; 3]>,
        range_idx: &mut u16,
    ) {
        match node.data.borrow().value {
            NodeValue::Item(..) => {
                match node.children().next() {
                    None => {
                        let par_inl = inlines::make_inline(
                            self.arena,
                            NodeValue::Paragraph,
                        );
                        let inl = inlines::make_inline(
                            self.arena,
                            NodeValue::Text(
                                b"".to_vec()
                            ),
                        );
                        par_inl.append(inl);
                        node.prepend(par_inl);
                    },
                    Some(n) => {
                        match n.data.borrow().value {
                            NodeValue::List(..) => {
                                let par_inl = inlines::make_inline(
                                    self.arena,
                                    NodeValue::Paragraph,
                                );
                                let inl = inlines::make_inline(
                                    self.arena,
                                    NodeValue::Text(
                                        b"".to_vec()
                                    ),
                                );
                                par_inl.append(inl);
                                node.prepend(par_inl);
                            },
                            _ => ()
                        }
                    }
                }
            },
            _ => (),
        }

        match node.data.borrow().value {
            NodeValue::Strong => self.remove_format(current_format, 1),
            NodeValue::Emph => self.remove_format(current_format, 2),
            NodeValue::Strikethrough => self.remove_format(current_format, 8),
            NodeValue::Superscript => self.remove_format(current_format, 32),
            _ => ()
        }

        if node.data.borrow_mut().value.contains_inlines() {
            match node.data.borrow_mut().value {
                NodeValue::Link(ref nl) => {
                    let link_node = if format_ranges.is_empty() {
                        NodeValue::UnformattedLink(NodeFormatLink{
                            url: nl.to_owned().url,
                            caption: unformatted_text.to_vec(),
                            element: nl.to_owned().title,
                            format_range: format_ranges.to_owned(),
                        })
                    } else {
                        self.consolidate_format(format_ranges);
                        NodeValue::FormattedLink(NodeFormatLink{
                            url: nl.to_owned().url,
                            caption: unformatted_text.to_vec(),
                            format_range: format_ranges.to_owned(),
                            element: nl.to_owned().title,
                        })
                    };
                    let inline_text_node = inlines::make_inline(
                        self.arena,
                        link_node
                    );
                    node.insert_before(inline_text_node);
                    self.reset_rtjson_node(unformatted_text, format_ranges, range_idx);
                    node.detach();
                }
                _ => {
                    if !unformatted_text.is_empty() {
                        let formatted_text_node = if !format_ranges.is_empty() {
                            self.consolidate_format(format_ranges);
                            inlines::make_inline(
                                self.arena,
                                NodeValue::FormattedText(
                                    unformatted_text.to_vec(),
                                    format_ranges.to_owned()
                                ),
                            )
                        } else {
                            inlines::make_inline(
                                self.arena,
                                NodeValue::Text(
                                    unformatted_text.to_vec()
                                ),
                            )
                        };
                        node.append(formatted_text_node);
                        self.reset_rtjson_node(unformatted_text, format_ranges, range_idx);
                    }
                }
            }
        } else {
            match node.data.borrow_mut().value {
                NodeValue::Text(..)
                | NodeValue::Emph
                | NodeValue::Strong
                | NodeValue::Strikethrough
                | NodeValue::Superscript
                | NodeValue::Code(..) => {
                    for ch in node.children() {
                        node.insert_before(ch);
                    }
                    node.detach()
                },
                NodeValue::Media(ref mut nl) => {
                    nl.e =  unformatted_text.to_vec();
                    self.reset_rtjson_node(unformatted_text, format_ranges, range_idx);
                }
                _ => ()
            }
        }
    }

    fn denest(&self, root_node: &'a AstNode<'a>) {
        enum Phase { Pre, Post };

        let mut stack = vec![(root_node, Phase::Pre)];

        let mut nested_level = 0;

        while let Some((node, phase)) = stack.pop() {
            match phase {
                Phase::Pre => {
                    nested_level += 1;

                    if nested_level <= NESTED_NODE_LIMIT {
                        stack.push((node, Phase::Post));
                        stack.extend(node.reverse_children().map(|cn| (cn, Phase::Pre)));
                    } else {
                        // Limit exceeded, detach and stop the descent
                        self.detach_nearest_removable(node);
                    }
                }
                Phase::Post => {
                    nested_level -= 1;
                }
            }
        }
    }

    fn detach_nearest_removable(&self, node: &'a AstNode<'a>) {
        let mut node = node;

        while let Some(parent) = node.parent() {
            if self.is_removable(node) {
                node.detach();
                return;
            }

            node = parent;
        }
    }

    // To enforce nesting limits we need to sometimes remove nodes, but because
    // there are sometimes invariants between nodes (e.g. Table contains
    // TableRow) we only remove some node types. These are they.
    fn is_removable(&self, node: &'a AstNode<'a>) -> bool {
        match node.data.borrow().value {
            NodeValue::BlockQuote
            | NodeValue::FootnoteDefinition(_)
            | NodeValue::List(..)
            | NodeValue::Item(..)
            | NodeValue::CodeBlock(..)
            | NodeValue::HtmlBlock(..)
            | NodeValue::Paragraph
            | NodeValue::SpoilerText
            | NodeValue::Heading(..)
            | NodeValue::ThematicBreak
            | NodeValue::Table(..) => {
                return true
            },
            _ => return false,
        }
    }

    // During the initial pass some nodes are guaranteed to be detached,
    // removing a layer of nesting, so they are not counted.
    fn count_as_nested(&self, node: &'a AstNode<'a>) -> bool {
        match node.data.borrow_mut().value {
            NodeValue::Emph
            | NodeValue::Strong
            | NodeValue::Strikethrough
            | NodeValue::Superscript => false,
            _ => true,
        }
    }
}
