cargo run -- --rtjson --spec specs/rtjson/rtjson.spec
# RTJSON TEST   

```````````````````````````````` example
this is a link with **bold** and *italic*
.
{"document":[{"c":[{"e":"text","f":[[1,20,4],[2,29,6]],"t":"this is a link with bold and italic"}],"e":"par"}]}
````````````````````````````````

```````````````````````````````` example
*Hello Reddit*, this an example paragraph. Read more RTJson [here](https://docs.google.com/document/d/1Qpf2tl8iabZIEKvUSE3bFRV2QCIjxaXn-6Mblvojpvs/edit#heading=h.w2llmo96i6e1)
.
{"document":[{"c":[{"e":"text","f":[[2,0,12]],"t":"Hello Reddit, this an example paragraph. Read more RTJson "},{"e":"link","t":"here","u":"https://docs.google.com/document/d/1Qpf2tl8iabZIEKvUSE3bFRV2QCIjxaXn-6Mblvojpvs/edit#heading=h.w2llmo96i6e1"}],"e":"par"}]}
````````````````````````````````

```````````````````````````````` example
### This heading contains plain text, [a link](https://reddit.com), and a u/username.

Hello, this is a paragraph.
.
{"document":[{"e":"h","l":3,"c":[{"e":"raw","t":"This heading contains plain text, "},{"e":"link","t":"a link","u":"https://reddit.com"},{"e":"raw","t":", and a "},{"e":"u/","l":false,"t":"username"},{"e":"raw","t":"."}]},{"e":"par","c":[{"e":"text","t":"Hello, this is a paragraph."}]}]}
````````````````````````````````

We also have multiple header levels that work with empty text.

```````````````````````````````` example
###

##

#

Hello, this is a paragraph.
.
{"document":[{"c":[{"e":"raw","t":""}],"e":"h","l":3},{"c":[{"e":"raw","t":""}],"e":"h","l":2},{"c":[{"e":"raw","t":""}],"e":"h","l":1},{"c":[{"e":"text","t":"Hello, this is a paragraph."}],"e":"par"}]}
````````````````````````````````


```````````````````````````````` example
>This post begins with a blockquote.

This post has a paragraph in the middle.

>This post ends with a blockquote.
.
{"document":[{"c":[{"e":"par","c":[{"e":"text","t":"This post begins with a blockquote."}]}],"e":"blockquote"},{"c":[{"e":"text","t":"This post has a paragraph in the middle."}],"e":"par"},{"c":[{"c":[{"e":"text","t":"This post ends with a blockquote."}],"e":"par"}],"e":"blockquote"}]}
````````````````````````````````

```````````````````````````````` example
>A blockquote with nothing else.
.
{"document":[{"c":[{"c":[{"e":"text","t":"A blockquote with nothing else."}],"e":"par"}],"e":"blockquote"}]}
````````````````````````````````

// TODO: This utilizes one of the breaks.

```````````````````````````````` example
>Line proceeding; this line has a [link](https://reddit.com) and a r/redditlink.
>  
>Line preceding; no line proceeding  
>  
>No line preceding; no line proceeding  
>  
>No line preceding; line proceeding
>
>Line preceding
.
{"document":[{"c":[{"c":[{"e":"text","t":"Line proceeding; this line has a "},{"e":"link","t":"link","u":"https://reddit.com"},{"e":"text","t":" and a "},{"e":"r/","l":false,"t":"redditlink"},{"e":"text","t":"."}],"e":"par"},{"c":[{"e":"text","t":"Line preceding; no line proceeding"}],"e":"par"},{"c":[{"e":"text","t":"No line preceding; no line proceeding"}],"e":"par"},{"c":[{"e":"text","t":"No line preceding; line proceeding"}],"e":"par"},{"c":[{"e":"text","t":"Line preceding"}],"e":"par"}],"e":"blockquote"}]}
````````````````````````````````

```````````````````````````````` example
>This post ends with a blockquote\n\nwith embedded newlines.
.
{"document":[{"c":[{"c":[{"e":"text","t":"This post ends with a blockquote\\n\\nwith embedded newlines."}],"e":"par"}],"e":"blockquote"}]}
````````````````````````````````

```````````````````````````````` example
Hello, **this is bold**, *this is italic*, ***this is both***. And this is a u/username and a /r/subreddit.
.
{"document":[{"c":[{"e":"text","f":[[1,7,12],[2,21,14],[3,37,12]],"t":"Hello, this is bold, this is italic, this is both. And this is a "},{"e":"u/","l":false,"t":"username"},{"e":"text","t":" and a "},{"e":"r/","l":true,"t":"subreddit"},{"e":"text","t":"."}],"e":"par"}]}
````````````````````````````````

```````````````````````````````` example
Below this is a list:

* First item
* Second item
* Third item

Above this is a list.
.
{"document":[{"c":[{"e":"text","t":"Below this is a list:"}],"e":"par"},{"c":[{"e":"li","c":[{"c":[{"e":"text","t":"First item"}],"e":"par"}]},{"c":[{"c":[{"e":"text","t":"Second item"}],"e":"par"}],"e":"li"},{"c":[{"c":[{"e":"text","t":"Third item"}],"e":"par"}],"e":"li"}],"e":"list","o":false},{"c":[{"e":"text","t":"Above this is a list."}],"e":"par"}]}
````````````````````````````````

```````````````````````````````` example
* First item
* Second item with [a link with **bold** and *italic*](https://reddit.com) and a u/username
   1. First item
   2. Second item
      * First item
      * Second item
.
{"document":[{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item with "},{"e":"link","f":[[1,12,4],[2,21,6]],"t":"a link with bold and italic","u":"https://reddit.com"},{"e":"text","t":" and a "},{"e":"u/","l":false,"t":"username"}]},{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]},{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]}]}]}]}]}]}]}]}
````````````````````````````````

```````````````````````````````` example
|Col 1|Col 2|Col 3|
|:-|:-:|-:|
|a |**bold**&#8203;***bold+italic***&#8203;*italic*|a |
.
{"document":[{"e":"table","h":[{"a":"L","c":[{"e":"text","t":"Col 1"}]},{"a":"C","c":[{"e":"text","t":"Col 2"}]},{"a":"R","c":[{"e":"text","t":"Col 3"}]}],"c":[[{"c":[{"e":"text","t":"a"}]},{"c":[{"e":"text","f":[[1,0,4],[3,5,11],[2,17,6]],"t":"bold​bold+italic​italic"}]},{"c":[{"e":"text","t":"a"}]}]]}]}
````````````````````````````````

```````````````````````````````` example
These are two tables:

|Table|1|
|:-|:-|
|c1:r1|c2:r1|

|Table|2|
|:-|:-|
|c1:r2|c2:r2|
.
{"document":[{"e":"par","c":[{"e":"text","t":"These are two tables:"}]},{"e":"table","h":[{"a":"L","c":[{"e":"text","t":"Table"}]},{"a":"L","c":[{"e":"text","t":"1"}]}],"c":[[{"c":[{"e":"text","t":"c1:r1"}]},{"c":[{"e":"text","t":"c2:r1"}]}]]},{"e":"table","h":[{"a":"L","c":[{"e":"text","t":"Table"}]},{"a":"L","c":[{"e":"text","t":"2"}]}],"c":[[{"c":[{"e":"text","t":"c1:r2"}]},{"c":[{"e":"text","t":"c2:r2"}]}]]}]}
````````````````````````````````

```````````````````````````````` example
Hello reddit, \*\***this should be bold,**\*\* the stars around it should not be.
.
{"document":[{"e":"par","c":[{"e":"text","f":[[1,16,20]],"t":"Hello reddit, **this should be bold,** the stars around it should not be."}]}]}
````````````````````````````````

```````````````````````````````` example
Hello reddit, \*\***this should be bold,**\*\* the stars around it should not be.

\> This is text with an arrow in front

>This is a quote

*Here we have something in italics*

\*Here we have something with single-stars around it\*

\`Is this a codeblock?\`

\~\~This should not be strike through\~\~

~~But this should be~~

\[Finally here we have no link\]\(www.example.com\)

www.thisisalink.com
.
{"document": [{"c": [{"e": "text", "t": "Hello reddit, **this should be bold,** the stars around it should not be.", "f": [[1, 16, 20]]}], "e": "par"}, {"c": [{"e": "text", "t": "> This is text with an arrow in front"}], "e": "par"}, {"c": [{"c": [{"e": "text", "t": "This is a quote"}], "e": "par"}], "e": "blockquote"}, {"c": [{"e": "text", "t": "Here we have something in italics", "f": [[2, 0, 33]]}], "e": "par"}, {"c": [{"e": "text", "t": "*Here we have something with single-stars around it*"}], "e": "par"}, {"c": [{"e": "text", "t": "`Is this a codeblock?`"}], "e": "par"}, {"c": [{"e": "text", "t": "~~This should not be strike through~~"}], "e": "par"}, {"c": [{"e": "text", "t": "But this should be", "f": [[8, 0, 18]]}], "e": "par"}, {"c": [{"e": "text", "t": "[Finally here we have no link]("}, {"u": "http://www.example.com", "e": "link", "t": "www.example.com"}, {"e": "text", "t": ")"}], "e": "par"}, {"c": [{"u": "http://www.thisisalink.com", "e": "link", "t": "www.thisisalink.com"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
1. 
   * 1 level [hello](http://www.reddit.com) nested - ul
2. 0 levels nested - ol
3. 0 levels nested - ol
   1. 1 level nested - ol
      1. 2 levels nested - ol
      2. 2 levels nested - ol
   2. 1 level nested - ol
      * 2 levels nested - ul
4. 0 levels nested - ol
.
{"document":[{"c":[{"c":[{"c":[{"e":"text","t":""}],"e":"par"},{"c":[{"c":[{"c":[{"e":"text","t":"1 level "},{"e":"link","t":"hello","u":"http://www.reddit.com"},{"e":"text","t":" nested - ul"}],"e":"par"}],"e":"li"}],"e":"list","o":false}],"e":"li"},{"c":[{"c":[{"e":"text","t":"0 levels nested - ol"}],"e":"par"}],"e":"li"},{"c":[{"c":[{"e":"text","t":"0 levels nested - ol"}],"e":"par"},{"c":[{"c":[{"c":[{"e":"text","t":"1 level nested - ol"}],"e":"par"},{"c":[{"c":[{"c":[{"e":"text","t":"2 levels nested - ol"}],"e":"par"}],"e":"li"},{"c":[{"c":[{"e":"text","t":"2 levels nested - ol"}],"e":"par"}],"e":"li"}],"e":"list","o":true}],"e":"li"},{"c":[{"c":[{"e":"text","t":"1 level nested - ol"}],"e":"par"},{"c":[{"c":[{"c":[{"e":"text","t":"2 levels nested - ul"}],"e":"par"}],"e":"li"}],"e":"list","o":false}],"e":"li"}],"e":"list","o":true}],"e":"li"},{"c":[{"c":[{"e":"text","t":"0 levels nested - ol"}],"e":"par"}],"e":"li"}],"e":"list","o":true}]}
````````````````````````````````

```````````````````````````````` example
* First item
* Second item with [a link with **bold** and *italic*](https://reddit.com) and a u/username
   1. First item
   2. Second item
      * First item
      * Second item
.
{"document":[{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item with "},{"e":"link","f":[[1,12,4],[2,21,6]],"t":"a link with bold and italic","u":"https://reddit.com"},{"e":"text","t":" and a "},{"e":"u/","l":false,"t":"username"}]},{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]},{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]}]}]}]}]}]}]}]}
````````````````````````````````

```````````````````````````````` example
    function test() {
      console.log("notice the blank line before this function?");
    }
.
{"document":[{"e":"code","c":[{"e":"raw","t":"function test() {"},{"e":"raw","t":"  console.log(\"notice the blank line before this function?\");"},{"e":"raw","t":"}"}]}]}
````````````````````````````````

Say I have many formats nested in one format range. We would want to keep that 
overall format through the whole thing, while also getting rid of the old format
each time we went on.

```````````````````````````````` example
*__bold__ ~underline~ ~~strikethrough~~*
.
{"document":[{"c":[{"e":"text","f":[[3,0,4],[2,4,1],[6,5,9],[2,14,1],[10,15,13]],"t":"bold underline strikethrough"}],"e":"par"}]}
````````````````````````````````

In the case that we have two of the same styles nested within one another we want
the ranges to all be the same. This will likely only result from the legacy client.

```````````````````````````````` example
**This is some __bold__ text.**
.
{"document":[{"c":[{"e":"text","f":[[1,0,23]],"t":"This is some bold text."}],"e":"par"}]}
````````````````````````````````

```````````````````````````````` example
foo^^^bar
.
{"document":[{"c":[{"e":"text","f":[[32,3,3]],"t":"foobar"}],"e":"par"}]}
````````````````````````````````

Lets try the same thing with links

```````````````````````````````` example
[**This is some __bold__ text.**](http://www.reddit.com)
.
{"document":[{"c":[{"e":"link","f":[[1,0,23]],"t":"This is some bold text.","u":"http://www.reddit.com"}],"e":"par"}]}
````````````````````````````````

Now we also allow images with captions for our parser. An exclamation point allows us to point towards our image using the format 
![alt](/mediaid "caption")

```````````````````````````````` example
These media assets have captions:

![gif](abcdef "an animated gif")

![img](fedcba "an image")
.
{"document":[{"c":[{"e":"text","t":"These media assets have captions:"}],"e":"par"},{"c":"an animated gif","e":"gif","id":"abcdef"},{"c":"an image","e":"img","id":"fedcba"}]}
````````````````````````````````

Or without captions

```````````````````````````````` example
These media assets don't have captions:

![gif](abcdef)

![img](fedcba)
.
{"document":[{"c":[{"e":"text","t":"These media assets don't have captions:"}],"e":"par"},{"e":"gif","id":"abcdef"},{"e":"img","id":"fedcba"}]}
````````````````````````````````


```````````````````````````````` example
Raw "quotes", &ampersands, and <lt & gt> should be escaped.
.
{"document":[{"c":[{"e":"text","t":"Raw \"quotes\", &ampersands, and <lt & gt> should be escaped."}],"e":"par"}]}
````````````````````````````````

```````````````````````````````` example
HTML entities like & \" < and > should not be escaped, unless they are malformed like &amp or &quot".
.
{"document":[{"c":[{"e":"text","t":"HTML entities like & \" < and > should not be escaped, unless they are malformed like &amp or &quot\"."}],"e":"par"}]}
````````````````````````````````

```````````````````````````````` example
Escaping to HTML entities like & and " shouldn't impact format ranges like **this** or ~~*this*~~.
.
{"document":[{"c":[{"e":"text","f":[[1,75,4],[10,83,4]],"t":"Escaping to HTML entities like & and \" shouldn't impact format ranges like this or this."}],"e":"par"}]}
````````````````````````````````

We now support spoiler text and here are some test for those.

```````````````````````````````` example
This >!areallylongword *followed* by something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","f":[[2,16,8]],"t":"areallylongword followed by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}
````````````````````````````````



```````````````````````````````` example
This >!areallylongword **in bold followed** by something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","f":[[1,16,16]],"t":"areallylongword in bold followed by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}
````````````````````````````````



```````````````````````````````` example
This >!areallylongword ~followed~ *by* something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","f":[[4,16,8],[2,25,2]],"t":"areallylongword followed by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}
````````````````````````````````


```````````````````````````````` example
This >!areallylongword [*followed*](http://www.example.com "Hoping captions still work") by something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","t":"areallylongword "},{"a":"Hoping captions still work","e":"link","f":[[2,0,8]],"t":"followed","u":"http://www.example.com"},{"e":"text","t":" by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}
````````````````````````````````


```````````````````````````````` example
This >!areallylongword /u/followed by something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","t":"areallylongword "},{"e":"u/","l":true,"t":"followed"},{"e":"text","t":" by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}
````````````````````````````````

String with opening marker (!>), but no closing marker

```````````````````````````````` example
This is a string with an >!opener but no closer
.
{"document":[{"c":[{"e":"text","t":"This is a string with an >!opener but no closer"}],"e":"par"}]}
````````````````````````````````

Spoiler contained within a formatting run, e.g., *These italics include !>spoilertext<!*

```````````````````````````````` example
This is a string with a >!Spoiler and then >!another spoiler!< inside of it.!<
.
{"document":[{"c":[{"e":"text","t":"This is a string with a "},{"c":[{"e":"text","t":"Spoiler and then "},{"c":[{"e":"text","t":"another spoiler"}],"e":"spoilertext"},{"e":"text","t":" inside of it."}],"e":"spoilertext"}],"e":"par"}]}
````````````````````````````````

Spoiler nested within another spoiler (not sure what the behavior is)

```````````````````````````````` example
*This is an italic sentence with >!this!< inside it.*
.
{"document":[{"c":[{"e":"text","f":[[2,0,32]],"t":"This is an italic sentence with "},{"c":[{"e":"text","f":[[2,0,4]],"t":"this"}],"e":"spoilertext"},{"e":"text","f":[[2,0,11]],"t":" inside it."}],"e":"par"}]}
````````````````````````````````

Headers can contain no text

```````````````````````````````` example
#
Bloop
.
{"document": [{"e": "h", "l": 1, "c":[{"e":"raw","t":""}]}, {"c": [{"e": "text", "t": "Bloop"}], "e": "par"}]}
````````````````````````````````

Headers can be separated by tabs

```````````````````````````````` example
#	Bleep
Bloop
.
{"document": [{"c": [{"e": "raw", "t": "Bleep"}], "e": "h", "l": 1}, {"c": [{"e": "text", "t": "Bloop"}], "e": "par"}]}
````````````````````````````````

Escaped non-spoilertext.

```````````````````````````````` example
\>!spoilertext!<
.
{"document": [{"c": [{"e": "text", "t": ">!spoilertext!<"}], "e": "par"}]}
````````````````````````````````

This one actually ends up as a blockquote

```````````````````````````````` example
>\!spoilertext!<
.
{"document": [{"c": [{"c": [{"e": "text", "t": "!spoilertext!<"}], "e": "par"}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
>!spoilertext\!<
.
{"document": [{"c": [{"e": "text", "t": ">!spoilertext!<"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
>!spoilertext!\<
.
{"document": [{"c": [{"e": "text", "t": ">!spoilertext!<"}], "e": "par"}]}
````````````````````````````````

Spoilertext doesn't presently need to be leading and trailing whitespace-delimited
like other emphasis

```````````````````````````````` example
a>!b!<c
.
{"document": [{"c": [{"e": "text", "t": "a"}, {"c": [{"e": "text", "t": "b"}], "e": "spoilertext"}, {"e": "text", "t": "c"}], "e": "par"}]}
````````````````````````````````

Nor do spoilertext delimiters need to be flanking text like other emphasis

```````````````````````````````` example
a >! b !< c
.
{"document": [{"c": [{"e": "text", "t": "a "}, {"c": [{"e": "text", "t": " b "}], "e": "spoilertext"}, {"e": "text", "t": " c"}], "e": "par"}]}
````````````````````````````````

Spoilertext doesn't care about matching quotes

```````````````````````````````` example
>!"spoilertext!<"!<
.
{"document": [{"c": [{"c": [{"e": "text", "t": "\"spoilertext"}], "e": "spoilertext"}, {"e": "text", "t": "\"!<"}], "e": "par"}]}
````````````````````````````````

Leftmost opener gets precedent when they are mismatched

This is spoilertext, not emphasis

```````````````````````````````` example
>! _spoilertext!<a_
.
{"document": [{"c": [{"c": [{"e": "text", "t": " _spoilertext"}], "e": "spoilertext"}, {"e": "text", "t": "a_"}], "e": "par"}]}
````````````````````````````````

This is emphasis, not spoilertext

```````````````````````````````` example
_a>!spoilertext_ !<
.
{"document": [{"c": [{"e": "text", "t": "a>!spoilertext !<", "f": [[2, 0, 14]]}], "e": "par"}]}
````````````````````````````````

Superscript time, baby.

```````````````````````````````` example
some ^basic superscript
.
{"document": [{"c": [{"e": "text", "t": "some basic superscript", "f": [[32, 5, 5]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
some ^(basic superscript)
.
{"document": [{"c": [{"e": "text", "t": "some basic superscript", "f": [[32, 5, 17]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
some ^(basic
superscript) across lines
.
{"document": [{"c": [{"e": "text", "t": "some basic superscript across lines", "f": [[32, 5, 17]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^(I'm a redditor)
.
{"document": [{"c": [{"e": "text", "t": "I'm a redditor", "f": [[32, 0, 14]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^(we're looking at you i.reddit.com)
.
{"document": [{"c": [{"e": "text", "t": "we're looking at you i.reddit.com", "f": [[32, 0, 33]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^(put some ☎ here)
.
{"document": [{"c": [{"e": "text", "t": "put some \u260e here", "f": [[32, 0, 15]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^☎☎☎
.
{"document": [{"c": [{"e": "text", "t": "\u260e\u260e\u260e", "f": [[32, 0, 3]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
This should not \^be superscript
.
{"document": [{"c": [{"e": "text", "t": "This should not ^be superscript"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
This should not \^(be superscript)
.
{"document": [{"c": [{"e": "text", "t": "This should not ^(be superscript)"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
What about ^(this http://example.com autolink)?
.
{"document": [{"c": [{"e": "text", "t": "What about this ", "f": [[32, 11, 5]]}, {"u": "http://example.com", "e": "link", "t": "http://example.com", "f": [[32, 0, 18]]}, {"e": "text", "t": " autolink?", "f": [[32, 0, 9]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
What about ^(this [explicit](http://example.com) link)?
.
{"document": [{"c": [{"e": "text", "t": "What about this ", "f": [[32, 11, 5]]}, {"u": "http://example.com", "e": "link", "t": "explicit", "f": [[32, 0, 8]]}, {"e": "text", "t": " link?", "f": [[32, 0, 5]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
What about ^(this _emphasis_ here)?
.
{"document": [{"c": [{"e": "text", "t": "What about this emphasis here?", "f": [[32, 11, 5], [34, 16, 8], [32, 24, 5]]}], "e": "par"}]}
````````````````````````````````

This is a plain-text caret because it is followed by whitespace

```````````````````````````````` example
^
.
{"document": [{"c": [{"e": "text", "t": "^"}], "e": "par"}]}
````````````````````````````````

This is a superscripted caret.

```````````````````````````````` example
^^
.
{"document":[{"c":[{"e":"text","f":[[32,0,1]],"t":"^"}],"e":"par"}]}
````````````````````````````````

```````````````````````````````` example
something ^ something else
.
{"document": [{"c": [{"e": "text", "t": "something ^ something else"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
some not ^ (superscript i imagine)
.
{"document": [{"c": [{"e": "text", "t": "some not ^ (superscript i imagine)"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
this surprisingly is a superscript caret ^^ here
.
{"document": [{"c": [{"e": "text", "t": "this surprisingly is a superscript caret ^ here", "f": [[32, 41, 1]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^[link](http://example.com)
.
{"document": [{"c": [{"u": "http://example.com", "e": "link", "t": "link", "f": [[32, 0, 4]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^test[link](http://example.com)text
.
{"document": [{"c": [{"e": "text", "t": "test", "f": [[32, 0, 4]]}, {"u": "http://example.com", "e": "link", "t": "link", "f": [[32, 0, 4]]}, {"e": "text", "t": "text", "f": [[32, 0, 4]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^
.
{"document": [{"c": [{"e": "text", "t": "^"}], "e": "par"}]}
````````````````````````````````

This is, perhaps suprisingly, a superscripted caret

```````````````````````````````` example
^^
.
{"document": [{"c": [{"e": "text", "t": "^", "f": [[32, 0, 1]]}], "e": "par"}]}
````````````````````````````````

Nested superscript is parsed, but not yet reflected in the rendering

```````````````````````````````` example
this is ^super^duper
.
{"document": [{"c": [{"e": "text", "t": "this is superduper", "f": [[32, 8, 10]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
this is ^(super ^duper yeah)
.
{"document": [{"c": [{"e": "text", "t": "this is super duper yeah", "f": [[32, 8, 16]]}], "e": "par"}]}
````````````````````````````````

Again, nested superscript parsed but not rendered

```````````````````````````````` example
this is ^(super ^(duper fooper) yeah)
.
{"document": [{"c": [{"e": "text", "t": "this is super duper fooper yeah", "f": [[32, 8, 23]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^
a
.
{"document": [{"c": [{"e": "text", "t": "^ a"}], "e": "par"}]}
````````````````````````````````

Again, nested.

```````````````````````````````` example
^^(a)
.
{"document": [{"c": [{"e": "text", "t": "a", "f": [[32, 0, 1]]}], "e": "par"}]}
````````````````````````````````

Weird case, parse ends at first space

```````````````````````````````` example
^^(a b)
.
{"document": [{"c": [{"e": "text", "t": "^(a b)", "f": [[32, 0, 3]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^(^(a))
.
{"document": [{"c": [{"e": "text", "t": "a", "f": [[32, 0, 1]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^(foo
.
{"document": [{"c": [{"e": "text", "t": "^(foo"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^\(parens\)
.
{"document": [{"c": [{"e": "text", "t": "(parens)", "f": [[32, 0, 8]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^\(parens)
.
{"document": [{"c": [{"e": "text", "t": "(parens)", "f": [[32, 0, 8]]}], "e": "par"}]}
````````````````````````````````

Tabs

```````````````````````````````` example
^test	tab
.
{"document": [{"c": [{"e": "text", "t": "test\ttab", "f": [[32, 0, 4]]}], "e": "par"}]}
````````````````````````````````

This case parses differently than snudown, the second simple superscript not
being recognized

```````````````````````````````` example
this is ^(super ^duper)
.
{"document": [{"c": [{"e": "text", "t": "this is super ^duper", "f": [[32, 8, 12]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
this is ^(super ^duper yeah)
.
{"document": [{"c": [{"e": "text", "t": "this is super duper yeah", "f": [[32, 8, 16]]}], "e": "par"}]}
````````````````````````````````


```````````````````````````````` example
this is ^super)
.
{"document": [{"c": [{"e": "text", "t": "this is super)", "f": [[32, 8, 6]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
- a ^b
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "a b", "f": [[32, 2, 1]]}], "e": "par"}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
^(>!test!<)
.
{"document": [{"c": [{"c": [{"e": "text", "t": "test", "f": [[32, 0, 4]]}], "e": "spoilertext"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^>!test!<
.
{"document": [{"c": [{"c": [{"e": "text", "t": "test", "f": [[32, 0, 4]]}], "e": "spoilertext"}], "e": "par"}]}
````````````````````````````````

Not spoilertext, broken at first space

```````````````````````````````` example
^>!te st!<
.
{"document": [{"c": [{"e": "text", "t": ">!te st!<", "f": [[32, 0, 4]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
raw html ^<br>
.
{"document": [{"c": [{"e": "text", "t": "raw html <br>", "f": [[32, 9, 4]]}], "e": "par"}]}
````````````````````````````````

Formatting ranges here seem questionable to me since they don't superscript the
space between "a" and "b"...

```````````````````````````````` example
^(_a_ *b*)
.
{"document": [{"c": [{"e": "text", "t": "a b", "f": [[34, 0, 1], [32, 1, 1], [34, 2, 1]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^a&nbsp;b
.
{"document": [{"c": [{"e": "text", "t": "a\u00a0b", "f": [[32, 0, 3]]}], "e": "par"}]}
````````````````````````````````

Parsed nested, but not rendered

```````````````````````````````` example
^a&nbsp;^b
.
{"document": [{"c": [{"e": "text", "t": "a\u00a0b", "f": [[32, 0, 3]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^a&#32;b
.
{"document": [{"c": [{"e": "text", "t": "a b", "f": [[32, 0, 3]]}], "e": "par"}]}
````````````````````````````````

Parsed nested, but not rendered

```````````````````````````````` example
^a&#32;^b
.
{"document": [{"c": [{"e": "text", "t": "a b", "f": [[32, 0, 3]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^(a (b\) c)
.
{"document": [{"c": [{"e": "text", "t": "a (b) c", "f": [[32, 0, 7]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^[broken link](http://example.com)
.
{"document": [{"c": [{"e": "text", "t": "^"}, {"u": "http://example.com", "e": "link", "t": "broken link"}], "e": "par"}]}
````````````````````````````````

Some weird cases with nested "simple" superscript. Correct parsing of these is
debatable, and usually snoomark is different from snudown.

Here, the third caret isn't closed.

```````````````````````````````` example
^^(^^(a))
.
{"document": [{"c": [{"e": "text", "t": "^a", "f": [[32, 0, 2]]}], "e": "par"}]}
````````````````````````````````

Here, the third and fifth

```````````````````````````````` example
^^(^^(^))
.
{"document": [{"c": [{"e": "text", "t": "^^", "f": [[32, 0, 2]]}], "e": "par"}]}
````````````````````````````````

Simple superscript with emphasis

```````````````````````````````` example
^_emphasis_
.
{"document": [{"c": [{"e": "text", "t": "emphasis", "f": [[34, 0, 8]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
^*emphasis*
.
{"document": [{"c": [{"e": "text", "t": "emphasis", "f": [[34, 0, 8]]}], "e": "par"}]}
````````````````````````````````

Thematic breaks (hr)

```````````````````````````````` example
a

***

b
.
{"document": [{"c": [{"e": "text", "t": "a"}], "e": "par"}, {"e": "hr"}, {"c": [{"e": "text", "t": "b"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
a
***
b
.
{"document": [{"c": [{"e": "text", "t": "a"}], "e": "par"}, {"e": "hr"}, {"c": [{"e": "text", "t": "b"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
***
.
{"document": [{"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
a

---

b
.
{"document": [{"c": [{"e": "text", "t": "a"}], "e": "par"}, {"e": "hr"}, {"c": [{"e": "text", "t": "b"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
**********
.
{"document": [{"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
***
***
.
{"document": [{"e": "hr"}, {"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
***
---
***
.
{"document": [{"e": "hr"}, {"e": "hr"}, {"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
---
***
---
.
{"document": [{"e": "hr"}, {"e": "hr"}, {"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
**

--
.
{"document": [{"c": [{"e": "text", "t": "**"}], "e": "par"}, {"c": [{"e": "text", "t": "--"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
a

--------

b
.
{"document": [{"c": [{"e": "text", "t": "a"}], "e": "par"}, {"e": "hr"}, {"c": [{"e": "text", "t": "b"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
a

********

b
.
{"document": [{"c": [{"e": "text", "t": "a"}], "e": "par"}, {"e": "hr"}, {"c": [{"e": "text", "t": "b"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
---a
.
{"document": [{"c": [{"e": "text", "t": "---a"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
--- a
.
{"document": [{"c": [{"e": "text", "t": "--- a"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
 ***

.
{"document": [{"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
  ***
.
{"document": [{"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
   ***

.
{"document": [{"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
    ***

.
{"document": [{"c": [{"e": "raw", "t": "***"}], "e": "code"}]}
````````````````````````````````

Hard linebreaks

Two spaces:

```````````````````````````````` example
line1  
line2
.
{"document": [{"c": [{"e": "text", "t": "line1"}, {"e": "br"}, {"e": "text", "t": "line2"}], "e": "par"}]}
````````````````````````````````

Three spaces:

```````````````````````````````` example
line1   
line2
.
{"document": [{"c": [{"e": "text", "t": "line1"}, {"e": "br"}, {"e": "text", "t": "line2"}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
line1\
line2
.
{"document": [{"c": [{"e": "text", "t": "line1"}, {"e": "br"}, {"e": "text", "t": "line2"}], "e": "par"}]}
````````````````````````````````

Backslash-space

```````````````````````````````` example
line1\ 
line2
.
{"document":[{"c":[{"e":"text","t":"line1\\ line2"}],"e":"par"}]}
````````````````````````````````

Space-backslash

```````````````````````````````` example
line1 \
line2
.
{"document": [{"c": [{"e": "text", "t": "line1 "}, {"e": "br"}, {"e": "text", "t": "line2"}], "e": "par"}]}
````````````````````````````````

With formatted text

```````````````````````````````` example
_line1_\
_line2_
.
{"document": [{"c": [{"e": "text", "t": "line1", "f": [[2, 0, 5]]}, {"e": "br"}, {"e": "text", "t": "line2", "f": [[2, 0, 5]]}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
_line1\
line2_
.
{"document": [{"c": [{"e": "text", "t": "line1", "f": [[2, 0, 5]]}, {"e": "br"}, {"e": "text", "t": "line2", "f": [[2, 0, 5]]}], "e": "par"}]}
````````````````````````````````

## Block elements inside list items

```````````````````````````````` example
- p

    - l
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- p

    1. l
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- p

    > q
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}], "e": "blockquote"}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- p

    ```
    c
    ```
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"e": "raw", "t": "c"}], "e": "code"}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- p

    ---
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"e": "hr"}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- p

    # h
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"e": "raw", "t": "h"}], "e": "h", "l": 1}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

These examples have block items embedded directly in line with a list item,
syntaxes snudown didn't generally support, with the exception of paragraphs.

```````````````````````````````` example
- p
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- - l
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- 1. l
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- > q
.
{"document": [{"c": [{"c": [{"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}], "e": "blockquote"}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- ```
  c
  ```
.
{"document": [{"c": [{"c": [{"c": [{"e": "raw", "t": "c"}], "e": "code"}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

Curious cases w/ dashes. This seems to be how cm specs it.

This is just a hr, not a li > hr:

```````````````````````````````` example
- ---
.
{"document": [{"e": "hr"}]}
````````````````````````````````

This though is an li > list

```````````````````````````````` example
- -
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}], "e": "li"}], "e": "list"
, "o": false}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

This again is just an hr...

```````````````````````````````` example
- - -
.
{"document": [{"e": "hr"}]}
````````````````````````````````

And the above with stars

```````````````````````````````` example
* ***
.
{"document": [{"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
* *
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}], "e": "li"}], "e": "list"
, "o": false}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
* * *
.
{"document": [{"e": "hr"}]}
````````````````````````````````

```````````````````````````````` example
- # h
.
{"document": [{"c": [{"c": [{"c": [{"e": "raw", "t": "h"}], "e": "h", "l": 1}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

Ok, now let's do all that again with ordered lists

```````````````````````````````` example
1. p

    - l
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. p

    1. l
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. p

    > q
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}], "e": "blockquote"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. p

    ```
    c
    ```
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"e": "raw", "t": "c"}], "e": "code"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. p

    ---
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"e": "hr"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. p

    # h
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"c": [{"e": "raw", "t": "h"}], "e": "h", "l": 1}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

These examples have block items embedded directly in line with a list item,
syntaxes snudown didn't generally support, with the exception of paragraphs.

```````````````````````````````` example
1. p
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. - l
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. 1. l
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. > q
.
{"document": [{"c": [{"c": [{"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}], "e": "blockquote"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. ```
   c
   ```
.
{"document": [{"c": [{"c": [{"c": [{"e": "raw", "t": "c"}], "e": "code"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. ---
.
{"document": [{"c": [{"c": [{"e": "hr"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. -
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}], "e": "li"}], "e": "list", "o": false}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

ordered list > unordered list > unordered list

```````````````````````````````` example
1. - -
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}], "e": "li"}], "e": "list", "o": false}], "e": "li"}], "e": "list", "o": false}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. # h
.
{"document": [{"c": [{"c": [{"c": [{"e": "raw", "t": "h"}], "e": "h", "l": 1}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

And tables.

```````````````````````````````` example
- p

    |a|b|
    |-|-|
    |c|d|
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "p"}], "e": "par"}, {"h": [{"a": "", "c": [{"e": "text", "t": "a"}]}, {"a": "", "c": [{"e": "text", "t": "b"}]}], "c": [[{"c": [{"e": "text", "t": "c"}]}, {"c": [{"e": "text", "t": "d"}]}]], "e": "table"}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

```````````````````````````````` example
- |a|b|
  |-|-|
  |c|d|
.
{"document": [{"c": [{"c": [{"h": [{"a": "", "c": [{"e": "text", "t": "a"}]}, {"a": "", "c": [{"e": "text", "t": "b"}]}], "c": [[{"c": [{"e": "text", "t": "c"}]}, {"c": [{"e": "text", "t": "d"}]}]], "e": "table"}], "e": "li"}], "e": "list", "o": false}]}
````````````````````````````````

## Block elements inside blockquotes

```````````````````````````````` example
> q
>
> - l
.
{"document": [{"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> q
>
> 1. l
.
{"document": [{"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> q
>
> > q
.
{"document": [{"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}, {"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}], "e": "blockquote"}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> q
>
> ```
> c
> ```
.
{"document": [{"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}, {"c": [{"e": "raw", "t": "c"}], "e": "code"}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> q
>
> ---

.
{"document": [{"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}, {"e": "hr"}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> q
>
> # h

.
{"document": [{"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}, {"c": [{"e": "raw", "t": "h"}], "e": "h", "l": 1}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> q
>
> |a|b|
> |-|-|
> |c|d|
.
{"document": [{"c": [{"c": [{"e": "text", "t": "q"}], "e": "par"}, {"h": [{"a": "", "c": [{"e": "text", "t": "a"}]}, {"a": "", "c": [{"e": "text", "t": "b"}]}], "c": [[{"c": [{"e": "text", "t": "c"}]}, {"c": [{"e": "text", "t": "d"}]}]], "e": "table"}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> - l

.
{"document": [{"c": [{"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> 1. l
.
{"document": [{"c": [{"c": [{"c": [{"c": [{"e": "text", "t": "l"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> ```
> c
> ```
.
{"document": [{"c": [{"c": [{"e": "raw", "t": "c"}], "e": "code"}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> ---
.
{"document": [{"c": [{"e": "hr"}], "e": "blockquote"}]}
````````````````````````````````

```````````````````````````````` example
> |a|b|
> |-|-|
> |c|d|
.
{"document": [{"c": [{"h": [{"a": "", "c": [{"e": "text", "t": "a"}]}, {"a": "", "c": [{"e": "text", "t": "b"}]}], "c": [[{"c": [{"e": "text", "t": "c"}]}, {"c": [{"e": "text", "t": "d"}]}]], "e": "table"}], "e": "blockquote"}]}
````````````````````````````````

## CREATE-1749

Ordered lists must start with "1".

```````````````````````````````` example
2. a
.
{"document": [{"c": [{"e": "text", "t": "2. a"}], "e": "par"}]}
````````````````````````````````

But "2" will parse for subsequent items.

```````````````````````````````` example
1. a
2. b
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "a"}], "e": "par"}], "e": "li"}, {"c": [{"c": [{"e": "text", "t": "b"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

It's ok to get the numbers wrong after "1".

```````````````````````````````` example
1. a
3. b
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "a"}], "e": "par"}], "e": "li"}, {"c": [{"c": [{"e": "text", "t": "b"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

Again sublists must start with "1".

```````````````````````````````` example
1. a
    2. b
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "a 2. b"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1. a
    1. b
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "a"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "b"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

Again, not a sublist, just paragraph text.

```````````````````````````````` example
1. 2. b
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": "2. b"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

A sublist.

```````````````````````````````` example
1. 1. b
.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "b"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

## Tests of no-markup cases to ensure the quick_render optimization works.

```````````````````````````````` example
One line paragraph.
.
{"document": [{"c": [{"e": "text", "t": "One line paragraph."}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
Multi
line
paragraph.
.
{"document": [{"c": [{"e": "text", "t": "Multi line paragraph."}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
One
paragraph.

Two
paragraphs.
.
{"document": [{"c": [{"e": "text", "t": "One paragraph."}], "e": "par"}, {"c": [{"e": "text", "t": "Two paragraphs."}], "e": "par"}]}
````````````````````````````````

With extra spaces

```````````````````````````````` example
 One 
  paragraph.

Two 
  paragraphs.
.
{"document": [{"c": [{"e": "text", "t": "One paragraph."}], "e": "par"}, {"c": [{"e": "text", "t": "Two paragraphs."}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
One
paragraph.


Two
paragraphs.

.
{"document": [{"c": [{"e": "text", "t": "One paragraph."}], "e": "par"}, {"c": [{"e": "text", "t": "Two paragraphs."}], "e": "par"}]}
````````````````````````````````

```````````````````````````````` example
Para
    not code
.
{"document": [{"c": [{"e": "text", "t": "Para not code"}], "e": "par"}]}
````````````````````````````````

Some bailout cases that quick_render rejects after speculating that it
will be able to process the document.

```````````````````````````````` example
Para
1. list
.
{"document": [{"c": [{"e": "text", "t": "Para"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "list"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
Para

1. list
.
{"document": [{"c": [{"e": "text", "t": "Para"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "list"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
Para
1) list
.
{"document": [{"c": [{"e": "text", "t": "Para"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "list"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
Para

1) list
.
{"document": [{"c": [{"e": "text", "t": "Para"}], "e": "par"}, {"c": [{"c": [{"c": [{"e": "text", "t": "list"}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
1.

.
{"document": [{"c": [{"c": [{"c": [{"e": "text", "t": ""}], "e": "par"}], "e": "li"}], "e": "list", "o": true}]}
````````````````````````````````

```````````````````````````````` example
Para

    code
.
{"document": [{"c": [{"e": "text", "t": "Para"}], "e": "par"}, {"c": [{"e": "raw", "t": "code"}], "e": "code"}]}
````````````````````````````````
