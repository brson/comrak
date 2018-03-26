// Reddit-flavored extensions
use nodes::{NodeValue, AstNode};
use parser::inlines::make_inline;
use regex::bytes::Regex;
use typed_arena::Arena;

// ZT: recursively call this to nest; enable multiple initial levels of nesting
pub fn process_glyphs<'a>(
    arena: &'a Arena<AstNode<'a>>,
    node: &'a AstNode<'a>,
    contents: &mut Vec<u8>
) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([\^]{1,10})(\([\w\s\(\)]+\)|\S+)").unwrap();
    }

    let owned_contents =  contents.to_owned();
    let matched = RE.find(&owned_contents);
    let m = match matched {
        Some(m) => m,
        _ => return
    };
        let start = m.start();
        let end = m.end();

        let inl = make_inline(
            arena,
            NodeValue::Superscript
        );

        let mut slice = &owned_contents[start..end].to_owned();
        let prefix: Vec<u8> = slice[0..2].to_vec();
        let mut idx = 0;
        let mut wrapped = false;
        for c in prefix.iter() {
            match *c {
                b'^' => idx += 1,
                b'(' => {
                    let suffix = slice.iter().rev().next().unwrap();
                    if *suffix == b')' {
                        wrapped = true;
                    }
                    break
                }
                _ => break
            }
        }

        let inner_text;
        if wrapped {
            inner_text = &slice[idx+1..slice.len()-1];
        } else {
            inner_text = &slice[idx..slice.len()]
        }

        inl.append(make_inline(
            arena,
            NodeValue::Text(
                inner_text.to_vec()
            )
        ));

        node.insert_after(inl);
        let remain = &owned_contents[end..];
        inl.insert_after(make_inline(arena, NodeValue::Text(remain.to_vec())));

        contents.truncate(start);
}
