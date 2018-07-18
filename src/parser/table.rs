use arena_tree::Node;
use nodes::{make_block, AstNode, NodeValue, TableAlignment};
use parser::Parser;
use scanners;
use std::cell::RefCell;
use std::cmp::min;
use strings::trim;

pub fn try_opening_block<'a, 'o>(
    parser: &mut Parser<'a, 'o>,
    container: &'a AstNode<'a>,
    line: &[u8],
) -> Option<(&'a AstNode<'a>, bool)> {
    let aligns = match container.data.borrow().value {
        NodeValue::Paragraph => None,
        NodeValue::Table(ref aligns) => Some(aligns.clone()),
        _ => return None,
    };

    match aligns {
        None => try_opening_header(parser, container, line),
        Some(ref aligns) => try_opening_row(parser, container, aligns, line),
    }
}

fn try_opening_header<'a, 'o>(
    parser: &mut Parser<'a, 'o>,
    container: &'a AstNode<'a>,
    line: &[u8],
) -> Option<(&'a AstNode<'a>, bool)> {
    if scanners::table_start(&line[parser.first_nonspace..],
                             parser.options.ext_reddit_quirks).is_none() {
        return Some((container, false));
    }

    let header_row = match row(&container.data.borrow().content) {
        Some(header_row) => header_row,
        None => return Some((container, false)),
    };

    let mut marker_row = row(&line[parser.first_nonspace..]).unwrap();

    // snudown allowed the marker row to contain extra junk at the
    // end of the line, here parsed as extra cells.
    if !parser.options.ext_reddit_quirks {
        if header_row.len() != marker_row.len() {
            return Some((container, false));
        }
    } else {
        // Though the marker row at least must be as long as the header row
        if marker_row.len() < header_row.len() {
            return Some((container, false));
        } else {
            // Drop extra marker cells ala snudown
            marker_row.truncate(header_row.len());
        }
        // Additionally, we've still got to verify that all the marker cells
        // contain only marker characters.
        for cell in &marker_row {
            let has_junk = scanners::table_marker(cell)
                .map(|end| end != cell.len()).unwrap_or(true);
            if has_junk {
                return Some((container, false));
            }
        }
    }

    let mut alignments = vec![];
    for cell in marker_row {
        let left = !cell.is_empty() && cell[0] == b':';
        let right = !cell.is_empty() && cell[cell.len() - 1] == b':';
        alignments.push(if left && right {
            TableAlignment::Center
        } else if left {
            TableAlignment::Left
        } else if right {
            TableAlignment::Right
        } else {
            TableAlignment::None
        });
    }

    let child = make_block(
        NodeValue::Table(alignments),
        parser.line_number,
    );
    let table = parser.arena.alloc(Node::new(RefCell::new(child)));
    container.append(table);

    let header = parser.add_child(table, NodeValue::TableRow(true));
    for header_str in header_row {
        let header_cell = parser.add_child(header, NodeValue::TableCell);
        header_cell.data.borrow_mut().content = header_str;
    }

    let offset = line.len() - 1 - parser.offset;
    parser.advance_offset(line, offset, false);

    Some((table, true))
}

fn try_opening_row<'a, 'o>(
    parser: &mut Parser<'a, 'o>,
    container: &'a AstNode<'a>,
    alignments: &[TableAlignment],
    line: &[u8],
) -> Option<(&'a AstNode<'a>, bool)> {
    if parser.blank {
        return None;
    }
    let this_row = row(&line[parser.first_nonspace..]).unwrap();
    let new_row = parser.add_child(
        container,
        NodeValue::TableRow(false),
    );

    let mut i = 0;
    while i < min(alignments.len(), this_row.len()) {
        let cell = parser.add_child(
            new_row,
            NodeValue::TableCell,
        );
        cell.data.borrow_mut().content = this_row[i].clone();
        i += 1;
    }

    while i < alignments.len() {
        parser.add_child(
            new_row,
            NodeValue::TableCell,
        );
        i += 1;
    }

    let offset = line.len() - 1 - parser.offset;
    parser.advance_offset(line, offset, false);

    Some((new_row, false))
}

fn row(string: &[u8]) -> Option<Vec<Vec<u8>>> {
    let len = string.len();
    let mut v = vec![];
    let mut offset = 0;

    if len > 0 && string[0] == b'|' {
        offset += 1;
    }

    loop {
        let cell_matched = scanners::table_cell(&string[offset..]).unwrap_or(0);
        let mut pipe_matched =
            scanners::table_cell_end(&string[offset + cell_matched..]).unwrap_or(0);

        if cell_matched > 0 || pipe_matched > 0 {
            let mut cell = unescape_pipes(&string[offset..offset + cell_matched]);
            trim(&mut cell);
            v.push(cell);
        }

        offset += cell_matched + pipe_matched;

        if pipe_matched == 0 {
            pipe_matched = scanners::table_row_end(&string[offset..]).unwrap_or(0);
            offset += pipe_matched;
        }

        if !((cell_matched > 0 || pipe_matched > 0) && offset < len) {
            break;
        }
    }

    if offset != len || v.is_empty() {
        None
    } else {
        Some(v)
    }
}

fn unescape_pipes(string: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(string.len());
    let mut escaping = false;

    for &c in string {
        // TODO
        if escaping {
            v.push(c);
            escaping = false;
        } else if c == b'\\' {
            escaping = true;
        } else {
            v.push(c);
        }
    }

    if escaping {
        v.push(b'\\');
    }

    v
}

pub fn matches(line: &[u8]) -> bool {
    row(line).is_some()
}
