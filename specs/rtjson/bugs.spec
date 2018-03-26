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

These test are for the spoiler tag when they are not completed.

```````````````````````````````` example
This >!is an opening spoiler tag with no closer.

This is a closing spoiler tag!< with no opener.

This is a >!regular old spoiler!< tag.
.
{"document":[{"c":[{"e":"text","t":"This >!is an opening spoiler tag with no closer."}],"e":"par"},{"c":[{"e":"text","t":"This is a closing spoiler tag!< with no opener."}],"e":"par"},{"c":[{"e":"text","t":"This is a "},{"c":[{"e":"text","t":"regular old spoiler"}],"e":"spoilertext"},{"e":"text","t":" tag."}],"e":"par"}]}````````````````````````````````

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

Reddit quirk - headers don't need spaces between hashes and headertext. CREATE-1363

```````````````````````````````` example
#Bleep
Bloop
.
{"document": [{"c": [{"e": "raw", "t": "Bleep"}], "e": "h", "l": 1}, {"c": [{"e": "text", "t": "Bloop"}], "e": "par"}]}````````````````````````````````

... and that makes tables on subsequent lines parse as tables

```````````````````````````````` example
###Header
A|B|C|D
:--|:--|:-:|:-:
a|b|c|d
.
{"document": [{"c": [{"e": "raw", "t": "Header"}], "e": "h", "l": 3}, {"h": [{"a": "L", "c": [{"e": "text", "t": "A"}]}, {"a": "L", "c": [{"e": "text", "t": "B"}]}, {"a": "C", "c": [{"e": "text", "t": "C"}]}, {"a": "C", "c": [{"e": "text", "t": "D"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}, {"c": [{"e": "text", "t": "c"}]}, {"c": [{"e": "text", "t": "d"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
##
#
### ###
.
{"document": [{"e": "h", "l": 2}, {"e": "h", "l": 1}, {"e": "h", "l": 3}]}````````````````````````````````

```````````````````````````````` example
##
#
###  ###
.
{"document": [{"e": "h", "l": 2}, {"e": "h", "l": 1}, {"e": "h", "l": 3}]}````````````````````````````````

Tests for compatibility with snudown tables with trailing 'junk' in rows. CREATE-1363

```````````````````````````````` example
|A|B|C|
|-|-|-|bleep
|a|b|c|
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}, {"a": "", "c": [{"e": "text", "t": "C"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}, {"c": [{"e": "text", "t": "c"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
|A|B|C|
|-|-|-|
|a|b|c|bloop
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}, {"a": "", "c": [{"e": "text", "t": "C"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}, {"c": [{"e": "text", "t": "c"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
|A|B|C|
|-|-|-|bleep
|a|b|c|bloop
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}, {"a": "", "c": [{"e": "text", "t": "C"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}, {"c": [{"e": "text", "t": "c"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
|A|B|C|
|-|-|-| bleep
|a|b|c| bloop
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}, {"a": "", "c": [{"e": "text", "t": "C"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}, {"c": [{"e": "text", "t": "c"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
|A|B|C
-|-|-|a
|a|b|c
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}, {"a": "", "c": [{"e": "text", "t": "C"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}, {"c": [{"e": "text", "t": "c"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
A|B|C|
-|-|-|bleep|bloop|
a|b|c|
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}, {"a": "", "c": [{"e": "text", "t": "C"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}, {"c": [{"e": "text", "t": "c"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
A|B|C|
-|-|-|
a|b|c|bleep|bloop|
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}, {"a": "", "c": [{"e": "text", "t": "C"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}, {"c": [{"e": "text", "t": "c"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
|A|B|C|
|-|-|-|bleep
|a|b|
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}, {"a": "", "c": [{"e": "text", "t": "C"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}, {"c": []}]], "e": "table"}]}````````````````````````````````

Finally a somewhat reduced real-world case of trailing junk in the marker row

```````````````````````````````` example
#Rotational Players

| Name | Position | Year | Height / Weight | 24/7 Rating |Highlights|
|:-------:|:-------:|:-----:|:----------------:|:------------:|:-----------:|+
| Daniel McMillian | OLB/ILB | Senior | 6'1 / 230 | **** | [Here](https://www.youtube.com/watch?v=1sIHfGNutv8) |
| Matt Rolin | ILB/OLB | Junior - Redshirt | 6'4 / 225 | **** | [Here](https://www.youtube.com/watch?v=kRtlakveW34) |

Unfortunately we have almost no veteran depth this season.
.
{"document": [{"c": [{"e": "raw", "t": "Rotational Players"}], "e": "h", "l": 1}, {"h": [{"a": "C", "c": [{"e": "text", "t": "Name"}]}, {"a": "C", "c": [{"e": "text", "t": "Position"}]}, {"a": "C", "c": [{"e": "text", "t": "Year"}]}, {"a": "C", "c": [{"e": "text", "t": "Height / Weight"}]}, {"a": "C", "c": [{"e": "text", "t": "24/7 Rating"}]}, {"a": "C", "c": [{"e": "text", "t": "Highlights"}]}], "c": [[{"c": [{"e": "text", "t": "Daniel McMillian"}]}, {"c": [{"e": "text", "t": "OLB/ILB"}]}, {"c": [{"e": "text", "t": "Senior"}]}, {"c": [{"e": "text", "t": "6'1 / 230"}]}, {"c": [{"e": "text", "t": "****"}]}, {"c": [{"u": "https://www.youtube.com/watch?v=1sIHfGNutv8", "e": "link", "t": "Here"}]}], [{"c": [{"e": "text", "t": "Matt Rolin"}]}, {"c": [{"e": "text", "t": "ILB/OLB"}]}, {"c": [{"e": "text", "t": "Junior - Redshirt"}]}, {"c": [{"e": "text", "t": "6'4 / 225"}]}, {"c": [{"e": "text", "t": "****"}]}, {"c": [{"u": "https://www.youtube.com/watch?v=kRtlakveW34", "e": "link", "t": "Here"}]}]], "e": "table"}, {"c": [{"e": "text", "t": "Unfortunately we have almost no veteran depth this season."}], "e": "par"}]}````````````````````````````````

Tests that table rows take precedence over lists. CREATE-1363

```````````````````````````````` example
A|B
-|-
-|b
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}], "c": [[{"c": [{"e": "text", "t": "-"}]}, {"c": [{"e": "text", "t": "b"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
A|B
-|-
*|b
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}], "c": [[{"c": [{"e": "text", "t": "*"}]}, {"c": [{"e": "text", "t": "b"}]}]], "e": "table"}]}````````````````````````````````

```````````````````````````````` example
A|B
-|-
 - | b
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}], "c": [[{"c": [{"e": "text", "t": "-"}]}, {"c": [{"e": "text", "t": "b"}]}]], "e": "table"}]}````````````````````````````````

Lines have to have at least one pipe to be parsed as rows instead of lists.
The following contains two list items.

```````````````````````````````` example
A|B
-|-
a|b
- 1
- 2
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}]], "e": "table"}, {"c": [{"c": [{"c": [{"e": "text", "t": "1"}], "e": "par"}], "e": "li"}, {"c": [{"c": [{"e": "text", "t": "2"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}]}````````````````````````````````

This though is a table following by a single list item.

```````````````````````````````` example
A|B
-|-
a|b
- 1 |
- 2
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}], [{"c": [{"e": "text", "t": "- 1"}]}, {"c": []}]], "e": "table"}, {"c": [{"c": [{"c": [{"e": "text", "t": "2"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}]}````````````````````````````````

Escaped pipes aren't considered table delimiters, so this one is again two list
items (this differs from snudown, which failed to escape the pipe here).

```````````````````````````````` example
A|B
-|-
a|b
- 1 \|
- 2
.
{"document": [{"h": [{"a": "", "c": [{"e": "text", "t": "A"}]}, {"a": "", "c": [{"e": "text", "t": "B"}]}], "c": [[{"c": [{"e": "text", "t": "a"}]}, {"c": [{"e": "text", "t": "b"}]}]], "e": "table"}, {"c": [{"c": [{"c": [{"e": "text", "t": "1 |"}], "e": "par"}], "e": "li"}, {"c": [{"c": [{"e": "text", "t": "2"}], "e": "par"}], "e": "li"}], "e": "list", "o": false}]}````````````````````````````````
