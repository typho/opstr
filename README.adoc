README
======

author::
  tajpulo
version::
  0.9.0

What is it about?
-----------------

As a software developer, I often need to look at strings and apply operations to them.
I frequently use python on the commandline or resort to client-side web applications.
But the operations are always the same and should be accessible with one CLI call.

I built opstr, so you can throw a bunch of strings in and get the result of various operations out.
Or you specify an operation and get a predictable result.

Why should I use it?
--------------------

To apply operations to strings.

Who should use it?
------------------

Anyone working with text strings (in the Unicode sense: as sequence of codepoints).

How to install
--------------

Install me via crates.io (TODO not yet!):

[source]
opstr = "1.0"

How to run
----------

1. Go to https://github.com/typho/opstr
2. Click on the Releases link
3. Scroll down, choose the download appropriate for your platform
4. Once the download has finished, extract the files of the tar-gz archive
5. Add executable rights to the file of your platform
6. Run the executable opstr on the command line

Adding your own function
------------------------

If you have a new function to implement …

1. Decide upon a function NAME
2. Create the file src/ops/NAME.rs (with underscores instead of hyphens in the basename)
4. Add the function to src/ops/mod.rs
5. The file must implement the [Op trait](src/ops/traits.rs)

Compatibility guarantees
------------------------

* We follow semver semantics: Breaking the API requires a major version update. Changing the behavior of functions or extending non-exhaustive API elements requires a minor version update. Security bugfixes or severe issues (if they can be fixed in a backwards-compatible manner) are fixed with a patch release.
* The op names are fixed since the 1.0 release. The ops will never disappear. The ops will always implement what they describe.
* The software license cannot change.

Release management
------------------

What to pay attention to before creating a new release:

1. Update link:https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt[UnicodeData]
2. Update link:https://www.unicode.org/Public/UCD/latest/ucd/SpecialCasing.txt[SpecialCasing]
3. Review which crate versions to update
4. verify whether you plan a major/minor/patch release
5. update the version number in README.adoc and main.rs

Future plans
------------

Until the 1.0 release, I want to …

* CLI
** review that `--list-ops` w/ and w/o makes sense and whether the arguments react appropriately to existing arguments
** review naming scheme for functions, esp. locale-dependent versus ASCII, char versus codepoint, plural vs singular (e.g. whitespace[s])
* ops concept
** icu4x dependency?
** introduce regex ops
* ops
** review is-charset-id op
** implement function `combine`: e.g. ["combining", "strike-through", text] … (relation to function `combining-codepoint-list`?) … original spec: X: add X combiner to all codepoints where X in {bold, italic, cursive, sans-serif, strike-through, underline, slash-through, double-struck, monospace, Fraktur, upside-down, bubble text, square text, small-caps, fullwidth, zigzag-above, diamond-enclosed, redact, circle-backslash}, c.f. https://yaytext.com/square-text/
** op sort: Unicode locale-dependent sorting
** op sort-lexicographically: rather simple sorting with Unicode codepoints
** op cmp-lexicographic: lexicographic comparison of two strings
** op cmp of two strings (is cmp in rust lexicographic? if so, ignore this)
** op camelcase: locale-dependent casing Unicode operation
** op titlecase: locale-dependent casing Unicode operation
** op lowercase-localized: locale-dependent Unicode casing operation
** op uppercase-localized: locale-dependent Unicode casing operation
** op replace-limited-from-start, replace-limited-from-end
** op byte-index-of-first-occurence, byte-index-of-last-occurence
** op split-limited-from-start, split-limited-from-end, split-…-with-separator
** op split-lines-with-offsets: split_by_linebreaks but also return the UTF-8 indices where line breaks happened
** op split-with-offsets: split but also return the UTF-8 indices where line breaks happened
** op split-by-whitespaces: add inclusive versions which keep the separator in the elements?
** op whitespace-lines-to-empty: convert lines filled with only whitespace to empty lines
** op split-at-codepoint-index
** op slice lines by maximum length (1. find center by midpoint of first ANSI highlight and last clear, 2. find better center if length exceeds by midpoint of first highlight and first clear, 3. trim whitespace optionally to achieve length, 4. print characters around center, wrap by "[…] " and " […]")
** op lines: simply split into lines
** op per line: remove leading/trailing whitespace, add final empty line, merge multiple empty lines to one empty line
** op line-start-byte-indices: return the list of byte indices where a new line starts
** op line-at-line-number: filter lines by index: return the n-th line where n is in 1..infty
** op line-at-index: filter lines by index: return the n-th line where n can be pos, 0, or neg
** op lines-with-minimum-length (lines len): filter lines by minimum length
** op lines-with-maximum-length (lines len): filter lines by maximum length
** op lines-by-range (lines start end): returns lines with indices in zero-based inclusive-exclusive range
** op lines-by-linenumber-range (lines start end): returns lines with indices in one-based inclusive-inclusive range
** op list of writing systems
** op split-by-whitespace-nth: return the nth item of the list
** op take file content, apply delimiter e.g. "\n--\n" and return segments
** op take file content, fetch recursive structure e.g. "(" and ")" or "\begin{…}" and "\end{…}" and return segments
** op substring-byte-indices: return list of byte indices where a given substring occurs
** op substring-codepoint-indices: return list of codepoint indices where a given substring occurs
** op prefix-line-number (lines [opt. separator]): attach line number (or line number and separator) before each line
** op return lines N–M … so given line numbers, return the corresponding range of lines
* final review of priorities

Source Code
-----------

The source code is available at link:https://github.com/typho/opstr[Github].

License
-------

See link:LICENSE[the LICENSE file] (Hint: MIT license).

Changelog
---------

0.7.0:: first public release
1.0.0:: uses Unicode Version 15.0, release with backwards compatibility guarantees

Issues
------

Please report any issues on the link:https://github.com/typho/opstr/issues[Github issues page].
