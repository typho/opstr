# Approach for Unicode/ASCII

We have one generic name. If the user specifies a locale, we need to supply a correct Unicode-compatible result (maybe require a proper `OPSTR_LOCALE_DATAFILE`). If the user specifies no locale, we need to provide a best-effort Unicode-less alternative.

We can also expose the Unicode-less algorithm as additional operation (e.g. `sort` versus `sort-lexicographically`), because a suffix like `lexicographically` indicates that the sorting algorithm does not need/consider Unicode.

# CLI proposals

* Option `--per-line-as-arg N`: run the operation for every line of the file mentioned in argument at index `N` and replace argument `N` with the line content, only works for ops with scalar output? Currently we don't have a concept to merge `Output` instances, so this is difficult to implement

# Proposals for more ops

* implement function `combine`: e.g. ["combining", "strike-through", text] … (relation to function `combining-codepoint-list`?) … original spec: X: add X combiner to all codepoints where X in {bold, italic, cursive, sans-serif, strike-through, underline, slash-through, double-struck, monospace, Fraktur, upside-down, bubble text, square text, small-caps, fullwidth, zigzag-above, diamond-enclosed, redact, circle-backslash}, c.f. https://yaytext.com/square-text/
* op cmp-lexicographic: lexicographic comparison of two strings
* op cmp of two strings (is cmp in rust lexicographic? if so, ignore this)
* op camelcase: locale-dependent casing Unicode operation
* op titlecase: locale-dependent casing Unicode operation
* op lowercase-localized: locale-dependent Unicode casing operation
* op uppercase-localized: locale-dependent Unicode casing operation
* op replace-limited-from-start, replace-limited-from-end
* op byte-index-of-first-occurence, byte-index-of-last-occurence
* op split-limited-from-start, split-limited-from-end, split-…-with-separator
* op split-lines-with-offsets: split_by_linebreaks but also return the UTF-8 indices where line breaks happened
* op split-with-offsets: split but also return the UTF-8 indices where line breaks happened
* op split-by-whitespaces: add inclusive versions which keep the separator in the elements?
* op whitespace-lines-to-empty: convert lines filled with only whitespace to empty lines
* op split-at-codepoint-index
* op slice lines by maximum length (1. find center by midpoint of first ANSI highlight and last clear, 2. find better center if length exceeds by midpoint of first highlight and first clear, 3. trim whitespace optionally to achieve length, 4. print characters around center, wrap by "[…] " and " […]")
* op lines: simply split into lines
* op per line: remove leading/trailing whitespace, add final empty line, merge multiple empty lines to one empty line
* op line-start-byte-indices: return the list of byte indices where a new line starts
* op line-at-line-number: filter lines by index: return the n-th line where n is in 1..infty
* op line-at-index: filter lines by index: return the n-th line where n can be pos, 0, or neg
* op lines-with-minimum-length (lines len): filter lines by minimum length
* op lines-with-maximum-length (lines len): filter lines by maximum length
* op lines-by-range (lines start end): returns lines with indices in zero-based inclusive-exclusive range
* op lines-by-linenumber-range (lines start end): returns lines with indices in one-based inclusive-inclusive range
* op list of writing systems
* op split-by-whitespace-nth: return the nth item of the list
* op take file content, apply delimiter e.g. "\n--\n" and return segments
* op take file content, fetch recursive structure e.g. "(" and ")" or "\begin{…}" and "\end{…}" and return segments
* op substring-byte-indices: return list of byte indices where a given substring occurs
* op substring-codepoint-indices: return list of codepoint indices where a given substring occurs
* op prefix-line-number (lines [opt. separator]): attach line number (or line number and separator) before each line
* op return lines N–M … so given line numbers, return the corresponding range of lines
* op binary-to-text encodings
* op ASCII art
* op random string (e.g. names) in writing system X
* op is-codepoint-boundary: is index #2 a boundary of a Unicode codepoint in string #1?
* op zip for line prefixes and line suffixes
* op apply multiple replacements simultaneously such that one replacement does not overlap with another
* op text wrapping to a certain width w.r.t https://www.unicode.org/reports/tr14/
* op implement some kind of human sorting? https://nedbatchelder.com/blog/200712/human_sorting.html
* op MIME type information?
* op file path operations? (remove file extension?)
* op guess writing system: (&str) -> String
* op writing-system-info: should return {writingsystem: [nations]} mapping
* op slice by byte indices
* op bool-answer-en: test for y|yes|t|true|1 or no|…
* op bool-answer-localized: test for y|yes|t|true|1 or no|… language-dependent
* op Unicode category information
* op for semver?

## op proposals with rust snippets

```rust
pub fn to_unicode_scalar_name(&self, _conf: &Configuration) -> String {
    // TODO: get Unicode code point name via ../data/UnicodeData.txt
    // retrieved via https://www.unicode.org/Public/UCD/latest/ucd/NamesList.txt on 2022-06-05
    "".to_string()
}

pub fn replace_limited_from_start(given: &str, substr: &str, replacement: &str, max_times: usize) -> String {
    given.replacen(substr, replacement, max_times)
}

pub fn replace_limited_from_end(given: &str, substr: &str, replacement: &str, max_times: usize) -> String {
    let mut positions = given.match_indices(substr).map(|tuple| { tuple.0 }).collect::<Vec<usize>>();
    let first_replacement_position = positions[positions.len().saturating_sub(max_times)];
    let mut result = String::from(&given[0..first_replacement_position]);
    result.push_str(&given[first_replacement_position..].replacen(substr, replacement, max_times));
    result
}

// TODO func: add limited_from_start and limited_from_end versions
// TODO add inclusive versions which keep the separator in the elements?
pub fn split_by_unicodenormalized_sep<'s>(s: &'s str, separator: &str, unicode_normalization: usize) -> Vec<&'s str> {
    s.split(separator).collect::<Vec<&str>>().to_vec() // TODO
}

// TODO: cmp-lexicographic: lexicographic comparison of two strings
// TODO: cmp of two strings (is cmp in rust lexicographic? if so, ignore this)
pub fn pairwise_cmp(s1: &str, s2: &str) -> i8 {
    use std::cmp;
    match s1.cmp(s2) {
        cmp::Ordering::Less => -1,
        cmp::Ordering::Equal => 0,
        cmp::Ordering::Greater => 1,
    }
}

pub fn find_byteindex_of_start_occurence(s: &str, substr: &str) -> Option<usize> {
    s.find(substr)
}

pub fn find_byteindex_of_last_occurence(s: &str, substr: &str) -> Option<usize> {
    s.rfind(substr)
}


pub fn byte_indexed_substr(s: &str, from: usize, to: usize) -> Option<String> {
    s.get(from..to).map(|slice| slice.to_owned())
}

pub fn to_unicode_escape_string(s: &str) -> String {
    s.escape_unicode().to_string()
}

pub fn xml_info(keyword: &str) -> String {
    let escaping = "characters {“<” U+003C LESS-THAN SIGN, “>” U+003E GREATER-THAN SIGN, “&” U+0026 AMPERSAND, “\"” U+0022 QUOTATION MARK, “'” U+0027 APOSTROPHE} must be escaped as {“&lt;”, “&gt;”, “&amp;”, “&quot;”, “&apos;”}".to_string();
    let declaration = r#"<?xml version="1.0" encoding="utf-8"?> or <?xml version="1.1" encoding="utf-8"?>"#.to_string();
    let spec = r#"XML 1.0 “W3C Recommendation 26 November 2008” (Fifth Edition) https://www.w3.org/TR/2008/REC-xml-20081126/ or XML 1.1 “W3C Recommendation 16 August 2006” (Second Edition) https://www.w3.org/TR/xml11/"#.to_string();

    match &keyword[0..4] {
        "esca" => return escaping, // escape, escaping, escaped, …
        "enco" => return escaping, // encoding, encoded, …
        "decl" => return declaration, // declaration
        "spec" => return spec, // specification
        _ => {},
    }

    match keyword {
        "marshal" => escaping,
        "unmarshal" => escaping,
        "version" => declaration,
        _ => String::new(),
    }
}
```
