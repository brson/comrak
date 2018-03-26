cargo run -- --rtjson --spec specs/rtjson/bugs.spec
# Tests for Known bugs

This file introduces test to ensure that fixes to ceratin bugs are working properly.

This should be used with rtjson.

```````````````````````````````` example
nonoe www.reddit.com /r/or r/or either/or
.
{"document":[{"c":[{"e":"text","t":"nonoe "},{"e":"link","t":"www.reddit.com","u":"http://www.reddit.com"},{"e":"text","t":" "},{"e":"r/","l":true,"t":"or"},{"e":"text","t":" "},{"e":"r/","t":"or"},{"e":"text","t":" either/or"}],"e":"par"}]}````````````````````````````````

We also need to check links at the beginning

```````````````````````````````` example
www.reddit.com nonoe /r/or r/or either/or
.
{"document":[{"c":[{"e":"link","t":"www.reddit.com","u":"http://www.reddit.com"},{"e":"text","t":" nonoe "},{"e":"r/","l":true,"t":"or"},{"e":"text","t":" "},{"e":"r/","t":"or"},{"e":"text","t":" either/or"}],"e":"par"}]}````````````````````````````````

...and end of lines.

```````````````````````````````` example
nonoe /r/or r/or either/or www.reddit.com
.
{"document":[{"c":[{"e":"text","t":"nonoe "},{"e":"r/","l":true,"t":"or"},{"e":"text","t":" "},{"e":"r/","t":"or"},{"e":"text","t":" either/or "},{"e":"link","t":"www.reddit.com","u":"http://www.reddit.com"}],"e":"par"}]}````````````````````````````````

We should also make sure that user redditlinks are being covered

```````````````````````````````` example
nonoe /u/or u/or eu/au
.
{"document":[{"c":[{"e":"text","t":"nonoe "},{"e":"u/","l":true,"t":"or"},{"e":"text","t":" "},{"e":"u/","t":"or"},{"e":"text","t":" eu/au"}],"e":"par"}]}````````````````````````````````

We have to make sure that nested styles get the proper rendering

```````````````````````````````` example
~curly**curly and bold**curly~
.
{"document":[{"c":[{"e":"text","f":[[4,0,5],[5,5,14],[4,19,5]],"t":"curlycurly and boldcurly"}],"e":"par"}]}````````````````````````````````

The below does not work and should be looked into

```````````````````````````````` example
~curly***curly and bold***curly~
.
{"document":[{"c":[{"e":"text","f":[[4,0,30]],"t":"curly***curly and bold***curly"}],"e":"par"}]}````````````````````````````````

We should also show code and underline when we are trying ot render content

```````````````````````````````` example
`monospace` ~underline~ <usr/>
.
{"document":[{"c":[{"e":"text","f":[[64,0,9],[4,10,9]],"t":"monospace underline <usr/>"}],"e":"par"}]}````````````````````````````````

The test above account for known bugs and fixes.

```````````````````````````````` example
 a。u/reddit
u/reddit
/u/reddit
.
{"document":[{"c":[{"e":"text","t":"a。u/reddit"},{"e":"u/","t":"reddit"},{"e":"u/","l":true,"t":"reddit"}],"e":"par"}]}````````````````````````````````

The redditlink should always be rendered if it starts with a slash.

```````````````````````````````` example
。/u/reddit
。//u/reddit
.
{"document":[{"c":[{"e":"text","t":"。/"},{"e":"u/","t":"reddit"},{"e":"text","t":"。/"},{"e":"u/","l":true,"t":"reddit"}],"e":"par"}]}````````````````````````````````

There was a bug where we were getting a panic on the malformed strings

```````````````````````````````` example
[If we don't end correctly](/reddit.com "Then the test shouldn't break"
.
{"document":[{"c":[{"e":"text","t":"[If we don't end correctly](/reddit.com \"Then the test shouldn't break\""}],"e":"par"}]}````````````````````````````````

There is also reddit specific conventions surrounding superscript.

```````````````````````````````` example
^Single will only have that work where as ^(In parens will include the whole parens).
.
{"document":[{"c":[{"e":"text","f":[[32,0,6],[32,41,39]],"t":"Single will only have that work where as In parens will include the whole parens."}],"e":"par"}]}````````````````````````````````

We must also make sure that non whitespace characters are accounted for

```````````````````````````````` example
^。here and here
.
{"document":[{"c":[{"e":"text","f":[[32,0,5]],"t":"。here and here"}],"e":"par"}]}````````````````````````````````

We want to have code blocks not include empty lines at the end.

```````````````````````````````` example
    for (var i in arr) {

        console.log(arr[i]);
    }
.
{"document":[{"c":[{"e":"raw","t":"for (var i in arr) {"},{"e":"raw", "t":""},{"e":"raw","t":"    console.log(arr[i]);"},{"e":"raw","t":"}"}],"e":"code"}]}````````````````````````````````

Our tables should show all of their rows and columns


```````````````````````````````` example
|Col 1|Col 2|Col 3|
|:-|:-:|-:|
| | | |
|c1:r2|c2:r2|c3:r2|
| |c2:r3|c3:r3|
|c1:r4| |c3:r4|
.
{"document":[{"c":[[{"c":[]},{"c":[]},{"c":[]}],[{"c":[{"e":"text","t":"c1:r2"}]},{"c":[{"e":"text","t":"c2:r2"}]},{"c":[{"e":"text","t":"c3:r2"}]}],[{"c":[]},{"c":[{"e":"text","t":"c2:r3"}]},{"c":[{"e":"text","t":"c3:r3"}]}],[{"c":[{"e":"text","t":"c1:r4"}]},{"c":[]},{"c":[{"e":"text","t":"c3:r4"}]}]],"e":"table","h":[{"a":"L","c":[{"e":"text","t":"Col 1"}]},{"a":"C","c":[{"e":"text","t":"Col 2"}]},{"a":"R","c":[{"e":"text","t":"Col 3"}]}]}]}````````````````````````````````

List with empty nodes should send back a empty paragraph node.

```````````````````````````````` example
* fdsa
*
* fds
.
{"document":[{"c":[{"c":[{"c":[{"e":"text","t":"fdsa"}],"e":"par"}],"e":"li"},{"c":[{"c":[{"e":"text","t":""}],"e":"par"}],"e":"li"},{"c":[{"c":[{"e":"text","t":"fds"}],"e":"par"}],"e":"li"}],"e":"list","o":false}]}````````````````````````````````

Bug where empty nested nodes were not being rendered correctly.

```````````````````````````````` example
*
  *
.
{"document":[{"c":[{"c":[{"c":[{"e":"text","t":""}],"e":"par"},{"c":[{"c":[{"c":[{"e":"text","t":""}],"e":"par"}],"e":"li"}],"e":"list","o":false}],"e":"li"}],"e":"list","o":false}]}````````````````````````````````

When a username has a hyphen in it we should support it.

```````````````````````````````` example
u/hello-there- hello-there
.
{"document":[{"c":[{"e":"u/","t":"hello-there-"},{"e":"text","t":" hello-there"}],"e":"par"}]}````````````````````````````````

Spoilertext should not turn into block quotes

```````````````````````````````` example
This >!works!<

>!So does this!!<

>!And this.!<

> Finally a regular blockquote
.
{"document":[{"c":[{"e":"text","t":"This "},{"c":[{"e":"text","t":"works"}],"e":"spoilertext"}],"e":"par"},{"c":[{"c":[{"e":"text","t":"So does this!"}],"e":"spoilertext"}],"e":"par"},{"c":[{"c":[{"e":"text","t":"And this."}],"e":"spoilertext"}],"e":"par"},{"c":[{"c":[{"e":"text","t":"Finally a regular blockquote"}],"e":"par"}],"e":"blockquote"}]}````````````````````````````````

Testing for unicode characters with incorrect lengths output.

```````````````````````````````` example
☃*aaa*bbb
.
{"document":[{"c":[{"e":"text","f":[[2,1,3]],"t":"☃aaabbb"}],"e":"par"}]}````````````````````````````````


```````````````````````````````` example
ɛ*aaa*bbb
.
{"document":[{"c":[{"e":"text","f":[[2,1,3]],"t":"ɛaaabbb"}],"e":"par"}]}````````````````````````````````


```````````````````````````````` example
ɛ*aa☃*bbb
.
{"document":[{"c":[{"e":"text","f":[[2,1,3]],"t":"ɛaa☃bbb"}],"e":"par"}]}````````````````````````````````


```````````````````````````````` example
ɛ`aaa`bbb
.
{"document":[{"c":[{"e":"text","f":[[64,1,3]],"t":"ɛaaabbb"}],"e":"par"}]}````````````````````````````````


```````````````````````````````` example
☃`aaa`bbb
.
{"document":[{"c":[{"e":"text","f":[[64,1,3]],"t":"☃aaabbb"}],"e":"par"}]}````````````````````````````````


```````````````````````````````` example
ɛ`aa☃`bbb
.
{"document":[{"c":[{"e":"text","f":[[64,1,3]],"t":"ɛaa☃bbb"}],"e":"par"}]}````````````````````````````````

Test pathological input

```````````````````````````````` example
a*a*a*a*a*a*a*a*a*a*
.
{"document":[{"c":[{"e":"text","f":[[2,1,1],[2,3,1],[2,5,1],[2,7,1],[2,9,1]],"t":"aaaaaaaaaa"}],"e":"par"}]}````````````````````````````````

We want no nodes to be dropped in the case of mixed or nested stylings.

```````````````````````````````` example
*This is a node [with a link](www.reddit.com) and another node.*
.
{"document":[{"c":[{"e":"text","f":[[2,0,15]],"t":"This is a node "},{"e":"link","f":[[2,0,11]],"t":"with a link","u":"www.reddit.com"},{"e":"text","f":[[2,0,18]],"t":" and another node."}],"e":"par"}]}````````````````````````````````

```````````````````````````````` example
*This is a node /u/redditlink and another node.*
.
{"document":[{"c":[{"e":"text","f":[[2,0,15]],"t":"This is a node "},{"e":"u/","l":true,"t":"redditlink"},{"e":"text","f":[[2,0,18]],"t":" and another node."}],"e":"par"}]}````````````````````````````````

```````````````````````````````` example
*This is a node >!a spoiler-node!< and another node.*
.
{"document":[{"c":[{"e":"text","f":[[2,0,15]],"t":"This is a node "},{"c":[{"e":"text","f":[[2,0,14]],"t":"a spoiler-node"}],"e":"spoilertext"},{"e":"text","f":[[2,0,18]],"t":" and another node."}],"e":"par"}]}````````````````````````````````

Autolinking tests

```````````````````````````````` example
http://www.google.com
.
{"document": [{"c": [{"u": "http://www.google.com", "e": "link", "t": "http://www.google.com"}], "e": "par"}]}````````````````````````````````

```````````````````````````````` example
https://www.google.com
.
{"document": [{"c": [{"u": "https://www.google.com", "e": "link", "t": "https://www.google.com"}], "e": "par"}]}````````````````````````````````

```````````````````````````````` example
www.google.com
.
{"document": [{"c": [{"u": "http://www.google.com", "e": "link", "t": "www.google.com"}], "e": "par"}]}````````````````````````````````

This one is checking that a url containing /r/foo isn't mangled in some weird way

```````````````````````````````` example
https://www.reddit.com/r/ModSupport/comments/81dz9w/automod_removing_crossposts/
.
{"document": [{"c": [{"u": "https://www.reddit.com/r/ModSupport/comments/81dz9w/automod_removing_crossposts/", "e": "link", "t": "https://www.reddit.com/r/ModSupport/comments/81dz9w/automod_removing_crossposts/"}], "e": "par"}]}````````````````````````````````

Finally, an example pulled from the wild:

```````````````````````````````` example
As seen here, naked URLs are not being parsed as URLs. If the URL is a Reddit
link, the subreddit is parsed as a clickable link to that subreddit. For
example,
https://www.reddit.com/r/ModSupport/comments/81dz9w/automod_removing_crossposts/

If the link is another link, it is merely displayed as plain text. For example,
https://www.google.com/
.
{"document": [{"c": [{"e": "text", "t": "As seen here, naked URLs are not being parsed as URLs. If the URL is a Redditlink, the subreddit is parsed as a clickable link to that subreddit. Forexample,"}, {"u": "https://www.reddit.com/r/ModSupport/comments/81dz9w/automod_removing_crossposts/", "e": "link", "t": "https://www.reddit.com/r/ModSupport/comments/81dz9w/automod_removing_crossposts/"}], "e": "par"}, {"c": [{"e": "text", "t": "If the link is another link, it is merely displayed as plain text. For example,"}, {"u": "https://www.google.com/", "e": "link", "t": "https://www.google.com/"}], "e": "par"}]}````````````````````````````````

Test a link with % in it.

```````````````````````````````` example
http://sfpublicworks.org/sites/default/files/Broadway%20Chinatown%20Factsheet.pdf
.
{"document":[{"c":[{"e":"link","t":"http://sfpublicworks.org/sites/default/files/Broadway%20Chinatown%20Factsheet.pdf","u":"http://sfpublicworks.org/sites/default/files/Broadway%20Chinatown%20Factsheet.pdf"}],"e":"par"}]}````````````````````````````````
