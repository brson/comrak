use ctype::{ispunct, isspace};
use entity;
use parser::AutolinkType;
use std::str;

pub fn unescape(v: &mut Vec<u8>) {
    let mut r = 0;
    let mut sz = v.len();

    while r < sz {
        if v[r] == b'\\' && r + 1 < sz && ispunct(v[r + 1]) {
            v.remove(r);
            sz -= 1;
        }
        if r >= sz {
            break;
        }
        r += 1;
    }
}

pub fn clean_autolink(url: &[u8], kind: AutolinkType) -> Vec<u8> {
    let mut url_vec = url.to_vec();
    trim(&mut url_vec);

    if url_vec.is_empty() {
        return url_vec;
    }

    let mut buf = Vec::with_capacity(url_vec.len());
    if kind == AutolinkType::Email {
        buf.extend_from_slice(b"mailto:");
    }

    buf.extend_from_slice(&entity::unescape_html(&url_vec));
    buf
}

pub fn normalize_whitespace(v: &[u8]) -> Vec<u8> {
    let mut last_char_was_space = false;
    let b = v;
    let mut r = Vec::with_capacity(v.len());
    let mut org = 0;
    let len = v.len();
    while org < len {
        let mut i = org;

        while i < len && !isspace(b[i]) {
            i += 1;
        }

        if i > org {
            r.extend_from_slice(&b[org..i]);
            last_char_was_space = false;
        }

        if i < len {
            if !last_char_was_space {
                r.push(b' ');
                last_char_was_space = true;
            }
            i += 1;
        }

        org = i;
    }

    r
}

pub fn remove_trailing_blank_lines(line: &mut Vec<u8>) {
    let mut i = line.len() - 1;
    loop {
        let c = line[i];

        if c != b' ' && c != b'\t' && !is_line_end_char(c) {
            break;
        }

        if i == 0 {
            line.clear();
            return;
        }

        i -= 1;
    }

    for i in i..line.len() {
        let c = line[i];

        if !is_line_end_char(c) {
            continue;
        }

        line.truncate(i);
        break;
    }
}

pub fn is_line_end_char(ch: u8) -> bool {
    match ch {
        10 | 13 => true,
        _ => false,
    }
}

pub fn is_space_or_tab(ch: u8) -> bool {
    match ch {
        9 | 32 => true,
        _ => false,
    }
}

pub fn chop_trailing_hashtags(line: &mut Vec<u8>) {
    rtrim(line);

    let orig_n = line.len() - 1;
    let mut n = orig_n;

    while line[n] == b'#' {
        if n == 0 {
            return;
        }
        n -= 1;
    }

    if n != orig_n && is_space_or_tab(line[n]) {
        line.truncate(n);
        rtrim(line);
    }
}

pub fn rtrim(line: &mut Vec<u8>) {
    let mut len = line.len();
    while len > 0 && isspace(line[len - 1]) {
        line.pop();
        len -= 1;
    }
}

pub fn ltrim(line: &mut Vec<u8>) {
    let mut len = line.len();
    while len > 0 && isspace(line[0]) {
        line.remove(0);
        len -= 1;
    }
}

pub fn trim(line: &mut Vec<u8>) {
    ltrim(line);
    rtrim(line);
}

pub fn rtrim_slice(mut i: &[u8]) -> &[u8] {
    let mut len = i.len();
    while len > 0 && isspace(i[len - 1]) {
        i = &i[..len - 1];
        len -= 1;
    }
    i
}

pub fn trim_slice(mut i: &[u8]) -> &[u8] {
    i = rtrim_slice(i);
    let mut len = i.len();
    while len > 0 && isspace(i[0]) {
        i = &i[1..];
        len -= 1;
    }
    i
}

pub fn clean_url(url: &[u8]) -> Vec<u8> {
    let url = trim_slice(url);

    let url_len = url.len();
    if url_len == 0 {
        return vec![];
    }

    let mut b = if url[0] == b'<' && url[url_len - 1] == b'>' {
        entity::unescape_html(&url[1..url_len - 1])
    } else {
        entity::unescape_html(url)
    };

    unescape(&mut b);
    b
}

pub fn clean_title(title: &[u8]) -> Vec<u8> {
    let title_len = title.len();
    if title_len == 0 {
        return vec![];
    }

    let first = title[0];
    let last = title[title_len - 1];

    let mut b = if (first == b'\'' && last == b'\'') || (first == b'(' && last == b')')
        || (first == b'"' && last == b'"')
    {
        entity::unescape_html(&title[1..title_len - 1])
    } else {
        entity::unescape_html(title)
    };

    unescape(&mut b);
    b
}

pub fn is_blank(s: &[u8]) -> bool {
    for &c in s {
        match c {
            10 | 13 => return true,
            32 | 9 => (),
            _ => return false,
        }
    }
    true
}

pub fn normalize_label(i: &[u8]) -> Vec<u8> {
    let i = trim_slice(i);
    let mut v = String::with_capacity(i.len());
    let mut last_was_whitespace = false;
    for c in unsafe { str::from_utf8_unchecked(i) }.chars() {
        for e in c.to_lowercase() {
            if e.is_whitespace() {
                if !last_was_whitespace {
                    last_was_whitespace = true;
                    v.push(' ');
                }
            } else {
                last_was_whitespace = false;
                v.push(e);
            }
        }
    }
    v.into_bytes()
}

// Determine whether a url is "safe", especially that it
// is not "javascript:".
// Similar to snudown:
// https://github.com/reddit/snudown/blob/master/src/autolink.c#L33
pub fn validate_url_scheme(url: &[u8]) -> bool {
    let url = match str::from_utf8(url) {
        Ok(url) => url,
        Err(_) => return false,
    }.to_lowercase();

    static VALID_SCHEMES: &[&str] = &[
        "http://", "https://", "ftp://", "mailto:",
        "/", "git://", "steam://", "irc://", "news://", "mumble://",
        "ssh://", "ircs://", "ts3server://", "#"
    ];

    VALID_SCHEMES.iter().any(|scheme| url.starts_with(scheme))
}

// This is significantly faster than the line iterator in std. Std's iterator
// goes through several complex layers of abstraction and uses a slower memchr
// implementation.
pub fn fast_lines(buf: &str) -> FastLines {
    FastLines(buf.as_bytes())
}

pub struct FastLines<'a>(&'a [u8]);

impl<'a> Iterator for FastLines<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        use memchr::memchr;

        let slice = &mut self.0;

        if self.0.is_empty() {
            return None;
        }

        let line;

        unsafe {
            if let Some(i) = memchr(b'\n', slice) {
                if i > 0 && slice.get_unchecked(i - 1) == &b'\r' {
                    line = slice.get_unchecked(0..i - 1);
                } else {
                    line = slice.get_unchecked(0..i);
                }
                *slice = slice.get_unchecked(i + 1..);
            } else {
                line = slice;
                *slice = slice.get_unchecked(0..0);
            }

            Some(::std::str::from_utf8_unchecked(line))
        }
    }
}

/// Look for characters that indicate that the fast-path renderer can't be
/// taken. In my synthetic tests this table-based approach is surprisingly
/// faster than the jetscii crate that uses explicit simd intrinsics, but it
/// might be worth revisiting that later.
#[inline]
pub fn contains_forbidden_chars(s: &str) -> bool {
    static FORBIDDEN_CHARS: &[char] = &[
        '#', '_', '*', '=', '-', '~', '|', '[', '\\', '<', '>', '^', '`', '&', '/', ':', '@'
    ];

    lazy_static! {
        static ref TABLE: &'static [bool; 256] = {
            static mut TABLE: [bool; 256] = [false; 256];
            unsafe {
                for ch in FORBIDDEN_CHARS {
                    TABLE[*ch as usize] = true;
                }
                &TABLE
            }
        };
    }

    for byte in s.as_bytes().iter() {
        if TABLE[*byte as usize] {
            return true;
        }
    }

    false
}
