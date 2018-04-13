use arena_tree::Node;
use ctype::{ispunct, isspace};
use entity;
use nodes::{Ast, AstNode, NodeLink, NodeValue, NodeMedia};
use parser::{unwrap_into, unwrap_into_copy, AutolinkType, ComrakOptions, Reference};
use scanners;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::ptr;
use std::str;
use strings;
use typed_arena::Arena;
use unicode_categories::UnicodeCategories;

const MAXBACKTICKS: usize = 80;
const MAX_LINK_LABEL_LENGTH: usize = 1000;

pub struct Subject<'a: 'd, 'r, 'o, 'd, 'i> {
    pub arena: &'a Arena<AstNode<'a>>,
    options: &'o ComrakOptions,
    pub input: &'i [u8],
    pub pos: usize,
    pub refmap: &'r mut HashMap<Vec<u8>, Reference>,
    delimiter_arena: &'d Arena<Delimiter<'a, 'd>>,
    last_delimiter: Option<&'d Delimiter<'a, 'd>>,
    simple_superscript_openers: usize,
    brackets: Vec<Bracket<'a, 'd>>,
    pub backticks: [usize; MAXBACKTICKS + 1],
    pub scanned_for_backticks: bool,
    special_chars: Vec<bool>,
}

pub struct Delimiter<'a: 'd, 'd> {
    inl: &'a AstNode<'a>,
    length: usize,
    delim_char: u8,
    can_open: bool,
    can_close: bool,
    prev: Cell<Option<&'d Delimiter<'a, 'd>>>,
    next: Cell<Option<&'d Delimiter<'a, 'd>>>,
}

struct Bracket<'a: 'd, 'd> {
    previous_delimiter: Option<&'d Delimiter<'a, 'd>>,
    inl_text: &'a AstNode<'a>,
    position: usize,
    image: bool,
    active: bool,
    bracket_after: bool,
}

impl<'a, 'r, 'o, 'd, 'i> Subject<'a, 'r, 'o, 'd, 'i> {
    pub fn new(
        arena: &'a Arena<AstNode<'a>>,
        options: &'o ComrakOptions,
        input: &'i [u8],
        refmap: &'r mut HashMap<Vec<u8>, Reference>,
        delimiter_arena: &'d Arena<Delimiter<'a, 'd>>,
    ) -> Self {
        let mut s = Subject {
            arena: arena,
            options: options,
            input: input,
            pos: 0,
            refmap: refmap,
            delimiter_arena: delimiter_arena,
            last_delimiter: None,
            simple_superscript_openers: 0,
            brackets: vec![],
            backticks: [0; MAXBACKTICKS + 1],
            scanned_for_backticks: false,
            special_chars: vec![],
        };
        s.special_chars.extend_from_slice(&[false; 256]);
        for &c in &[
            b'\n', b'\r', b'_', b'*', b'"', b'`', b'\\', b'&', b'<', b'[', b']', b'!', b'>'
        ] {
            s.special_chars[c as usize] = true;
        }
        if options.ext_strikethrough {
            s.special_chars[b'~' as usize] = true;
        }
        if options.ext_superscript {
            s.special_chars[b'^' as usize] = true;
        }
        if options.ext_reddit_quirks {
                assert!(!(options.ext_superscript && options.ext_reddit_quirks),
                        "ext_superscript and ext_reddit_quirks are incompatible");
            s.special_chars[b'^' as usize] = true;
            s.special_chars[b')' as usize] = true;
        }
        s
    }

    pub fn pop_bracket(&mut self) -> bool {
        self.brackets.pop().is_some()
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    pub fn parse_inline(&mut self, node: &'a AstNode<'a>) -> bool {
        if self.handle_reddit_simple_superscript_closer(node) {
            return true;
        }

        let new_inl: Option<&'a AstNode<'a>>;
        let c = match self.peek_char() {
            None => return false,
            Some(ch) => *ch as char,
        };

        match c {
            '\0' => return false,
            '\r' | '\n' => new_inl = Some(self.handle_newline()),
            '`' => new_inl = Some(self.handle_backticks()),
            '\\' => new_inl = Some(self.handle_backslash()),
            '&' => new_inl = Some(self.handle_entity()),
            '<' => new_inl = Some(self.handle_pointy_brace()),
            '>' => {
                self.pos += 1;
                if self.peek_char() == Some(&(b'!')) {
                    self.pos += 1;
                    new_inl = Some(self.handle_spoiler(true));
                } else {
                    new_inl = Some(make_inline(self.arena, NodeValue::Text(b">".to_vec())));
                }
            }
            '*' | '_' | '\'' | '"' => new_inl = Some(self.handle_delim(c as u8)),
            // TODO: smart characters. Eh.
            //'-' => new_inl => Some(self.handle_hyphen()),
            //'.' => new_inl => Some(self.handle_period()),
            '[' => {
                self.pos += 1;
                let inl = make_inline(self.arena, NodeValue::Text(b"[".to_vec()));
                new_inl = Some(inl);
                self.push_bracket(false, inl);
            }
            ']' => new_inl = self.handle_close_bracket(),
            '!' => {
                self.pos += 1;
                if self.peek_char() == Some(&(b'[')) && self.peek_char_n(1) != Some(&(b'^')) {
                    self.pos += 1;
                    let inl = make_inline(self.arena, NodeValue::Text(b"![".to_vec()));
                    new_inl = Some(inl);
                    self.push_bracket(true, inl);
                } else if self.peek_char() == Some(&(b'<')) {
                    self.pos += 1;
                    new_inl = Some(self.handle_spoiler(false));
                } else {
                    new_inl = Some(make_inline(self.arena, NodeValue::Text(b"!".to_vec())));
                }
            }
            _ => if self.options.ext_strikethrough && c == '~' {
                // Reddit quirk - strikethrough requires at least two twiddles
                if self.options.ext_reddit_quirks && self.peek_char_n(1) != Some(&(b'~')) {
                    self.pos += 1;
                    new_inl = Some(make_inline(self.arena, NodeValue::Text(b"~".to_vec())));
                } else {
                    new_inl = Some(self.handle_delim(b'~'));
                }
            } else if self.options.ext_superscript && c == '^' {
                new_inl = Some(self.handle_delim(b'^'));
            } else if self.options.ext_reddit_quirks && c == '^' {
                new_inl = Some(self.handle_reddit_superscript_opener());
            } else if self.options.ext_reddit_quirks && c == ')' {
                new_inl = Some(self.handle_reddit_superscript_closer());
            } else {
                let endpos = self.find_special_char();
                let mut contents = self.input[self.pos..endpos].to_vec();
                self.pos = endpos;

                if self.peek_char()
                    .map_or(false, |&c| strings::is_line_end_char(c))
                {
                    strings::rtrim(&mut contents);
                }

                new_inl = Some(make_inline(self.arena, NodeValue::Text(contents)));
            },
        }

        if let Some(inl) = new_inl {
            node.append(inl);
        }

        true
    }

    fn del_ref_eq(lhs: Option<&'d Delimiter<'a, 'd>>, rhs: Option<&'d Delimiter<'a, 'd>>) -> bool {
        match (lhs, rhs) {
            (None, None) => true,
            (Some(l), Some(r)) => ptr::eq(l, r),
            _ => false,
        }
    }

    // After parsing a block (and sometimes during), this function traverses the
    // stack of `Delimiters`, tokens ("*", "_", etc.) that may delimit regions
    // of text for special rendering: emphasis, strong, superscript,
    // spoilertext; looking for pairs of opening and closing delimiters,
    // with the goal of placing the intervening nodes into new emphasis,
    // etc AST nodes.
    //
    // The term stack here is a bit of a misnomer, as the `Delimiters` actually
    // form a doubly-linked list. Items are pushed onto the stack during parsing,
    // but during post-processing are removed from arbitrary locations.
    //
    // The `Delimiter` contains references AST `Text` nodes, which are also
    // linked into the AST as siblings in the order they are parsed. This
    // function doesn't know a-priori which ones are markdown syntax and which
    // are just text: candidate delimiters that match have their nodes removed
    // from the AST, as they are markdown, and their intervening siblings
    // lowered into a new AST parent node via the `insert_emph` function;
    // candidate delimiters that don't match are left in the tree.
    //
    // The basic algorithm is to start at the bottom of the stack, walk upwards
    // looking for closing delimiters, and from each closing delimiter walk back
    // down the stack looking for its matching opening delimiter. This traversal
    // favors the smallest matching leftmost pairs, e.g.
    //
    //   _a *b c_ d* e_
    //    ~~~~~~
    //
    // (The emphasis region is wavy-underlined)
    //
    // All of the `_` and `*` tokens are scanned as candidates, but only the
    // region "a *b c" is lowered into an `Emph` node; the other candidate
    // delimiters are all actually text.
    //
    // And in
    //
    //   _a _b c_
    //       ~~~
    //
    // "b c" is the emphasis region, not "a _b c".
    //
    // Note that Delimiters are matched by comparing their `delim_char`, which
    // is simply a value used to compare opening and closing delimiters - the
    // actual text value of the scanned token can theoretically be different.
    //
    // There's some additional trickiness in the logic because "_", "__", and
    // "___" (and etc. etc.) all share the same delim_char, but represent
    // different emphasis. Note also that "_"- and "*"-delimited regions have
    // complex rules for which can be opening and/or closing delimiters,
    // determined in `scan_delims`.
    #[cfg_attr(feature = "flamegraphs", flame)]
    pub fn process_emphasis(&mut self, stack_bottom: Option<&'d Delimiter<'a, 'd>>) {
        let mut closer = self.last_delimiter;

        // This array is an important optimization that prevents searching down
        // the stack for openers we've previously searched for and know don't
        // exist, preventing exponential blowup on pathological cases.
        // TODO: It _seems_ like these should be initialized for the other
        // delimiters too.
        let mut openers_bottom: [[Option<&'d Delimiter<'a, 'd>>; 128]; 3] = [[None; 128]; 3];
        for i in &mut openers_bottom {
            i['*' as usize] = stack_bottom;
            i['_' as usize] = stack_bottom;
            i['\'' as usize] = stack_bottom;
            i['"' as usize] = stack_bottom;
        }

        // This is traversing the stack from the top to the bottom, setting `closer` to
        // the delimiter directly above `stack_bottom`. In the case where we are processing
        // emphasis on an entire block, `stack_bottom` is `None`, so `closer` references
        // the very bottom of the stack.
        while closer.is_some() && !Self::del_ref_eq(closer.unwrap().prev.get(), stack_bottom) {
            closer = closer.unwrap().prev.get();
        }

        while closer.is_some() {
            // Curiously, ' and " are never actually pushed on the delimiter
            // stack: handle_delim is short-circuited for both, so there seems
            // to be lots of dead code related to quote delimiters.
            debug_assert!(closer.unwrap().delim_char != b'\'');
            debug_assert!(closer.unwrap().delim_char != b'"');

            if closer.unwrap().can_close {
                // Each time through the outer `closer` loop we reset the opener
                // to the element below the closer, and search down the stack
                // for a matching opener.

                let mut opener = closer.unwrap().prev.get();
                let mut opener_found = false;

                // Here's where we find the opener by searching down the stack,
                // looking for matching delims with the `can_open` flag.
                // On any invocation, on the first time through the outer
                // `closer` loop, this inner `opener` search doesn't succeed:
                // when processing a full block, `opener` starts out `None`;
                // when processing emphasis otherwise, opener will be equal to
                // `stack_bottom`.
                //
                // This search short-circuits for openers we've previously
                // failed to find, avoiding repeatedly rescanning the bottom of
                // the stack, using the openers_bottom array.
                while opener.is_some() && !Self::del_ref_eq(opener, stack_bottom)
                    && !Self::del_ref_eq(
                        opener,
                        openers_bottom[closer.unwrap().length % 3]
                            [closer.unwrap().delim_char as usize],
                    ) {
                    if opener.unwrap().can_open
                        && opener.unwrap().delim_char == closer.unwrap().delim_char
                    {
                        // This is a bit convoluted; see points 9 and 10 here:
                        // http://spec.commonmark.org/0.28/#can-open-emphasis.
                        // This is to aid processing of runs like this:
                        // “***hello*there**” or “***hello**there*”. In this
                        // case, the middle delimiter can both open and close
                        // emphasis; when trying to find an opening delimiter
                        // that matches the last ** or *, we need to skip it,
                        // and this algorithm ensures we do. (The sum of the
                        // lengths are a multiple of 3.)
                        let odd_match = (closer.unwrap().can_open || opener.unwrap().can_close)
                            && ((opener.unwrap().length + closer.unwrap().length) % 3 == 0);
                        if !odd_match {
                            opener_found = true;
                            break;
                        }
                    }
                    opener = opener.unwrap().prev.get();
                }

                let old_closer = closer;

                // There's a case here for every possible delimiter. If we found
                // a matching opening delimiter for our closing delimiter, they
                // both get passed.
                if closer.unwrap().delim_char == b'*' || closer.unwrap().delim_char == b'_'
                    || (self.options.ext_strikethrough && closer.unwrap().delim_char == b'~')
                    || (self.options.ext_superscript && closer.unwrap().delim_char == b'^')
                    || (self.options.ext_reddit_quirks && closer.unwrap().delim_char == b'^')
                    || (self.options.ext_reddit_quirks && closer.unwrap().delim_char == b'.')
                    || (self.options.ext_spoilertext && closer.unwrap().delim_char == b'!')
                {
                    if opener_found {
                        // Finally, here's the happy case where the delimiters
                        // match and they are inserted. We get a new closer
                        // delimiter and go around the loop again.
                        //
                        // Note that for "***" and "___" delimiters of length
                        // greater than 2, insert_emph will create a `Strong`
                        // node (i.e. "**"), then _truncate_ the delimiters in
                        // place, turning them into e.g. "*" delimiters, and
                        // hand us back the same mutated closer to be matched
                        // again.
                        //
                        // In general though the closer will be the next
                        // delimiter up the stack.
                        closer = self.insert_emph(opener.unwrap(), closer.unwrap());
                    } else {
                        // When no matching opener is found we move the closer
                        // up the stack, do some bookkeeping with old_closer
                        // (below), try again.
                        closer = closer.unwrap().next.get();
                    }
                } else if closer.unwrap().delim_char == b'\'' {
                    debug_assert!(false, "NB: dead code (brson)");
                    *closer
                        .unwrap()
                        .inl
                        .data
                        .borrow_mut()
                        .value
                        .text_mut()
                        .unwrap() = "’".to_string().into_bytes();
                    if opener_found {
                        *opener
                            .unwrap()
                            .inl
                            .data
                            .borrow_mut()
                            .value
                            .text_mut()
                            .unwrap() = "‘".to_string().into_bytes();
                    }
                    closer = closer.unwrap().next.get();
                } else if closer.unwrap().delim_char == b'"' {
                    debug_assert!(false, "NB: dead code (brson)");
                    *closer
                        .unwrap()
                        .inl
                        .data
                        .borrow_mut()
                        .value
                        .text_mut()
                        .unwrap() = "”".to_string().into_bytes();
                    if opener_found {
                        *opener
                            .unwrap()
                            .inl
                            .data
                            .borrow_mut()
                            .value
                            .text_mut()
                            .unwrap() = "“".to_string().into_bytes();
                    }
                    closer = closer.unwrap().next.get();
                }

                // If the search for an opener was unsuccessful, then record
                // the position the search started at in the `openers_bottom`
                // so that the `opener` search can avoid looking for this
                // same opener at the bottom of the stack later.
                if !opener_found {
                    let ix = old_closer.unwrap().length % 3;
                    openers_bottom[ix][old_closer.unwrap().delim_char as usize] =
                        old_closer.unwrap().prev.get();

                    // Now that we've failed the `opener` search starting from
                    // `old_closer`, future opener searches will be searching it
                    // for openers - if `old_closer` can't be used as an opener
                    // then we know it's just text - remove it from the
                    // delimiter stack, leaving it in the AST as text
                    if !old_closer.unwrap().can_open {
                        self.remove_delimiter(old_closer.unwrap());
                    }
                }
            } else {
                // Closer is !can_close. Move up the stack
                closer = closer.unwrap().next.get();
            }
        }

        // At this point the entire delimiter stack from `stack_bottom` up has
        // been scanned for matches, everything left is just text. Pop it all
        // off.
        while self.last_delimiter.is_some() && !Self::del_ref_eq(self.last_delimiter, stack_bottom)
        {
            let last_del = self.last_delimiter.unwrap();
            self.remove_delimiter(last_del);
        }
    }

    fn remove_delimiter(&mut self, delimiter: &'d Delimiter<'a, 'd>) {
        if delimiter.next.get().is_none() {
            assert!(ptr::eq(delimiter, self.last_delimiter.unwrap()));
            self.last_delimiter = delimiter.prev.get();
        } else {
            delimiter.next.get().unwrap().prev.set(delimiter.prev.get());
        }
        if delimiter.prev.get().is_some() {
            delimiter.prev.get().unwrap().next.set(delimiter.next.get());
        }
    }

    #[inline]
    pub fn eof(&self) -> bool {
        self.pos >= self.input.len()
    }

    #[inline]
    pub fn peek_char(&self) -> Option<&u8> {
        self.peek_char_n(0)
    }

    #[inline]
    fn peek_char_n(&self, n: usize) -> Option<&u8> {
        if self.pos + n >= self.input.len() {
            None
        } else {
            let c = &self.input[self.pos + n];
            assert!(*c > 0);
            Some(c)
        }
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    pub fn find_special_char(&self) -> usize {
        for n in self.pos..self.input.len() {
            if self.special_chars[self.input[n] as usize] {
                return n;
            }
        }

        self.input.len()
    }

    pub fn handle_newline(&mut self) -> &'a AstNode<'a> {
        let nlpos = self.pos;
        if self.input[self.pos] == b'\r' {
            self.pos += 1;
        }
        if self.input[self.pos] == b'\n' {
            self.pos += 1;
        }
        self.skip_spaces();
        if nlpos > 1 && self.input[nlpos - 1] == b' ' && self.input[nlpos - 2] == b' ' {
            make_inline(self.arena, NodeValue::LineBreak)
        } else {
            make_inline(self.arena, NodeValue::SoftBreak)
        }
    }

    pub fn take_while(&mut self, c: u8) -> usize {
        let start_pos = self.pos;
        while self.peek_char() == Some(&c) {
            self.pos += 1;
        }
        self.pos - start_pos
    }

    pub fn scan_to_closing_backtick(&mut self, openticklength: usize) -> Option<usize> {
        if openticklength > MAXBACKTICKS {
            return None;
        }

        if self.scanned_for_backticks && self.backticks[openticklength] <= self.pos {
            return None;
        }

        loop {
            while self.peek_char().map_or(false, |&c| c != b'`') {
                self.pos += 1;
            }
            if self.pos >= self.input.len() {
                self.scanned_for_backticks = true;
                return None;
            }
            let numticks = self.take_while(b'`');
            if numticks <= MAXBACKTICKS {
                self.backticks[numticks] = self.pos - numticks;
            }
            if numticks == openticklength {
                return Some(self.pos);
            }
        }
    }

    pub fn handle_backticks(&mut self) -> &'a AstNode<'a> {
        let openticks = self.take_while(b'`');
        let startpos = self.pos;
        let endpos = self.scan_to_closing_backtick(openticks);

        match endpos {
            None => {
                self.pos = startpos;
                make_inline(self.arena, NodeValue::Text(vec![b'`'; openticks]))
            }
            Some(endpos) => {
                let mut buf = &self.input[startpos..endpos - openticks];
                buf = strings::trim_slice(buf);
                let buf = strings::normalize_whitespace(buf);
                make_inline(self.arena, NodeValue::Code(buf))
            }
        }
    }

    pub fn skip_spaces(&mut self) -> bool {
        let mut skipped = false;
        while self.peek_char().map_or(false, |&c| c == b' ' || c == b'\t') {
            self.pos += 1;
            skipped = true;
        }
        skipped
    }

    pub fn handle_delim(&mut self, c: u8) -> &'a AstNode<'a> {
        let (numdelims, can_open, can_close) = self.scan_delims(c);

        let contents = self.input[self.pos - numdelims..self.pos].to_vec();
        let inl = make_inline(self.arena, NodeValue::Text(contents));

        if (can_open || can_close) && c != b'\'' && c != b'"' {
            self.push_delimiter(c, can_open, can_close, inl);
        }

        inl
    }

    pub fn handle_spoiler(&mut self, open: bool) -> &'a AstNode<'a> {
        let inl;
        let (can_open, can_close) = if open == true {
            inl = make_inline(self.arena, NodeValue::Text(b">!".to_vec()));
            (true, false)
        } else {
            inl = make_inline(self.arena, NodeValue::Text(b"!<".to_vec()));
            (false, true)
        };

        self.push_delimiter(b'!', can_open, can_close, inl);

        inl
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    pub fn scan_delims(&mut self, c: u8) -> (usize, bool, bool) {
        let before_char = if self.pos == 0 {
            '\n'
        } else {
            let mut before_char_pos = self.pos - 1;
            while before_char_pos > 0 && self.input[before_char_pos] >> 6 == 2 {
                before_char_pos -= 1;
            }
            unsafe { str::from_utf8_unchecked(&self.input[before_char_pos..self.pos]) }
                .chars()
                .next()
                .unwrap()
        };

        let mut numdelims = 0;
        if c == b'\'' || c == b'"' {
            numdelims += 1;
            self.pos += 1;
        } else {
            while self.peek_char() == Some(&c) {
                numdelims += 1;
                self.pos += 1;
            }
        }

        let after_char = if self.eof() {
            '\n'
        } else {
            unsafe { str::from_utf8_unchecked(&self.input[self.pos..]) }
                .chars()
                .next()
                .unwrap()
        };

        // HACK: For "simple" superscript parsing, e.g. `^_foo_` emphasis can be
        // left-flanking following a caret. It's possible this could allow bad
        // emphasis where process_emphasis doesn't interpret the caret as
        // superscript but does interpret the following emphasis.
        let is_superscript_caret = |ch| ch == '^' && self.simple_superscript_openers > 0;

        let left_flanking = numdelims > 0 && !after_char.is_whitespace()
            && !(after_char.is_punctuation() && !before_char.is_whitespace()
                && !before_char.is_punctuation());
        let right_flanking = numdelims > 0 && !before_char.is_whitespace()
            && !(before_char.is_punctuation() && !after_char.is_whitespace()
                && !after_char.is_punctuation());

        if c == b'_' {
            (
                numdelims,
                left_flanking && (!right_flanking || before_char.is_punctuation() || is_superscript_caret(before_char)),
                right_flanking && (!left_flanking || after_char.is_punctuation()),
            )
        } else if c == b'\'' || c == b'"' {
            (numdelims, left_flanking && !right_flanking, right_flanking)
        } else {
            (numdelims, left_flanking, right_flanking)
        }
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    pub fn push_delimiter(&mut self, c: u8, can_open: bool, can_close: bool, inl: &'a AstNode<'a>) {
        let d = self.delimiter_arena.alloc(Delimiter {
            prev: Cell::new(self.last_delimiter),
            next: Cell::new(None),
            inl: inl,
            length: inl.data.borrow().value.text().unwrap().len(),
            delim_char: c,
            can_open: can_open,
            can_close: can_close,
        });
        if d.prev.get().is_some() {
            d.prev.get().unwrap().next.set(Some(d));
        }
        self.last_delimiter = Some(d);
    }

    // Create a new emphasis node, move all the nodes between `opener`
    // and `closer` into it, and insert it into the AST.
    //
    // As a side-effect, handle long "***" and "___" nodes by truncating them in
    // place to be re-matched by `process_emphasis`.
    #[cfg_attr(feature = "flamegraphs", flame)]
    pub fn insert_emph(
        &mut self,
        opener: &'d Delimiter<'a, 'd>,
        closer: &'d Delimiter<'a, 'd>,
    ) -> Option<&'d Delimiter<'a, 'd>> {
        let opener_char = opener.inl.data.borrow().value.text().unwrap()[0];
        let mut opener_num_chars = opener.inl.data.borrow().value.text().unwrap().len();
        let mut closer_num_chars = closer.inl.data.borrow().value.text().unwrap().len();

        // Decide how many "*"s or "_" to truncate from the delimiters, and thus
        // whether to insert an `Emph` (1) or `Strong` (2) node.
        let use_delims = if closer_num_chars >= 2 && opener_num_chars >= 2 {
            2
        } else {
            1
        };

        let is_strikethrough = self.options.ext_strikethrough && opener_char == b'~';
        let is_reddit_superscript = self.options.ext_reddit_quirks && opener_char == b'^';

        // The number of chars (bytes) we'll have truncate the delimiter nodes
        // to. At 0 bytes they'll be dropped from the AST and the delimiter
        // stack.
        if is_strikethrough || is_reddit_superscript {
            opener_num_chars = 0;
            closer_num_chars = 0;
        } else {
            opener_num_chars -= use_delims;
            closer_num_chars -= use_delims;
        }

        opener
            .inl
            .data
            .borrow_mut()
            .value
            .text_mut()
            .unwrap()
            .truncate(opener_num_chars);
        closer
            .inl
            .data
            .borrow_mut()
            .value
            .text_mut()
            .unwrap()
            .truncate(closer_num_chars);

        // Remove all the candidate delimiters from between the opener and the
        // closer. None of them are matched pairs. They've been scanned already.
        let mut delim = closer.prev.get();
        while delim.is_some() && !Self::del_ref_eq(delim, Some(opener)) {
            self.remove_delimiter(delim.unwrap());
            delim = delim.unwrap().prev.get();
        }

        let emph = make_inline(
            self.arena,
            if self.options.ext_strikethrough && opener_char == b'~' {
                NodeValue::Strikethrough
            } else if self.options.ext_superscript && opener_char == b'^' {
                NodeValue::Superscript
            } else if self.options.ext_reddit_quirks && opener_char == b'^' {
                NodeValue::Superscript
            } else if self.options.ext_spoilertext && opener_char == b'>' {
                NodeValue::SpoilerText
            } else if use_delims == 1 {
                NodeValue::Emph
            } else {
                NodeValue::Strong
            },
        );

        // Drop all the interior AST nodes into the emphasis node
        // and then insert the emphasis node
        let mut tmp = opener.inl.next_sibling().unwrap();
        while !tmp.same_node(closer.inl) {
            let next = tmp.next_sibling();
            emph.append(tmp);
            if let Some(n) = next {
                tmp = n;
            } else {
                break;
            }
        }
        opener.inl.insert_after(emph);

        // Drop the delimiters and return the next closer to process

        if opener_num_chars == 0 {
            opener.inl.detach();
            self.remove_delimiter(opener);
        }

        if closer_num_chars == 0 {
            closer.inl.detach();
            self.remove_delimiter(closer);
            closer.next.get()
        } else {
            Some(closer)
        }
    }

    pub fn handle_backslash(&mut self) -> &'a AstNode<'a> {
        self.pos += 1;
        if self.peek_char().map_or(false, |&c| ispunct(c)) {
            self.pos += 1;
            // TODO
            make_inline(self.arena, NodeValue::Text(vec![self.input[self.pos - 1]]))
        } else if !self.eof() && self.skip_line_end() {
            make_inline(self.arena, NodeValue::LineBreak)
        } else {
            make_inline(self.arena, NodeValue::Text(b"\\".to_vec()))
        }
    }

    pub fn skip_line_end(&mut self) -> bool {
        let old_pos = self.pos;
        if self.peek_char() == Some(&(b'\r')) {
            self.pos += 1;
        }
        if self.peek_char() == Some(&(b'\n')) {
            self.pos += 1;
        }
        self.pos > old_pos || self.eof()
    }

    pub fn handle_entity(&mut self) -> &'a AstNode<'a> {
        self.pos += 1;

        match entity::unescape(&self.input[self.pos..]) {
            None => make_inline(self.arena, NodeValue::Text(b"&".to_vec())),
            Some((entity, len)) => {
                self.pos += len;
                make_inline(self.arena, NodeValue::Text(entity))
            }
        }
    }

    pub fn handle_pointy_brace(&mut self) -> &'a AstNode<'a> {
        self.pos += 1;

        if let Some(matchlen) = scanners::autolink_uri(&self.input[self.pos..]) {
            let inl = make_autolink(
                self.arena,
                &self.input[self.pos..self.pos + matchlen - 1],
                AutolinkType::URI,
            );
            self.pos += matchlen;
            return inl;
        }

        if let Some(matchlen) = scanners::autolink_email(&self.input[self.pos..]) {
            let inl = make_autolink(
                self.arena,
                &self.input[self.pos..self.pos + matchlen - 1],
                AutolinkType::Email,
            );
            self.pos += matchlen;
            return inl;
        }

        if let Some(matchlen) = scanners::html_tag(&self.input[self.pos..]) {
            let contents = &self.input[self.pos - 1..self.pos + matchlen];
            let inl = make_inline(self.arena, NodeValue::Text(contents.to_vec()));
            self.pos += matchlen;
            return inl;
        }

        make_inline(self.arena, NodeValue::Text(b"<".to_vec()))
    }

    pub fn push_bracket(&mut self, image: bool, inl_text: &'a AstNode<'a>) {
        let len = self.brackets.len();
        if len > 0 {
            self.brackets[len - 1].bracket_after = true;
        }
        self.brackets.push(Bracket {
            previous_delimiter: self.last_delimiter,
            inl_text: inl_text,
            position: self.pos,
            image: image,
            active: true,
            bracket_after: false,
        });
    }

    pub fn handle_close_bracket(&mut self) -> Option<&'a AstNode<'a>> {
        self.pos += 1;
        let initial_pos = self.pos;

        let brackets_len = self.brackets.len();
        if brackets_len == 0 {
            return Some(make_inline(self.arena, NodeValue::Text(b"]".to_vec())));
        }

        if !self.brackets[brackets_len - 1].active {
            self.brackets.pop();
            return Some(make_inline(self.arena, NodeValue::Text(b"]".to_vec())));
        }

        let is_image = self.brackets[brackets_len - 1].image;
        let after_link_text_pos = self.pos;

        let mut sps = 0;
        let mut n = 0;
        if self.peek_char() == Some(&(b'(')) && {
            sps = scanners::spacechars(&self.input[self.pos + 1..]).unwrap_or(0);
            unwrap_into(
                manual_scan_link_url(&self.input[self.pos + 1 + sps..],
                                     self.options.ext_reddit_quirks),
                &mut n,
            )
        } {
            let starturl = self.pos + 1 + sps;
            let endurl = starturl + n;
            let starttitle = endurl + scanners::spacechars(&self.input[endurl..]).unwrap_or(0);
            let endtitle = if starttitle == endurl {
                starttitle
            } else {
                starttitle + scanners::link_title(&self.input[starttitle..]).unwrap_or(0)
            };
            let endall = endtitle + scanners::spacechars(&self.input[endtitle..]).unwrap_or(0);

            if endall < self.input.len() && self.input[endall] == b')' {
                self.pos = endall + 1;
                let url = strings::clean_url(&self.input[starturl..endurl]);
                let title = strings::clean_title(&self.input[starttitle..endtitle]);
                self.close_bracket_match(is_image, url, title);
                return None;
            } else {
                self.pos = after_link_text_pos;
            }
        }

        let (mut lab, mut found_label) = match self.link_label() {
            Some(lab) => (lab.to_vec(), true),
            None => (vec![], false),
        };

        if !found_label {
            self.pos = initial_pos;
        }

        if (!found_label || lab.is_empty()) && !self.brackets[brackets_len - 1].bracket_after {
            lab = self.input[self.brackets[brackets_len - 1].position..initial_pos - 1].to_vec();
            found_label = true;
        }

        let reff: Option<Reference> = if found_label {
            lab = strings::normalize_label(&lab);
            self.refmap.get(&lab).cloned()
        } else {
            None
        };

        if let Some(reff) = reff {
            self.close_bracket_match(is_image, reff.url.clone(), reff.title.clone());
            return None;
        }

        let mut text: Option<Vec<u8>> = None;
        if self.options.ext_footnotes
            && match self.brackets[brackets_len - 1].inl_text.next_sibling() {
                Some(n) => {
                    text = n.data.borrow().value.text().cloned();
                    text.is_some() && n.next_sibling().is_none()
                }
                _ => false,
            } {
            let text = text.unwrap();
            if text.len() > 1 && text[0] == b'^' {
                let inl = make_inline(self.arena, NodeValue::FootnoteReference(text[1..].to_vec()));
                self.brackets[brackets_len - 1].inl_text.insert_before(inl);
                self.brackets[brackets_len - 1]
                    .inl_text
                    .next_sibling()
                    .unwrap()
                    .detach();
                self.brackets[brackets_len - 1].inl_text.detach();
                let previous_delimiter = self.brackets[brackets_len - 1].previous_delimiter;
                self.process_emphasis(previous_delimiter);
                self.brackets.pop();
                return None;
            }
        }

        self.brackets.pop();
        self.pos = initial_pos;
        Some(make_inline(self.arena, NodeValue::Text(b"]".to_vec())))
    }

    pub fn close_bracket_match(&mut self, is_image: bool, url: Vec<u8>, title: Vec<u8>) {
        // Only accept certain url schemes, particularly reject javascript:
        let good_url = strings::validate_url_scheme(&url);

        // Reddit extension - images are actually 'rich text media' and urls are
        // actually base36 hashes
        use regex::bytes::Regex;
        lazy_static! {
            static ref BASE36: Regex = Regex::new(r"^[[:alnum:]]+$").unwrap();
        }
        let good_url = if !is_image { good_url } else { BASE36.is_match(&url) };

        let nl = NodeLink {
            url: url,
            title: title,
            l: false,
        };
        let inl = make_inline(
            self.arena,
            if is_image {
                if !self.options.rtjson {
                    NodeValue::Image(NodeLink{
                        url: nl.url,
                        title: nl.title,
                        l: false,
                    })
                } else {
                    NodeValue::Media(NodeMedia{
                        e: b"".to_vec(),
                        url: nl.url,
                        title: nl.title,
                    })
                }
            } else {
                NodeValue::Link(nl)
            },
        );

        let mut brackets_len = self.brackets.len();
        if good_url {
            // If it's a good url then insert the new link node before the opening
            // bracket, move all the link text to a child of the link node,
            // then (further down) detach the opening bracket from the AST.
            self.brackets[brackets_len - 1].inl_text.insert_before(inl);
            let mut tmpch = self.brackets[brackets_len - 1].inl_text.next_sibling();
            while let Some(tmp) = tmpch {
                tmpch = tmp.next_sibling();
                inl.append(tmp);
            }
        } else {
            // If it's a bogus URL then we don't have to do anything special,
            // the `detach` call below will remove the opening bracket from the
            // AST, the remaining link text will still exist, and the url and
            // title will just evaporate.
        }
        self.brackets[brackets_len - 1].inl_text.detach();
        let previous_delimiter = self.brackets[brackets_len - 1].previous_delimiter;
        self.process_emphasis(previous_delimiter);
        self.brackets.pop();
        brackets_len -= 1;

        if !is_image {
            let mut i = brackets_len as i32 - 1;
            while i >= 0 {
                if !self.brackets[i as usize].image {
                    if !self.brackets[i as usize].active {
                        break;
                    } else {
                        self.brackets[i as usize].active = false;
                    }
                }
                i -= 1;
            }
        }
    }

    pub fn link_label(&mut self) -> Option<&[u8]> {
        let startpos = self.pos;

        if self.peek_char() != Some(&(b'[')) {
            return None;
        }

        self.pos += 1;

        let mut length = 0;
        let mut c = 0;
        while unwrap_into_copy(self.peek_char(), &mut c) && c != b'[' && c != b']' {
            if c == b'\\' {
                self.pos += 1;
                length += 1;
                if self.peek_char().map_or(false, |&c| ispunct(c)) {
                    self.pos += 1;
                    length += 1;
                }
            } else {
                self.pos += 1;
                length += 1;
            }
            if length > MAX_LINK_LABEL_LENGTH {
                self.pos = startpos;
                return None;
            }
        }

        if c == b']' {
            let raw_label = strings::trim_slice(&self.input[startpos + 1..self.pos]);
            self.pos += 1;
            Some(raw_label)
        } else {
            self.pos = startpos;
            None
        }
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    fn handle_reddit_superscript_opener(&mut self) -> &'a AstNode<'a> {
        // Reddit handles superscript differently from comrak, With
        // either `^nonwhitespace` or `^(any inlines)`. snudown's
        // superscript implementation is a bit notorious for it's weird
        // and accidental behavior, and snoomark doesn't try to emulate
        // it perfectly, but rather to do something reasonable.

        let new_inl: &'a AstNode<'a>;

        self.pos += 1;

        let next_char = self.peek_char().cloned();

        let next_is_whitespace_or_end = next_char.map(|c| {
            strings::is_space_or_tab(c) || strings::is_line_end_char(c)
        }).unwrap_or(true);

        if next_is_whitespace_or_end {
            // Just a text caret
            new_inl = make_inline(self.arena, NodeValue::Text(b"^".to_vec()));
        } else if next_char == Some(b'(') {
            // The easy syntax, `^(...)`, where we can just push the
            // opening delimiter, wait for the closing delimiter, and
            // let `postprocess_emphasis` sort it out later.
            self.pos += 1;
            let del = make_inline(self.arena, NodeValue::Text(b"^(".to_vec()));
            self.push_delimiter(b'^', true, false, &del);
            new_inl = del;
        } else {
            // The completely baffling case, `^...`, here called "simple
            // superscript", where the closing delimiter is whitespace. We don't
            // want to be doing a lot of parsing work every time we encounter
            // whitespace like we do with other delimiters, so we're only going
            // to do that when we know we're scanning a simple superscript.
            //
            // Simple superscript can be nested arbitrarily deep, as long as no
            // whitespace is encountered, and the nesting is completely unwound
            // by a single whitespace. e.g.
            //
            //     "^a^^b^^^c "
            //
            // is 6 levels of superscript, terminated by a space.
            //
            // The good news is that since we don't need to match delimiters we
            // can know definitely that we're parsing superscript once we've
            // reached this point, and start scanning for whitespace.
            //
            // Aside: This turns out not to be true in cases like:
            //
            //    ^^(^foo)
            //
            // (Under the current parser that third caret doesn't end up being
            // superscript and is inserted as text).
            //
            // So what we're going to do is push another kind of delimiter
            // opener onto the delimiter stack, bump the
            // `simple_superscript_openers` counter, and turn whitespace into
            // "special" chars so that `find_special_char` will stop at
            // whitespace, where we'll dump a `simple_superscript_openers`
            // number of matching closing delimiters onto the delimiter stack
            // when we see whitespace, newlines, or eof.
            //
            // We use '.' as the delim char instead of '^' because this the
            // closing delimiter is diferent from the parenthesized superscript,
            // which already uses '^'.
            //
            // Parenthesized superscript can't be nested inside simple
            // superscript - it's just too much trouble to parse.
            //
            // FIXME: In hindsight, this may not be the simplest or most
            // intuitive algorithm. It might be better to: when hitting a `^`,
            // scan forward to the next space, and lower all the intervening
            // text to a new kind of AST node, SimpleSuperscript, which is
            // marked as `contains_inlines`, to be further processed by the
            // block parser's inline expansion. The results would be similar in
            // common cases, and probably more closely resemble what snudown
            // does.

            let del = make_inline(self.arena, NodeValue::Text(b"^".to_vec()));
            self.push_delimiter(b'.', true, false, &del);
            new_inl = del;

            self.simple_superscript_openers += 1;
            self.special_chars[b' ' as usize] = true;
            self.special_chars[b'\t' as usize] = true;
        }

        new_inl
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    fn handle_reddit_superscript_closer(&mut self) -> &'a AstNode<'a> {
        // The closing delimiter for parenthesized superscript
        self.pos += 1;
        let del = make_inline(self.arena, NodeValue::Text(b")".to_vec()));
        self.push_delimiter(b'^', false, true, &del);
        del
    }

    #[cfg_attr(feature = "flamegraphs", flame)]
    fn handle_reddit_simple_superscript_closer(&mut self, node: &'a AstNode<'a>) -> bool {
        // Short circuit this parse if we find a closer for our simple
        // superscript, close the superscript without consuming any chars, then
        // resume parsing. Unfortunately we have to do this first to catch the
        // eof case - this seems to be the only place in the inline syntax where
        // eof is a delimiter that needs to be processed like this.
        if self.simple_superscript_openers > 0 {
            let next = self.peek_char().cloned();
            let close = match next {
                None |
                Some(b' ') |
                Some(b'\t') |
                Some(b'\r') |
                Some(b'\n') => true,
                _ => false
            };
            if close {
                for _ in 0..self.simple_superscript_openers {
                    let del = make_inline(self.arena, NodeValue::Text(b"".to_vec()));
                    self.push_delimiter(b'.', false, true, &del);
                    node.append(del);
                }

                self.simple_superscript_openers = 0;
                self.special_chars[b' ' as usize] = false;
                self.special_chars[b'\t' as usize] = false;
                return true;
            }
        }

        false
    }

    pub fn spnl(&mut self) {
        self.skip_spaces();
        if self.skip_line_end() {
            self.skip_spaces();
        }
    }
}

pub fn manual_scan_link_url(input: &[u8], reddit_quirks: bool) -> Option<usize> {
    let len = input.len();
    let mut i = 0;
    let mut nb_p = 0;

    if i < len && input[i] == b'<' {
        i += 1;
        while i < len {
            let b = input[i];
            if b == b'>' {
                i += 1;
                break;
            } else if b == b'\\' {
                i += 2;
            } else if b == b'\n' || b == b'<' {
                return None;
            } else {
                i += 1;
            }
        }
    } else {
        while i < len {
            if input[i] == b'\\' {
                i += 2;
            } else if input[i] == b'(' {
                nb_p += 1;
                i += 1;
                if nb_p > 32 {
                    return None;
                }
            } else if input[i] == b')' {
                if nb_p == 0 {
                    break;
                }
                nb_p -= 1;
                i += 1;
            } else if !reddit_quirks && isspace(input[i]) {
                break;
            } else if reddit_quirks && isspace(input[i]) {
                // Reddit allows space in links (but not newlines and tabs)
                if input[i] != b' ' {
                    break;
                }

                // Reddit allows spaces in links. Here we duplicate the logic
                // from handle_close_brackets to figure out if the thing that
                // comes after a space will parse as a title; and if so, if what
                // follows is a closing paren (for inline links) or eol (for
                // reference links).
                let starttitle = i + scanners::spacechars(&input[i..]).unwrap_or(0);
                let endtitle = starttitle + scanners::link_title(&input[starttitle..]).unwrap_or(0);
                if starttitle == endtitle {
                    // Not a title, just spaces
                    i = endtitle;
                } else {
                    // Scan past any spaces after the title
                    let endall = endtitle + scanners::spacechars(&input[endtitle..]).unwrap_or(0);
                    if endall < input.len() && input[endall] == b')' {
                        // The space was (probably) separating the title in an inline link
                        break;
                    } else if endall == input.len() {
                        // The space was (probably) separating the title in a reference link
                        break;
                    } else {
                        // Not a title, just spaces
                        // NB: endall - 1 is the _last_ space scanned by spacechars above,
                        // so that we leave one to interpret again in the loop, while not
                        // rescanning any others if there are multiple spaces.
                        i = endall - 1;
                    }
                }
            } else {
                i += 1;
            }
        }
    }

    if i >= len {
        None
    } else {
        Some(i)
    }
}

#[cfg_attr(feature = "flamegraphs", flame)]
pub fn make_inline<'a>(arena: &'a Arena<AstNode<'a>>, value: NodeValue) -> &'a AstNode<'a> {
    let ast = Ast {
        value: value,
        content: vec![],
        start_line: 0,
        start_column: 0,
        end_line: 0,
        end_column: 0,
        open: false,
        last_line_blank: false,
    };
    arena.alloc(Node::new(RefCell::new(ast)))
}

fn make_autolink<'a>(
    arena: &'a Arena<AstNode<'a>>,
    url: &[u8],
    kind: AutolinkType,
) -> &'a AstNode<'a> {
    let inl = make_inline(
        arena,
        NodeValue::Link(NodeLink {
            url: strings::clean_autolink(url, kind),
            title: vec![],
            l: false,
        }),
    );
    inl.append(make_inline(
        arena,
        NodeValue::Text(entity::unescape_html(url)),
    ));
    inl
}
