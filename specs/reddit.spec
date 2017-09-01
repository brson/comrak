cargo run -- --spec specs/reddit.spec
# Reddit Specific Markdown 

`18-Nov-2011`: Updated to include the [latest markdown changes.](http://www.reddit.com/r/changelog/comments/mg1j6/reddit_change_new_markdown_interpreter/)

####Contents

>1. Basic text formatting (*Italics*, **Bold**, ~~Strikethrough~~, Super^script, `inline code`, Quoting)
>2. Linking
>3. Line Breaks & Paragraphs
>4. Lists
>5. Tables
>6. Block Code
>7. Headlines and Horizontals

#### 1. Basic Text Formatting


> **Italics** are created using either a single asterisk (\*) or single underscore (\_).

>Example:

```````````````````````````````` example
This is *italic text*, this is also _italic text_.
.
<p>This is <em>italic text</em>, this is also <em>italic text</em>.</p>
````````````````````````````````

>>This is \*italic text\*, this is also \_italic text\_.

>becomes:

>>This is *italic text*, this is also _italic text_.

#

```````````````````````````````` example
This is **bold text**, this is also __bold text__.
.
<p>This is <strong>bold text</strong>, this is also <strong>bold text</strong>.</p>
````````````````````````````````

>**Bold text** is created with double asterisks (\*\*) or double underscores (\_\_).

>Example:

>>This is \*\*bold text\*\*, this is also \_\_bold text\_\_.

>becomes:

>>This is **bold text**, this is also __bold text__.

#

>**Strikethrough text** is created using a double tilde (`~~`).

>Example:

>>This is `~~`strikethrough text`~~`.

>becomes:

>>This is ~~strikethrough~~ text.


```````````````````````````````` example
This is ~~strikethrough~~ text.
.
<p>This is <del>strikethrough</del> text.</p>
````````````````````````````````

#

>**Superscript text** is created using the carot (`^`).

>Example:

>> This sentence contains super`^`script.

>becomes:

>> This sentence contains super^script.

```````````````````````````````` example
This sentence contains super^script.
.
<p>This sentence contains super<sup>script</sup>.</p>
````````````````````````````````

>Note that you cannot leave space before the carot, and there is no *closing tag*.

>>Superscript can also be stacked^like^this.

#

>**inline code** (monospaced text) is created using the backtick ([grave accents](http://en.wikipedia.org/wiki/Grave_accent)) (\`).

>Example:

>> This sentence contains inline code: `` \` ``javascript:alert("hello world");`` ` ``

>becomes:

>> This sentence contains inline code: `javascript:alert("hello world");`

```````````````````````````````` example
This sentence contains inline code: `javascript:alert("hello world");`
.
<p>This sentence contains inline code: <code>javascript:alert(&quot;hello world&quot;);</code></p>
````````````````````````````````

#

>**Quoting** is achieved by starting a line with an Angle Bracket (>)

>Example:

>>\>Here's a quote.

>>\>Another paragraph in the same quote.
>>\>\>A nested quote.

>>\>Back to a single quote.

>>And finally some unquoted text.

>becomes:

>>>Here's a quote.

>>>Another paragraph in the same quote.
>>>>A nested quote.

>>>Back to a single quote.

>>And finally some unquoted text.


#

>To **remove formatting** you will need to use a Backslash (\\)

>Example:

>>This sentence escapes \\\*italic text\\\* and \\\*\\\*bold text\\\*\\\*.

>becomes:

>>This sentence escapes \*italic text\* and \*\*bold text\*\*.


```````````````````````````````` example
This sentence escapes \*italic text\* and \*\*bold text\*\*.
.
<p>This sentence escapes *italic text* and **bold text**.</p>
````````````````````````````````

####2\. Linking

>**Creating a link**

>Example:

>> \[Reddit\]\(`http://reddit.com`\)

>becomes:

>> [Reddit](http://reddit.com)

```````````````````````````````` example
[Reddit](http://reddit.com)
.
<p><a href="http://reddit.com">Reddit</a></p>
````````````````````````````````

>You cannot begin a link with "www", it must begin with one of the following URL schemes:

>>* http://
* https://
* ftp://
* mailto:
* steam://
* irc://
* news://
* mumble://
* ssh://

#

>You can also provide **title text** for links:

>> \[Reddit\]\(`http://reddit.com` "what's new online!"\).

>becomes:

>> [Reddit](http://reddit.com "what's new online!") ← (*hover*!)

```````````````````````````````` example
[Reddit](http://reddit.com "what's new online!")
.
<p><a href="http://reddit.com" title="what's new online!">Reddit</a></p>
````````````````````````````````

>Title text can be used to hide **spoilers**:

>> `[spoiler](/s"The spoiler text goes here")`

>becomes:

>> [spoiler](/s"The spoiler text goes here") ← (*hover*!)


#

>Reddit now recognises when you want to **link to a subreddit**.

>Example:

>>This is a shameless plug for \/r/BritishTV!

>becomes:

>>This is a shameless plug for /r/BritishTV!

```````````````````````````````` example
This is a shameless plug for /r/BritishTV!
.
<p>This is a shameless plug for <a href="/r/BritishTV" title="/r/BritishTV">/r/BritishTV</a>!</p>
````````````````````````````````

#

>If a URL contains brackets you will need to escape these.

>Example without escaping:

>>`[Cube](http://en.wikipedia.org/wiki/Cube_(film))`

>becomes:

>>[Cube](http://en.wikipedia.org/wiki/Cube_(film)) ← (*note the surplus bracket*!)

>Example with escaping:

>>`[Cube](http://en.wikipedia.org/wiki/Cube_(film\))`

>becomes:

>>[Cube](http://en.wikipedia.org/wiki/Cube_(film\)) ← (*no surplus bracket*!)

```````````````````````````````` example
[Cube](http://en.wikipedia.org/wiki/Cube_(film))
.
<p><a href="http://en.wikipedia.org/wiki/Cube_(film)">Cube</a></p>
````````````````````````````````

####3\. Line Breaks & Paragraphs

>**Line breaks** in comments are achieved by adding four spaces (*shown using* ░) to the end of the line. Simply hitting return (*shown using* ↵) will not work.

>Example:

>>First line↵
>>Second line

>becomes:

>>First line Second line

>but:

>>First line░░░░↵
>>Second line

>becomes:

>>First line
>>Second line

```````````````````````````````` example
First line    
Second line
.
<p>First line<br />
Second line</p>
````````````````````````````````

#

>**Paragraphs** are formed when you hit return (*shown using* ↵) twice.

>>First Paragraph↵
>>↵
>>Second Paragraph

>becomes:

>>First Paragraph

>>Second Paragraph

####4\. Lists

>To create **Unordered Lists** each item should begin with either an asterisk (\*), plus sign (\+) or minus sign (\-).

>Example:

>>\* Item 1
>>\+ Item 2
>>\- Item 3

>becomes:

>>* Item 1
>>+ Item 2
>>- Item 3

```````````````````````````````` example
* Item 1
+ Item 2
- Item 3
.
<ul>
<li>Item 1</li>
</ul>
<ul>
<li>Item 2</li>
</ul>
<ul>
<li>Item 3</li>
</ul>
````````````````````````````````

#

>**Ordered Lists** are created with a number and period. It doesn't matter which number you start with, as markdown will always start with 1.

>Example:

>>3\. Item 1
>>2\. Item 2
>>1\. Item 3

>becomes:

>>3. Item 1
>>2. Item 2
>>1. Item 3

```````````````````````````````` example
3. Item 1
2. Item 2
1. Item 3
.
<ol start="3">
<li>Item 1</li>
<li>Item 2</li>
<li>Item 3</li>
</ol>
````````````````````````````````
#

>The markup for **Nested Lists** has changed slightly:

>Example:

>>1\. This is Item 1
>>2\.
>>░░░░1\. This is Item 2.1
>>░░░░2\. This is Item 2.2
>>3\. This is Item 3
>>4\. This is Item 4

>becomes:

>>1. This is Item 1
>>2.
>>    1. This is Item 2.1
>>    2. This is Item 2.2
>>3. This is Item 3
>>4. This is Item 4

```````````````````````````````` example
1. This is Item 1
2.
    1. This is Item 2.1
    2. This is Item 2.2
3. This is Item 3
4. This is Item 4
.
<ol>
<li>This is Item 1</li>
<li>
<ol>
<li>This is Item 2.1</li>
<li>This is Item 2.2</li>
</ol>
</li>
<li>This is Item 3</li>
<li>This is Item 4</li>
</ol>
````````````````````````````````
#

>Lists should be clear of any text in the line immediately above and below, the same as making a new paragraph:

>>This is the **wrong way to make a list**
>>1. lorem
>>2. ispum
>>reddit doesn't realize it should listify...

>becomes:

>>This is the **wrong way to make a list**
>>1. lorem
>>2. ispum
>>reddit doesn't realize it should listify...

>Place lists in their own paragraph:

>>This is the **correct way to make a list**

>>1. lorem
>>2. ispum

>>reddit realizes it should listify!

>**Paragraphs in Lists** and **Nested lists using a combination of ordered and unordered lists**, are no longer supported.

####5\. Tables

>**Tables** are created using pipes (|):

>Example

>>
[](ftp://grrr.net)Left align | Center align | Right align
[](ftp://grrr.net):--|:--:|--:
This | This | This
column | column | column
will | will | will
be | be |  be
left |  center | right
aligned | aligned   | aligned

>becomes:

>>
 Left align | Center align | Right align
:--|:--:|--:
 This       |     This     |        This
 column     |    column    |      column
 will       |     will     |        will
 be         |      be      |          be
 left       |    center    |       right
 aligned    |    aligned   |     aligned

>Note that by default the first row is always **bolded**.

```````````````````````````````` example
 Left align | Center align | Right align
:--|:--:|--:
 This       |     This     |        This
 column     |    column    |      column
 will       |     will     |        will
 be         |      be      |          be
 left       |    center    |       right
 aligned    |    aligned   |     aligned
.
<table>
<thead>
<tr>
<th align="left">Left align</th>
<th align="center">Center align</th>
<th align="right">Right align</th>
</tr>
</thead>
<tbody>
<tr>
<td align="left">This</td>
<td align="center">This</td>
<td align="right">This</td>
</tr>
<tr>
<td align="left">column</td>
<td align="center">column</td>
<td align="right">column</td>
</tr>
<tr>
<td align="left">will</td>
<td align="center">will</td>
<td align="right">will</td>
</tr>
<tr>
<td align="left">be</td>
<td align="center">be</td>
<td align="right">be</td>
</tr>
<tr>
<td align="left">left</td>
<td align="center">center</td>
<td align="right">right</td>
</tr>
<tr>
<td align="left">aligned</td>
<td align="center">aligned</td>
<td align="right">aligned</td>
</tr></tbody></table>
````````````````````````````````

>**Column Alignment** is determined by the second row.

>>Use "**:--:**" for centre aligned text, "**--:**" for right, and "**:--**" for left.

> You can also leave the top row empty, as long as you have the correct amount of pipes:

>>
[](ftp://grr.net)||
[](ftp://grrr.net):--|:--:|--:
the|top|row
is|now|empty

>becomes

>>
  |  |
:--|:--:|--:
the|top|row
is|now|empty

####6\. Block code

>Displaying **block code**, without formatting and in monospaced font, is as simple as starting the line with four spaces (*shown using* ░).

>Example:

>>░░░░line of code
>>░░░░░░░░line of code
>>░░░░░░░░░░░░line of code
>>░░░░░░░░line of code
>>░░░░line of code

>becomes:

>>`    line of code`
>>`        line of code`
>>`            line of code`
>>`        line of code`
>>`    line of code`

```````````````````````````````` example
`    line of code`
`        line of code`
`            line of code`
`        line of code`
`    line of code`
.
<p><code>    line of code</code>
<code>        line of code</code>
<code>            line of code</code>
<code>        line of code</code>
<code>    line of code</code></p>
````````````````````````````````

####7\. Headlines & Horizonal Rules

>**Headline text** can be created by using a number of hashes (#) corresponding to the tag you want. Headline tags will format all text until it encounters a Line Break or new Paragraph.

>>\# Headline 1
>>\#\# Headline 2
>>\#\#\# Headline 3

>becomes:

>>#Headline 1
>>##Headline 2
>>###Headline 3

```````````````````````````````` example
#Headline 1
##Headline 2
###Headline 3
.
<h1>Headline 1</h1>
<h2>Headline 2</h2>
<h3>Headline 3</h3>
````````````````````````````````

>`NOTE`: Markdown supports up to six headline tags, but only the first three have default formatting.

#

>To create a **Horizontal Rule**, simply add three asterisks (\*) to an empty line.

>>\*\*\*

>becomes:

>>***

```````````````````````````````` example
***
.
<hr />
````````````````````````````````
