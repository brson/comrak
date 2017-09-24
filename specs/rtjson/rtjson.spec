cargo run -- --rtjson --spec specs/rtjson.spec
# RTJSON TEST   

```````````````````````````````` example
this is a link with **bold** and *italic*
.
{"document":[{"e":"par","c":[{"e":"text","t":"this is a link with bold and italic","f":[[1, 20, 4], [2, 29, 6]]}]}]}````````````````````````````````

```````````````````````````````` example
*Hello Reddit*, this an example paragraph. Read more RTJson [here](https://docs.google.com/document/d/1Qpf2tl8iabZIEKvUSE3bFRV2QCIjxaXn-6Mblvojpvs/edit#heading=h.w2llmo96i6e1)
.
{"document":[{"e":"par","c":[{"e":"text","t":"Hello Reddit, this an example paragraph. Read more RTJson ","f":[[2, 0, 12]]},{"e":"link","u":"https://docs.google.com/document/d/1Qpf2tl8iabZIEKvUSE3bFRV2QCIjxaXn-6Mblvojpvs/edit#heading=h.w2llmo96i6e1","t":"here"}]}]}````````````````````````````````

```````````````````````````````` example
### This heading contains plain text, [a link](https://reddit.com), and a u/username.

Hello, this is a paragraph.
.
{"document":[{"e":"h","l":3,"c":[{"e":"raw","t":"This heading contains plain text, "},{"e":"link","u":"https://reddit.com","t":"a link"},{"e":"raw","t":", and a "},{"e":"u/","t":"username"},{"e":"raw","t":"."}]},{"e":"par","c":[{"e":"text","t":"Hello, this is a paragraph."}]}]}````````````````````````````````

```````````````````````````````` example
>This post begins with a blockquote.

This post has a paragraph in the middle.

>This post ends with a blockquote.
.
{"document":[{"e":"blockquote","c":[{"e":"par","c":[{"e":"text","t":"This post begins with a blockquote."}]}]},{"e":"par","c":[{"e":"text","t":"This post has a paragraph in the middle."}]},{"e":"blockquote","c":[{"e":"par","c":[{"e":"text","t":"This post ends with a blockquote."}]}]}]}````````````````````````````````

```````````````````````````````` example
>A blockquote with nothing else.
.
{"document":[{"e":"blockquote","c":[{"e":"par","c":[{"e":"text","t":"A blockquote with nothing else."}]}]}]}````````````````````````````````

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
{"document":[{"e":"blockquote","c":[{"e":"par","c":[{"e":"text","t":"Line proceeding; this line has a "},{"e":"link","u":"https://reddit.com","t":"link"},{"e":"text","t":" and a "},{"e":"r/","t":"redditlink"},{"e":"text","t":"."}]},{"e":"par","c":[{"e":"text","t":"Line preceding; no line proceeding"}]},{"e":"par","c":[{"e":"text","t":"No line preceding; no line proceeding"}]},{"e":"par","c":[{"e":"text","t":"No line preceding; line proceeding"}]},{"e":"par","c":[{"e":"text","t":"Line preceding"}]}]}]}````````````````````````````````

```````````````````````````````` example
>This post ends with a blockquote\n\nwith embedded newlines.
.
{"document":[{"e":"blockquote","c":[{"e":"par","c":[{"e":"text","t":"This post ends with a blockquote\n\nwith embedded newlines."}]}]}]}````````````````````````````````

```````````````````````````````` example
Hello, **this is bold**, *this is italic*, ***this is both***. And this is a u/username and a /r/subreddit.
.
{"document":[{"e":"par","c":[{"e":"text","t":"Hello, this is bold, this is italic, this is both. And this is a ","f":[[1, 7, 12], [2, 21, 14], [3, 37, 12]]},{"e":"u/","t":"username"},{"e":"text","t":" and a "},{"e":"r/","t":"subreddit"},{"e":"text","t":"."}]}]}````````````````````````````````

```````````````````````````````` example
Below this is a list:

* First item
* Second item
* Third item

Above this is a list.
.
{"document":[{"e":"par","c":[{"e":"text","t":"Below this is a list:"}]},{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Third item"}]}]}]},{"e":"par","c":[{"e":"text","t":"Above this is a list."}]}]}````````````````````````````````

```````````````````````````````` example
* First item
* Second item with [a link with **bold** and *italic*](https://reddit.com) and a u/username
   1. First item
   2. Second item
      * First item
      * Second item
.
{"document":[{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item with "},{"e":"link","u":"https://reddit.com","t":"a link with bold and italic","f":[[1, 12, 4], [2, 21, 6]]},{"e":"text","t":" and a "},{"e":"u/","t":"username"}]},{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]},{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]}]}]}]}]}]}]}]}````````````````````````````````

```````````````````````````````` example
|Col 1|Col 2|Col 3|
|:-|:-:|-:|
|a |**bold**&#8203;***bold+italic***&#8203;*italic*|a |
.
{"document":[{"e":"table","h":[{"a":"L","c":[{"e":"text","t":"Col 1"}]},{"a":"C","c":[{"e":"text","t":"Col 2"}]},{"a":"R","c":[{"e":"text","t":"Col 3"}]}],"c":[[{"c":[{"e":"text","t":"a"}]},{"c":[{"e":"text","t":"bold​bold+italic​italic","f":[[1, 0, 4], [3, 7, 11], [2, 21, 6]]}]},{"c":[{"e":"text","t":"a"}]}]]}]}````````````````````````````````

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
{"document":[{"e":"par","c":[{"e":"text","t":"Hello reddit, **this should be bold,** the stars around it should not be.","f":[[1, 16, 20]]}]}]}````````````````````````````````

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
{"document":[{"e":"par","c":[{"e":"text","t":"Hello reddit, **this should be bold,** the stars around it should not be.","f":[[1, 16, 20]]}]},{"e":"par","c":[{"e":"text","t":"&gt; This is text with an arrow in front"}]},{"e":"blockquote","c":[{"e":"par","c":[{"e":"text","t":"This is a quote"}]}]},{"e":"par","c":[{"e":"text","t":"Here we have something in italics","f":[[2, 0, 33]]}]},{"e":"par","c":[{"e":"text","t":"*Here we have something with single-stars around it*"}]},{"e":"par","c":[{"e":"text","t":"`Is this a codeblock?`"}]},{"e":"par","c":[{"e":"text","t":"~~This should not be strike through~~"}]},{"e":"par","c":[{"e":"text","t":"But this should be","f":[[8, 0, 18]]}]},{"e":"par","c":[{"e":"text","t":"[Finally here we have no link](www.example.com)"}]},{"e":"par","c":[{"e":"text","t":"www.thisisalink.com"}]}]}````````````````````````````````

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
{"document":[{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"1 level "},{"e":"link","u":"www.reddit.com","t":"hello"},{"e":"text","t":" nested - ul"}]}]}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"0 levels nested - ol"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"0 levels nested - ol"}]},{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"1 level nested - ol"}]},{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"2 levels nested - ol"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"2 levels nested - ol"}]}]}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"1 level nested - ol"}]},{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"2 levels nested - ul"}]}]}]}]}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"0 levels nested - ol"}]}]}]}]}````````````````````````````````

```````````````````````````````` example
* First item
* Second item with [a link with **bold** and *italic*](https://reddit.com) and a u/username
   1. First item
   2. Second item
      * First item
      * Second item
.
{"document":[{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item with "},{"e":"link","u":"https://reddit.com","t":"a link with bold and italic","f":[[1, 12, 4], [2, 21, 6]]},{"e":"text","t":" and a "},{"e":"u/","t":"username"}]},{"e":"list","o":true,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]},{"e":"list","o":false,"c":[{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"First item"}]}]},{"e":"li","c":[{"e":"par","c":[{"e":"text","t":"Second item"}]}]}]}]}]}]}]}]}````````````````````````````````

```````````````````````````````` example
    function test() {
      console.log("notice the blank line before this function?");
    }
.
{"document":[{"e":"code","c":[{"e":"raw","t":"function test() {"},{"e":"raw","t":"  console.log(&quot;notice the blank line before this function?&quot;);"},{"e":"raw","t":"}"},{"e":"raw","t":""}]}]}````````````````````````````````

Say I have many formats nested in one format range. We would want to keep that 
overall format through the whole thing, while also getting rid of the old format
each time we went on.

```````````````````````````````` example
*__bold__ ~underline~ ~~strikethrough~~*
.
{"document":[{"e":"par","c":[{"e":"text","t":"bold underline strikethrough","f":[[3, 0, 4], [2, 4, 1], [6, 5, 9], [2, 14, 1], [10, 15, 13]]}]}]}````````````````````````````````

In the case that we have two of the same styles nested within one another we want
the ranges to all be the same. This will likely only result from the legacy client.

```````````````````````````````` example
**This is some __bold__ text.**
.
{"document":[{"e":"par","c":[{"e":"text","t":"This is some bold text.","f":[[1, 0, 23]]}]}]}````````````````````````````````

```````````````````````````````` example
foo^^^bar
.
{"document":[{"e":"par","c":[{"e":"text","t":"foobar","f":[[32, 3, 3]]}]}]}````````````````````````````````

Lets try the same thing with links

```````````````````````````````` example
[**This is some __bold__ text.**](www.reddit.com)
.
{"document":[{"e":"par","c":[{"e":"link","u":"www.reddit.com","t":"This is some bold text.","f":[[1, 0, 23]]}]}]}````````````````````````````````

Now we also allow images with captions for our parser. An exclamation point allows us to point towards our image using the format 
![alt](/mediaid "caption")

```````````````````````````````` example
![gif](/mediaid "animated gif")
![img](/mediaid "image")
.
{"document":[{"e":"par","c":[{"e":"gif","id":"/mediaid","c":"animated gif"},{"e":"img","id":"/mediaid","c":"image"}]}]}````````````````````````````````

Or without captions

```````````````````````````````` example
![gif](/mediaid)
![img](/mediaid)
.
{"document":[{"e":"par","c":[{"e":"gif","id":"/mediaid"},{"e":"img","id":"/mediaid"}]}]}````````````````````````````````


```````````````````````````````` example
Raw "quotes", &ampersands, and <lt & gt> should be escaped.
.
{"document":[{"e":"par","c":[{"e":"text","t":"Raw &quot;quotes&quot;, &amp;ampersands, and &lt;lt &amp; gt&gt; should be escaped."}]}]}````````````````````````````````

```````````````````````````````` example
HTML entities like &amp; &quot; &lt; and &gt; should not be escaped, unless they are malformed like &amp or &quot".
.
{"document":[{"e":"par","c":[{"e":"text","t":"HTML entities like &amp; &quot; &lt; and &gt; should not be escaped, unless they are malformed like &amp;amp or &amp;quot&quot;."}]}]}````````````````````````````````

```````````````````````````````` example
Escaping to HTML entities like & and " shouldn't impact format ranges like **this** or ~~*this*~~.
.
{"document":[{"e":"par","c":[{"e":"text","t":"Escaping to HTML entities like &amp; and &quot; shouldn't impact format ranges like this or this.","f":[[1, 84, 4], [10, 92, 4]]}]}]}````````````````````````````````

