# rssed

a cli rss feed reader with what you've always wanted: an ed(1) style REPL!

N.B. for our purposes, a "line" is an index

line addressing:

```
.    - the current line
$    - the last line
,    - range from the first line to the last line
;    - range from the current line to the last line
n    - the nth line
x,y  - range from the xth line to the yth line
```

commands:

```
we define [.,.] as a line address accepting all above forms
we define [n] as a line address accepting only the specified form

a <url>   - add an rss feed to the session
[.,.]d    - delete the feeds from the session at the specified addresses
g         - update the feeds in the session
h         - show this help text
[n]       - change the current line
[n]o      - print the nth item in a feed at the current line
[.,.]p    - print the feed titles at the specified addresses
q         - quit the session
```