cargo run -- --rtjson --spec specs/rtjson/bugs.spec
# Tests for Known bugs

This file introduces test to ensure that fixes to ceratin bugs are working properly.

This should be used with rtjson.

```````````````````````````````` example
nonoe www.reddit.com /r/or r/or either/or
.
{"document":[{"e":"par","c":[{"e":"text","t":"nonoe www.reddit.com "},{"e":"r/","t":"or"},{"e":"text","t":" "},{"e":"r/","t":"or"},{"e":"text","t":" either/or"}]}]}````````````````````````````````

We also need to check links at the beginning 

```````````````````````````````` example
www.reddit.com nonoe /r/or r/or either/or
.
{"document":[{"e":"par","c":[{"e":"text","t":"www.reddit.com nonoe "},{"e":"r/","t":"or"},{"e":"text","t":" "},{"e":"r/","t":"or"},{"e":"text","t":" either/or"}]}]}````````````````````````````````

...and end of lines.

```````````````````````````````` example
nonoe /r/or r/or either/or www.reddit.com
.
{"document":[{"e":"par","c":[{"e":"text","t":"nonoe "},{"e":"r/","t":"or"},{"e":"text","t":" "},{"e":"r/","t":"or"},{"e":"text","t":" either/or www.reddit.com"}]}]}````````````````````````````````

We should also make sure that user redditlinks are being covered

```````````````````````````````` example
nonoe /u/or u/or eu/au
.
{"document":[{"e":"par","c":[{"e":"text","t":"nonoe "},{"e":"u/","t":"or"},{"e":"text","t":" "},{"e":"u/","t":"or"},{"e":"text","t":" eu/au"}]}]}````````````````````````````````

We have to make sure that nested styles get the proper rendering

```````````````````````````````` example
~curly**curly and bold**curly~
.
{"document":[{"e":"par","c":[{"e":"text","t":"curlycurly and boldcurly","f":[[4, 0, 5], [5, 5, 14], [4, 19, 5]]}]}]}````````````````````````````````

The below does not work and should be looked into 

```````````````````````````````` example
~curly***curly and bold***curly~
.
{"document":[{"e":"par","c":[{"e":"text","t":"curly***curly and bold***curly","f":[[4, 0, 30]]}]}]}````````````````````````````````

We should also show code and underline when we are trying ot render content

```````````````````````````````` example
`monospace` ~underline~ <usr/>
.
{"document":[{"e":"par","c":[{"e":"text","t":"monospace underline &lt;usr/&gt;","f":[[64, 0, 9], [4, 10, 9]]}]}]}````````````````````````````````

The test above account for known bugs and fixes.

```````````````````````````````` example
 a。u/reddit
u/reddit
/u/reddit
.
{"document":[{"e":"par","c":[{"e":"text","t":"a。u/reddit"},{"e":"u/","t":"reddit"},{"e":"u/","t":"reddit"}]}]}````````````````````````````````

The redditlink should always be rendered if it starts with a slash.

```````````````````````````````` example
。/u/reddit
。//u/reddit
.
{"document":[{"e":"par","c":[{"e":"text","t":"。/"},{"e":"u/","t":"reddit"},{"e":"text","t":"。/"},{"e":"u/","t":"reddit"}]}]}````````````````````````````````

There was a bug where we were getting a panic on the malformed strings

```````````````````````````````` example
[If we don't end correctly](/reddit.com "Then the test shouldn't break"
.
{"document":[{"e":"par","c":[{"e":"text","t":"[If we don't end correctly](/reddit.com &quot;Then the test shouldn't break&quot;"}]}]}````````````````````````````````

There is also reddit specific conventions surrounding superscript.

```````````````````````````````` example
^Single will only have that work where as ^(In parens will include the whole parens).
.
{"document":[{"e":"par","c":[{"e":"text","t":"Single will only have that work where as In parens will include the whole parens.","f":[[32, 0, 6], [32, 41, 39]]}]}]}````````````````````````````````

We must also make sure that non whitespace characters are accounted for

```````````````````````````````` example
^。here and here
.
{"document":[{"e":"par","c":[{"e":"text","t":"。here and here","f":[[32, 0, 7]]}]}]}````````````````````````````````
