cargo run -- --rtjson --spec specs/rtjson/bugs.spec
# Tests for Known bugs

This file introduces test to ensure that fixes to ceratin bugs are working properly.

This should be used with rtjson.

```````````````````````````````` example
nonoe www.reddit.com /r/or r/or either/or
.
{"document":[{"c":[{"e":"text","t":"nonoe www.reddit.com "},{"e":"r/","t":"or"},{"e":"text","t":" "},{"e":"r/","t":"or"},{"e":"text","t":" either/or"}],"e":"par"}]}````````````````````````````````

We also need to check links at the beginning 

```````````````````````````````` example
www.reddit.com nonoe /r/or r/or either/or
.
{"document":[{"c":[{"e":"text","t":"www.reddit.com nonoe "},{"e":"r/","t":"or"},{"e":"text","t":" "},{"e":"r/","t":"or"},{"e":"text","t":" either/or"}],"e":"par"}]}````````````````````````````````

...and end of lines.

```````````````````````````````` example
nonoe /r/or r/or either/or www.reddit.com
.
{"document":[{"c":[{"e":"text","t":"nonoe "},{"e":"r/","t":"or"},{"e":"text","t":" "},{"e":"r/","t":"or"},{"e":"text","t":" either/or www.reddit.com"}],"e":"par"}]}````````````````````````````````

We should also make sure that user redditlinks are being covered

```````````````````````````````` example
nonoe /u/or u/or eu/au
.
{"document":[{"c":[{"e":"text","t":"nonoe "},{"e":"u/","t":"or"},{"e":"text","t":" "},{"e":"u/","t":"or"},{"e":"text","t":" eu/au"}],"e":"par"}]}````````````````````````````````

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
{"document":[{"c":[{"e":"text","f":[[64,0,9],[4,10,9]],"t":"monospace underline &lt;usr/&gt;"}],"e":"par"}]}````````````````````````````````

The test above account for known bugs and fixes.

```````````````````````````````` example
 a。u/reddit
u/reddit
/u/reddit
.
{"document":[{"c":[{"e":"text","t":"a。u/reddit"},{"e":"u/","t":"reddit"},{"e":"u/","t":"reddit"}],"e":"par"}]}````````````````````````````````

The redditlink should always be rendered if it starts with a slash.

```````````````````````````````` example
。/u/reddit
。//u/reddit
.
{"document":[{"c":[{"e":"text","t":"。/"},{"e":"u/","t":"reddit"},{"e":"text","t":"。/"},{"e":"u/","t":"reddit"}],"e":"par"}]}````````````````````````````````

There was a bug where we were getting a panic on the malformed strings

```````````````````````````````` example
[If we don't end correctly](/reddit.com "Then the test shouldn't break"
.
{"document":[{"c":[{"e":"text","t":"[If we don't end correctly](/reddit.com &quot;Then the test shouldn't break&quot;"}],"e":"par"}]}````````````````````````````````

There is also reddit specific conventions surrounding superscript.

```````````````````````````````` example
^Single will only have that work where as ^(In parens will include the whole parens).
.
{"document":[{"c":[{"e":"text","f":[[32,0,6],[32,41,39]],"t":"Single will only have that work where as In parens will include the whole parens."}],"e":"par"}]}````````````````````````````````

We must also make sure that non whitespace characters are accounted for

```````````````````````````````` example
^。here and here
.
{"document":[{"c":[{"e":"text","f":[[32,0,7]],"t":"。here and here"}],"e":"par"}]}````````````````````````````````

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
