// Reddit-flavored extensions
use nodes::{NodeValue, AstNode};
use parser::inlines::make_inline;
use regex::Regex;
use typed_arena::Arena;

// ZT: recursively call this to nest; enable multiple initial levels of nesting
pub fn process_glyphs<'a>(
    arena: &'a Arena<AstNode<'a>>,
    node: &'a AstNode<'a>,
    contents: &mut String
) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"([\^%]{1,10})(\([\w\s\(\)]+\)|\S+)").unwrap();
    }

    let owned_contents = contents.to_owned();
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

        let slice = &owned_contents[start..end].to_owned();
        let prefix: String = slice.chars().take(2).collect();
        let mut idx = 0;
        let mut wrapped = false;
        for c in prefix.into_bytes() {
            match c {
                b'^' | b'%' => idx += 1,
                b'(' => {
                    wrapped = true;
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
                inner_text.to_owned()
            )
        ));

        node.insert_after(inl);
        let remain = owned_contents[end..].to_string();
        inl.insert_after(make_inline(arena, NodeValue::Text(remain)));

        contents.truncate(start);
}
