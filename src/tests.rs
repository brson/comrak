use {parse_document, Arena, ComrakOptions};
use cm;
use html;
use rtjson;

fn compare_strs(output: &str, expected: &str, kind: &str) {
    if output != expected {
        println!("Running {} test", kind);
        println!("Got:");
        println!("==============================");
        println!("{}", output);
        println!("==============================");
        println!();
        println!("Expected:");
        println!("==============================");
        println!("{}", expected);
        println!("==============================");
        println!();
    }
    assert_eq!(output, expected);
}

fn rtjson(input: &str, expected: &str) {
    rtjson_opts(input, expected, |_| ());
}

fn rtjson_opts<F>(input: &str, expected: &str, opts: F)
where
    F: Fn(&mut ComrakOptions),
{
    let arena = Arena::new();
    let mut options = ComrakOptions::default();
    opts(&mut options);
    options.rtjson = true;
    options.ext_table = true;

    let root = parse_document(&arena, input, &options);
    let mut output = vec![];
    html::format_document(root, &options, &mut output).unwrap();
    compare_strs(&String::from_utf8(output).unwrap(), expected, "regular");

    let mut md = vec![];
    cm::format_document(root, &options, &mut md).unwrap();
    let root = parse_document(&arena, &String::from_utf8(md).unwrap(), &options);
    let mut output_from_rt = vec![];
    html::format_document(root, &options, &mut output_from_rt).unwrap();
    compare_strs(
        &String::from_utf8(output_from_rt).unwrap(),
        expected,
        "roundtrip",
    );
}

#[test]
fn basic() {
    rtjson(
        concat!(
            "this is a link with **bold** and *italic*"
        ),
        concat!(
            "'document': [ { 'e': 'par', 'c': [{ 'e': 'link', 'u': 'https://reddit.com', 't': 'this is a link with bold and italic', 'f': [[1, 20, 4], [2, 29, 6]], }, ], }, ],"
        ),
    );
}

#[test]
fn basic_2() {
    rtjson(
        concat!(
            "*Hello Reddit*, this an example paragraph. Read more RTJson [here](https://docs.google.com/document/d/1Qpf2tl8iabZIEKvUSE3bFRV2QCIjxaXn-6Mblvojpvs/edit#heading=h.w2llmo96i6e1)",
        ),
        concat!(
            "'document': [",
                "{",
                    "'e': 'par',",
                    "'c': [",
                        "{",
                            "'e': 'text',",
                            "'t': 'Hello Reddit, this an example paragraph. Read more RTJson ',",
                            "'f': [[2, 0, 12]],",
                        "},",
                        "{",
                            "'e': 'link',",
                            "'t': 'here',",
                            "'u': 'https://docs.google.com/document/d/1Qpf2tl8iabZIEKvUSE3bFRV2QCIjxaXn-6Mblvojpvs/edit#heading=h.w2llmo96i6e1',",
                        "}",
                    "]",
                "},",
            "]",
        ),
    );
}

#[test]
fn heading() {
    rtjson(
        concat!(
            "### This heading contains plain text, [a link](https://reddit.com), and a u/username.\n\nHello, this is a paragraph."
        ),
        concat!(
            "'document':[{'e': 'h','l': 3,'c': [{'e': 'raw','t': 'This heading contains plain text, '},{'e': 'link','u': 'https://reddit.com','t': 'a link'},{'e': 'raw','t': ', and a '},{'e': 'u/','t': 'username'},{'e': 'raw','t': '.'}],},{'e': 'par','c': [{'e': 'text','t': 'Hello, this is a paragraph.'}],},]"
        ),
    );
}

#[test]
fn blockquote_1() {
    rtjson(
        concat!(
            ">This post begins with a blockquote.\n\nThis post has a paragraph in the middle.\n\n>This post ends with a blockquote."
        ),
        concat!(
            "'document': [{'e': 'blockquote','c': [{'e': 'par','c': [{'e': 'text','t': 'This post begins with a blockquote.',}]}]},{'e': 'par','c': [{'e': 'text','t': 'This post has a paragraph in the middle.',}]},{'e': 'blockquote','c': [{'e': 'par','c': [{'e': 'text','t': 'This post ends with a blockquote.',}]}]}]"
        ),
    );
}

#[test]
fn blockquote_only() {
    rtjson(
        concat!(
            ">A blockquote with nothing else."
        ),
        concat!(
            "'document': [{'e': 'blockquote','c': [{'e': 'par','c': [{'e': 'text','t': 'A blockquote with nothing else.'}]}],}]"
        ),
    );
}


// TODO: This utilizes one of the breaks.
// works for mow, but we should figure out which break it uses
#[test]
fn blockquote_with_br() {
    rtjson(
        concat!(
            ">Line proceeding; this line has a [link](https://reddit.com) and a r/redditlink  \n>Line preceding; no line proceeding  \n>  \n>No line preceding; no line proceeding  \n>  \n>No line preceding; line proceeding  \n>Line preceding"
        ),
        concat!(
            "'document': [{'e': 'blockquote','c': [{'e': 'par','c': [{'e': 'text','t': 'Line proceeding; this line has a '},{'e': 'link','u': 'https://reddit.com','t': 'link'},{'e': 'text','t': ' and a '},{'e': 'r/','t': 'redditlink'}]},{'e': 'par','c': [{'e': 'text','t': 'Line preceding; no line proceeding'}]},{'e': 'par','c': [{'e': 'text','t': ''}]},{'e': 'par','c': [{'e': 'text','t': 'No line preceding; no line proceeding'}]},{'e': 'par','c': [{'e': 'text','t': ''}]},{'e': 'par','c': [{'e': 'text','t': 'No line preceding; line proceeding'}]},{'e': 'par','c': [{'e': 'text','t': 'Line preceding'}]}]}]"
        ),
    );
}


// TODO: Ask Z about how this renders
#[test]
fn blockquote_newline_literal() {
    rtjson(
        concat!(
            ">This post ends with a blockquote\n\nwith embedded newlines."
        ),
        concat!(
            "'document': [{'e': 'blockquote','c': [{'e': 'par','c': [{'e': 'text','t': 'This post ends with a blockquote\n\nwith embedded newlines.',}]}]}]"
        ),
    );
}

#[test]
fn redditlink() {
    rtjson(
        concat!(
            "Hello, **this is bold**, *this is italic*, ***this is both***. And this is a u/username and a r/subreddit."
        ),
        concat!(
            "'document': [{'e': 'par','c': [{'e': 'text','t': 'Hello, this is bold, this is italic, this is both. And this is a ', 'f': [[1, 7, 12], [2, 21, 14], [3, 37, 12]],},{'e': 'u/','t': 'username'},{'e': 'text','t': ' and a '},{'e': 'r/','t': 'subreddit'},{'e': 'text','t': '.'}]}]"
        ),
    );
}

#[test]
fn ul() {
    rtjson(
        concat!(
            "Below this is a list:\n\n* First item\n* Second item\n* Third item\n\nAbove this is a list."
        ),
        concat!(
            "<p>URI autolink: <a \
             href=\"https://www.pixiv.net\">https://www.pixiv.net</a></p>\n",
            "<p>Email autolink: <a \
             href=\"mailto:bill@microsoft.com\">bill@microsoft.com</a></p>\n",
            "<ul>\n",
            "<li>Inline <em>tag</em> <strong>ha</strong>.</li>\n",
            "<li>Inline <!-- comment --> <strong>ha</strong>.</li>\n",
            "<li>Inline <? processing instruction ?> <strong>ha</strong>.</li>\n",
            "<li>Inline <!DECLARATION OKAY> <strong>ha</strong>.</li>\n",
            "<li>Inline <![CDATA[ok]ha **ha** ]]> <strong>ha</strong>.</li>\n",
            "</ul>\n"
        ),
    );
}

#[test]
fn mixed_list_1() {
    rtjson(
        concat!(
            "<p>Where are you <a href=\"https://microsoft.com\" \
             title=\"today\">going</a>?</p>\n",
            "<p><a href=\"/here\">Where am I?</a></p>\n"
        ),
        concat!(
            "<p>I am <img src=\"http://i.imgur.com/QqK1vq7.png\" alt=\"eating things\" \
             />.</p>\n"
        ),
    );
}

#[test]
fn table_1() {
    rtjson_opts(
        concat!(
            "|Col 1|Col 2|Col 3|\n|:-|:-:|-:|\n|a |**bold*****bold+italic****italic*|a |"
        ),
        concat!(
            "<p>This [is] <a href=\"ok\">legit</a>, <a href=\"sure\" title=\"hm\">very</a> \
             legit.</p>\n"
        ),
        |opts| opts.ext_table = true,
    );
}

#[test]
fn table_2() {
    rtjson_opts(
        concat!(
            "These are two tables:\n\n|Table|1|\n|:-|:-|\n|c1:r1|c2:r1|\n\n|Table|2|\n|:-|:-|\n|c1:r2|c2:r2|"
        ),
        concat!(
            "'document': [{'e': 'par','c': [{'e': 'text','t': 'These are two tables:',}]},{'e': 'table','h': [{'c': [{'e': 'text','t': 'Table'}]},{'c': [{'e': 'text','t': '1',}]},],'b': [[{'c': [{'e': 'text','t': 'c1:r1'}]},{'c': [{'e': 'text','t': 'c2:r1'}]},]]},{'e': 'table','h': [{'c': [{'e': 'text','t': 'Table'}]},{'c': [{'e': 'text','t': '2'}]},],'b': [[{'c': [{'e': 'text','t': 'c1:r2'}]},{'c': [{'e': 'text','t': 'c2:r2'}]},]]},]"
        ),
        |opts| opts.ext_table = true,
    );
}

#[test]
fn autolink_www() {
    html_opts(
        concat!("www.autolink.com\n"),
        concat!("<p><a href=\"http://www.autolink.com\">www.autolink.com</a></p>\n"),
        |opts| opts.ext_autolink = true,
    );
}

#[test]
fn autolink_email() {
    html_opts(
        concat!("john@smith.com\n"),
        concat!("<p><a href=\"mailto:john@smith.com\">john@smith.com</a></p>\n"),
        |opts| opts.ext_autolink = true,
    );
}

#[test]
fn lists() {
    rtjson(
        concat!("1.\n   * 1 level [hello](www.reddit.com) nested - ul\n2. 0 levels nested - ol\n3. 0 levels nested - ol\n   1. 1 level nested - ol\n      1. 2 levels nested - ol\n      2. 2 levels nested - ol\n   2. 1 level nested - ol\n      * 2 levels nested - ul\n4. 0 levels nested - ol"),
        concat!(
            "<p><a href=\"https://google.com/search\">https://google.\
             com/search</a></p>\n"
        ),
    );

#[test]
fn autolink_scheme_multiline() {
    html_opts(
        concat!("https://google.com/search\nhttps://www.google.com/maps"),
        concat!(
            "<p><a href=\"https://google.com/search\">https://google.\
             com/search</a>\n<a href=\"https://www.google.com/maps\">\
             https://www.google.com/maps</a></p>\n"
        ),
        |opts| opts.ext_autolink = true,
    );
}

#[test]
fn tagfilter() {
    html_opts(
        concat!("hi <xmp> ok\n", "\n", "<xmp>\n"),
        concat!("<p>hi &lt;xmp> ok</p>\n", "&lt;xmp>\n"),
        |opts| opts.ext_tagfilter = true,
    );
}

#[test]
fn codeblock() {
    rtjson(
        concat!("    function test() {\n      console.log(\"notice the blank line before this function?\");\n    }"),
        concat!(
            "'document':[",
            "{",
            "'e':'code',",
            "'c':[",
            "{",
            "'e':'raw',",
            "'t':'function test() {'",
            "},",
            "{",
            "'e':'raw',",
            "'t':'  console.log(&quot;notice the blank line before this function?&quot;);'",
            "},",
            "{",
            "'e':'raw',",
            "'t':'}'",
            "},",
            "{",
            "'e':'raw',",
            "'t':''",
            "},",
            "],",
            "},",
            "],",
        ),
    );
}

#[test]
fn tasklist_32() {
    html_opts(
        concat!(
            "- [ ] List item 1\n",
            "- [ ] This list item is **bold**\n",
            "- [x] There is some `code` here\n"
        ),
        concat!(
            "<ul>\n",
            "<li><input type=\"checkbox\" disabled=\"\" /> List item 1</li>\n",
            "<li><input type=\"checkbox\" disabled=\"\" /> This list item is <strong>bold</strong></li>\n",
            "<li><input type=\"checkbox\" disabled=\"\" checked=\"\" /> There is some <code>code</code> here</li>\n",
            "</ul>\n"
        ),
        |opts| opts.ext_tasklist = true,
    );
}

#[test]
fn superscript() {
    html_opts(
        concat!("e = mc^2^.\n"),
        concat!("<p>e = mc<sup>2</sup>.</p>\n"),
        |opts| opts.ext_superscript = true,
    );
}

#[test]
fn header_ids() {
    html_opts(
        concat!(
            "# Hi.\n",
            "## Hi 1.\n",
            "### Hi.\n",
            "#### Hello.\n",
            "##### Hi.\n",
            "###### Hello.\n"
        ),
        concat!(
            "<h1><a href=\"#hi\" aria-hidden=\"true\" class=\"anchor\" id=\"user-content-hi\"></a>Hi.</h1>\n",
            "<h2><a href=\"#hi-1\" aria-hidden=\"true\" class=\"anchor\" id=\"user-content-hi-1\"></a>Hi 1.</h2>\n",
            "<h3><a href=\"#hi-2\" aria-hidden=\"true\" class=\"anchor\" id=\"user-content-hi-2\"></a>Hi.</h3>\n",
            "<h4><a href=\"#hello\" aria-hidden=\"true\" class=\"anchor\" id=\"user-content-hello\"></a>Hello.</h4>\n",
            "<h5><a href=\"#hi-3\" aria-hidden=\"true\" class=\"anchor\" id=\"user-content-hi-3\"></a>Hi.</h5>\n",
            "<h6><a href=\"#hello-1\" aria-hidden=\"true\" class=\"anchor\" id=\"user-content-hello-1\"></a>Hello.</h6>\n"
        ),
        |opts| opts.ext_header_ids = Some("user-content-".to_owned()),
    );
}

#[test]
fn footnotes() {
    html_opts(
        concat!(
            "Here is a[^nowhere] footnote reference,[^1] and another.[^longnote]\n",
            "\n",
            "This is another note.[^note]\n",
            "\n",
            "[^note]: Hi.\n",
            "\n",
            "[^1]: Here is the footnote.\n",
            "\n",
            "[^longnote]: Here's one with multiple blocks.\n",
            "\n",
            "    Subsequent paragraphs are indented.\n",
            "\n",
            "        code\n",
            "\n",
            "This is regular content.\n",
            "\n",
            "[^unused]: This is not used.\n"
        ),
        concat!(
            "<p>Here is a[^nowhere] footnote reference,<sup class=\"footnote-ref\"><a href=\"#fn1\" \
             id=\"fnref1\">[1]</a></sup> and another.<sup class=\"footnote-ref\"><a \
             href=\"#fn2\" id=\"fnref2\">[2]</a></sup></p>\n",
            "<p>This is another note.<sup class=\"footnote-ref\"><a href=\"#fn3\" \
             id=\"fnref3\">[3]</a></sup></p>\n",
            "<p>This is regular content.</p>\n",
            "<section class=\"footnotes\">\n",
            "<ol>\n",
            "<li id=\"fn1\">\n",
            "<p>Here is the footnote. <a href=\"#fnref1\" \
             class=\"footnote-backref\">↩</a></p>\n",
            "</li>\n",
            "<li id=\"fn2\">\n",
            "<p>Here's one with multiple blocks.</p>\n",
            "<p>Subsequent paragraphs are indented.</p>\n",
            "<pre><code>code\n",
            "</code></pre>\n",
            "<a href=\"#fnref2\" class=\"footnote-backref\">↩</a>\n",
            "</li>\n",
            "<li id=\"fn3\">\n",
            "<p>Hi. <a href=\"#fnref3\" \
             class=\"footnote-backref\">↩</a></p>\n",
            "</li>\n",
            "</ol>\n",
            "</section>\n"
        ),
        |opts| opts.ext_footnotes = true,
    );
}

#[test]
fn footnote_does_not_eat_exclamation() {
    html_opts(
        concat!("Here's my footnote![^a]\n", "\n", "[^a]: Yep.\n"),
        concat!(
            "<p>Here's my footnote!<sup class=\"footnote-ref\"><a href=\"#fn1\" \
             id=\"fnref1\">[1]</a></sup></p>\n",
            "<section class=\"footnotes\">\n",
            "<ol>\n",
            "<li id=\"fn1\">\n",
            "<p>Yep. <a href=\"#fnref1\" class=\"footnote-backref\">↩</a></p>\n",
            "</li>\n",
            "</ol>\n",
            "</section>\n"
        ),
        |opts| opts.ext_footnotes = true,
    );
}

#[test]
fn regression_back_to_back_ranges() {
    html(
        "**bold*****bold+italic***",
        "<p><strong>bold</strong><em><strong>bold+italic</strong></em></p>\n",
    );
}
