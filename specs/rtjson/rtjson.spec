cargo run -- --rtjson --spec specs/rtjson/rtjson.spec
# RTJSON TEST   

```````````````````````````````` example
this is a link with **bold** and *italic*
.
{"document":[{"c":[{"e":"text","f":[[1,20,4],[2,29,6]],"t":"this is a link with bold and italic"}],"e":"par"}]}````````````````````````````````

```````````````````````````````` example
*Hello Reddit*, this an example paragraph. Read more RTJson [here](https://docs.google.com/document/d/1Qpf2tl8iabZIEKvUSE3bFRV2QCIjxaXn-6Mblvojpvs/edit#heading=h.w2llmo96i6e1)
.
{"document":[{"c":[{"e":"text","f":[[2,0,12]],"t":"Hello Reddit, this an example paragraph. Read more RTJson "},{"e":"link","t":"here","u":"https://docs.google.com/document/d/1Qpf2tl8iabZIEKvUSE3bFRV2QCIjxaXn-6Mblvojpvs/edit#heading=h.w2llmo96i6e1"}],"e":"par"}]}````````````````````````````````

```````````````````````````````` example
### This heading contains plain text, [a link](https://reddit.com), and a u/username.

Hello, this is a paragraph.
.
{"document":[{"e":"h","l":3,"c":[{"e":"raw","t":"This heading contains plain text, "},{"e":"link","t":"a link","u":"https://reddit.com"},{"e":"raw","t":", and a "},{"e":"u/","t":"username"},{"e":"raw","t":"."}]},{"e":"par","c":[{"e":"text","t":"Hello, this is a paragraph."}]}]}````````````````````````````````

```````````````````````````````` example
>This post begins with a blockquote.

This post has a paragraph in the middle.

>This post ends with a blockquote.
.
{"document":[{"c":[{"e":"par","c":[{"e":"text","t":"This post begins with a blockquote."}]}],"e":"blockquote"},{"c":[{"e":"text","t":"This post has a paragraph in the middle."}],"e":"par"},{"c":[{"c":[{"e":"text","t":"This post ends with a blockquote."}],"e":"par"}],"e":"blockquote"}]}````````````````````````````````

```````````````````````````````` example
>A blockquote with nothing else.
.
{"document":[{"c":[{"c":[{"e":"text","t":"A blockquote with nothing else."}],"e":"par"}],"e":"blockquote"}]}````````````````````````````````

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
{"document":[{"c":[{"c":[{"e":"text","t":"Line proceeding; this line has a "},{"e":"link","t":"link","u":"https://reddit.com"},{"e":"text","t":" and a "},{"e":"r/","t":"redditlink"},{"e":"text","t":"."}],"e":"par"},{"c":[{"e":"text","t":"Line preceding; no line proceeding"}],"e":"par"},{"c":[{"e":"text","t":"No line preceding; no line proceeding"}],"e":"par"},{"c":[{"e":"text","t":"No line preceding; line proceeding"}],"e":"par"},{"c":[{"e":"text","t":"Line preceding"}],"e":"par"}],"e":"blockquote"}]}````````````````````````````````

```````````````````````````````` example
>This post ends with a blockquote\n\nwith embedded newlines.
.
{"document":[{"c":[{"c":[{"e":"text","t":"This post ends with a blockquote\\n\\nwith embedded newlines."}],"e":"par"}],"e":"blockquote"}]}````````````````````````````````

```````````````````````````````` example
Hello, **this is bold**, *this is italic*, ***this is both***. And this is a u/username and a /r/subreddit.
.
{"document":[{"c":[{"e":"text","f":[[1,7,12],[2,21,14],[3,37,12]],"t":"Hello, this is bold, this is italic, this is both. And this is a "},{"e":"u/","t":"username"},{"e":"text","t":" and a "},{"e":"r/","l":true,"t":"subreddit"},{"e":"text","t":"."}],"e":"par"}]}````````````````````````````````

```````````````````````````````` example
Below this is a list:

* First item
* Second item
* Third item

Above this is a list.
.
{"document":[{"c":[{"e":"text","t":"Below this is a list:"}],"e":"par"},{"c":[{"e":"li","c":[{"c":[{"e":"text","t":"First item"}],"e":"par"}]},{"c":[{"c":[{"e":"text","t":"Second item"}],"e":"par"}],"e":"li"},{"c":[{"c":[{"e":"text","t":"Third item"}],"e":"par"}],"e":"li"}],"e":"list","o":false},{"c":[{"e":"text","t":"Above this is a list."}],"e":"par"}]}````````````````````````````````

```````````````````````````````` example
* First item
* Second item with [a link with **bold** and *italic*](https://reddit.com) and a u/username
   1. First item
   2. Second item
      * First item
      * Second item
.
{"document":[{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item with "},{"e":"link","f":[[1,12,4],[2,21,6]],"t":"a link with bold and italic","u":"https://reddit.com"},{"e":"text","t":" and a "},{"e":"u/","t":"username"}]},{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]},{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]}]}]}]}]}]}]}]}````````````````````````````````

```````````````````````````````` example
|Col 1|Col 2|Col 3|
|:-|:-:|-:|
|a |**bold**&#8203;***bold+italic***&#8203;*italic*|a |
.
{"document":[{"e":"table","h":[{"a":"L","c":[{"e":"text","t":"Col 1"}]},{"a":"C","c":[{"e":"text","t":"Col 2"}]},{"a":"R","c":[{"e":"text","t":"Col 3"}]}],"c":[[{"c":[{"e":"text","t":"a"}]},{"c":[{"e":"text","f":[[1,0,4],[3,5,11],[2,17,6]],"t":"bold​bold+italic​italic"}]},{"c":[{"e":"text","t":"a"}]}]]}]}````````````````````````````````

```````````````````````````````` example
These are two tables:

|Table|1|
|:-|:-|
|c1:r1|c2:r1|

|Table|2|
|:-|:-|
|c1:r2|c2:r2|
.
{"document":[{"e":"par","c":[{"e":"text","t":"These are two tables:"}]},{"e":"table","h":[{"a":"L","c":[{"e":"text","t":"Table"}]},{"a":"L","c":[{"e":"text","t":"1"}]}],"c":[[{"c":[{"e":"text","t":"c1:r1"}]},{"c":[{"e":"text","t":"c2:r1"}]}]]},{"e":"table","h":[{"a":"L","c":[{"e":"text","t":"Table"}]},{"a":"L","c":[{"e":"text","t":"2"}]}],"c":[[{"c":[{"e":"text","t":"c1:r2"}]},{"c":[{"e":"text","t":"c2:r2"}]}]]}]}````````````````````````````````

```````````````````````````````` example
Hello reddit, \*\***this should be bold,**\*\* the stars around it should not be.
.
{"document":[{"e":"par","c":[{"e":"text","f":[[1,16,20]],"t":"Hello reddit, **this should be bold,** the stars around it should not be."}]}]}````````````````````````````````

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
{"document": [{"c": [{"e": "text", "t": "Hello reddit, **this should be bold,** the stars around it should not be.", "f": [[1, 16, 20]]}], "e": "par"}, {"c": [{"e": "text", "t": "> This is text with an arrow in front"}], "e": "par"}, {"c": [{"c": [{"e": "text", "t": "This is a quote"}], "e": "par"}], "e": "blockquote"}, {"c": [{"e": "text", "t": "Here we have something in italics", "f": [[2, 0, 33]]}], "e": "par"}, {"c": [{"e": "text", "t": "*Here we have something with single-stars around it*"}], "e": "par"}, {"c": [{"e": "text", "t": "`Is this a codeblock?`"}], "e": "par"}, {"c": [{"e": "text", "t": "~~This should not be strike through~~"}], "e": "par"}, {"c": [{"e": "text", "t": "But this should be", "f": [[8, 0, 18]]}], "e": "par"}, {"c": [{"e": "text", "t": "[Finally here we have no link]("}, {"u": "http://www.example.com", "e": "link", "t": "www.example.com"}, {"e": "text", "t": ")"}], "e": "par"}, {"c": [{"u": "http://www.thisisalink.com", "e": "link", "t": "www.thisisalink.com"}], "e": "par"}]}````````````````````````````````

```````````````````````````````` example
1. 
   * 1 level [hello](www.reddit.com) nested - ul
2. 0 levels nested - ol
3. 0 levels nested - ol
   1. 1 level nested - ol
      1. 2 levels nested - ol
      2. 2 levels nested - ol
   2. 1 level nested - ol
      * 2 levels nested - ul
4. 0 levels nested - ol
.
{"document":[{"c":[{"c":[{"c":[{"e":"text","t":""}],"e":"par"},{"c":[{"c":[{"c":[{"e":"text","t":"1 level "},{"e":"link","t":"hello","u":"www.reddit.com"},{"e":"text","t":" nested - ul"}],"e":"par"}],"e":"li"}],"e":"list","o":false}],"e":"li"},{"c":[{"c":[{"e":"text","t":"0 levels nested - ol"}],"e":"par"}],"e":"li"},{"c":[{"c":[{"e":"text","t":"0 levels nested - ol"}],"e":"par"},{"c":[{"c":[{"c":[{"e":"text","t":"1 level nested - ol"}],"e":"par"},{"c":[{"c":[{"c":[{"e":"text","t":"2 levels nested - ol"}],"e":"par"}],"e":"li"},{"c":[{"c":[{"e":"text","t":"2 levels nested - ol"}],"e":"par"}],"e":"li"}],"e":"list","o":true}],"e":"li"},{"c":[{"c":[{"e":"text","t":"1 level nested - ol"}],"e":"par"},{"c":[{"c":[{"c":[{"e":"text","t":"2 levels nested - ul"}],"e":"par"}],"e":"li"}],"e":"list","o":false}],"e":"li"}],"e":"list","o":true}],"e":"li"},{"c":[{"c":[{"e":"text","t":"0 levels nested - ol"}],"e":"par"}],"e":"li"}],"e":"list","o":true}]}````````````````````````````````

```````````````````````````````` example
* First item
* Second item with [a link with **bold** and *italic*](https://reddit.com) and a u/username
   1. First item
   2. Second item
      * First item
      * Second item
.
{"document":[{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item with "},{"e":"link","f":[[1,12,4],[2,21,6]],"t":"a link with bold and italic","u":"https://reddit.com"},{"e":"text","t":" and a "},{"e":"u/","t":"username"}]},{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]},{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]}]}]}]}]}]}]}]}````````````````````````````````

```````````````````````````````` example
    function test() {
      console.log("notice the blank line before this function?");
    }
.
{"document":[{"e":"code","c":[{"e":"raw","t":"function test() {"},{"e":"raw","t":"  console.log(\"notice the blank line before this function?\");"},{"e":"raw","t":"}"}]}]}````````````````````````````````

Say I have many formats nested in one format range. We would want to keep that 
overall format through the whole thing, while also getting rid of the old format
each time we went on.

```````````````````````````````` example
*__bold__ ~underline~ ~~strikethrough~~*
.
{"document":[{"c":[{"e":"text","f":[[3,0,4],[2,4,1],[6,5,9],[2,14,1],[10,15,13]],"t":"bold underline strikethrough"}],"e":"par"}]}````````````````````````````````

In the case that we have two of the same styles nested within one another we want
the ranges to all be the same. This will likely only result from the legacy client.

```````````````````````````````` example
**This is some __bold__ text.**
.
{"document":[{"c":[{"e":"text","f":[[1,0,23]],"t":"This is some bold text."}],"e":"par"}]}````````````````````````````````

```````````````````````````````` example
foo^^^bar
.
{"document":[{"c":[{"e":"text","f":[[32,3,3]],"t":"foobar"}],"e":"par"}]}````````````````````````````````

Lets try the same thing with links

```````````````````````````````` example
[**This is some __bold__ text.**](www.reddit.com)
.
{"document":[{"c":[{"e":"link","f":[[1,0,23]],"t":"This is some bold text.","u":"www.reddit.com"}],"e":"par"}]}````````````````````````````````

Now we also allow images with captions for our parser. An exclamation point allows us to point towards our image using the format 
![alt](/mediaid "caption")

```````````````````````````````` example
These media assets have captions:

![gif](abcdef "an animated gif")

![img](fedcba "an image")
.
{"document":[{"c":[{"e":"text","t":"These media assets have captions:"}],"e":"par"},{"c":"an animated gif","e":"gif","id":"abcdef"},{"c":"an image","e":"img","id":"fedcba"}]}````````````````````````````````

Or without captions

```````````````````````````````` example
These media assets don't have captions:

![gif](abcdef)

![img](fedcba)
.
{"document":[{"c":[{"e":"text","t":"These media assets don't have captions:"}],"e":"par"},{"e":"gif","id":"abcdef"},{"e":"img","id":"fedcba"}]}````````````````````````````````


```````````````````````````````` example
Raw "quotes", &ampersands, and <lt & gt> should be escaped.
.
{"document":[{"c":[{"e":"text","t":"Raw \"quotes\", &ampersands, and <lt & gt> should be escaped."}],"e":"par"}]}````````````````````````````````

```````````````````````````````` example
HTML entities like & \" < and > should not be escaped, unless they are malformed like &amp or &quot".
.
{"document":[{"c":[{"e":"text","t":"HTML entities like & \" < and > should not be escaped, unless they are malformed like &amp or &quot\"."}],"e":"par"}]}````````````````````````````````

```````````````````````````````` example
Escaping to HTML entities like & and " shouldn't impact format ranges like **this** or ~~*this*~~.
.
{"document":[{"c":[{"e":"text","f":[[1,75,4],[10,83,4]],"t":"Escaping to HTML entities like & and \" shouldn't impact format ranges like this or this."}],"e":"par"}]}````````````````````````````````

We now support spoiler text and here are some test for those.

```````````````````````````````` example
This >!areallylongword *followed* by something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","f":[[2,16,8]],"t":"areallylongword followed by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}````````````````````````````````



```````````````````````````````` example
This >!areallylongword **in bold followed** by something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","f":[[1,16,16]],"t":"areallylongword in bold followed by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}````````````````````````````````



```````````````````````````````` example
This >!areallylongword ~followed~ *by* something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","f":[[4,16,8],[2,25,2]],"t":"areallylongword followed by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}````````````````````````````````



```````````````````````````````` example
This >!areallylongword [*followed*](www.example.com "Hoping captions still work") by something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","t":"areallylongword "},{"a":"Hoping captions still work","e":"link","f":[[2,0,8]],"t":"followed","u":"www.example.com"},{"e":"text","t":" by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}````````````````````````````````



```````````````````````````````` example
This >!areallylongword /u/followed by something!< EMAIL_OK_SET
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","t":"areallylongword "},{"e":"u/","l":true,"t":"followed"},{"e":"text","t":" by something"}],"e":"spoilertext"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}````````````````````````````````

String with opening marker (!>), but no closing marker

```````````````````````````````` example
This is a string with an >!opener but no closer
.
{"document":[{"c":[{"e":"text","t":"This is a string with an >!opener but no closer"}],"e":"par"}]}````````````````````````````````

Spoiler contained within a formatting run, e.g., *These italics include !>spoilertext<!*

```````````````````````````````` example
This is a string with a >!Spoiler and then >!another spoiler!< inside of it.!<
.
{"document":[{"c":[{"e":"text","t":"This is a string with a "},{"c":[{"e":"text","t":"Spoiler and then "},{"c":[{"e":"text","t":"another spoiler"}],"e":"spoilertext"},{"e":"text","t":" inside of it."}],"e":"spoilertext"}],"e":"par"}]}````````````````````````````````

Spoiler nested within another spoiler (not sure what the behavior is)

```````````````````````````````` example
*This is an italic sentence with >!this!< inside it.*
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","f":[[2,16,8]],"t":"areallylongword followed by something"}],"e":"s"},{"e":"text","t":" EMAIL_OK_SET"}],"e":"par"}]}````````````````````````````````
