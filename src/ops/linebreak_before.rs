use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

use std::iter;
use std::str;

/// LinebreakOpportunities iterates over line breaking opportunities
/// in a text. Thus, they emit a tuple (UTF-8 byte offset in the text,
/// count of Unicode scalars before this position) with each iteration.
/// The first (``(0, 0)``) and last position (``(text.len(), _)``) are
/// not emitted by implementors of this trait.
trait LinebreakOpportunities: Iterator<Item=(usize, usize)> {}

struct LinebreakOpportunitiesByChar<'l> {
    iter: iter::Enumerate<str::CharIndices<'l>>,
    separator: char,
}

impl<'s> LinebreakOpportunitiesByChar<'s> {
    fn new(text: &'s str, separator: char) -> LinebreakOpportunitiesByChar<'s> {
        Self { iter: text.char_indices().enumerate(), separator }
    }
}

impl<'l> Iterator for LinebreakOpportunitiesByChar<'l> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for (pos, (byte_offset, chr)) in &mut self.iter {
            if chr == self.separator {
                return Some((byte_offset, pos));
            }
        }

        None
    }
}

impl<'l> LinebreakOpportunities for LinebreakOpportunitiesByChar<'l> {}

struct LinebreakOpportunitiesAtN<'l> {
    iter: iter::Enumerate<str::CharIndices<'l>>,
    n: usize,
}

impl<'s> LinebreakOpportunitiesAtN<'s> {
    fn new(text: &'s str, n: usize) -> LinebreakOpportunitiesAtN<'s> {
        Self { iter: text.char_indices().enumerate(), n }
    }
}

impl<'l> Iterator for LinebreakOpportunitiesAtN<'l> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        for (pos, (byte_offset, _chr)) in &mut self.iter {
            if pos == self.n {
                return Some((byte_offset, pos));
            }
        }

        None
    }
}

impl<'l> LinebreakOpportunities for LinebreakOpportunitiesAtN<'l> {}

/// Break lines before reaching N characters per line
/// 
/// This implementation determines all legal line break opportunities,
/// given a string of text according to Unicode TR #14. It does not
/// support hyphenation. The result is left-align (= right-ragged) text.
pub struct LinebreakBefore {}

impl LinebreakBefore {
    // TODO do we need this?
    #[allow(dead_code)]
    fn find_linebreak_opportunities(line: &str) -> Vec<usize> {
        let mut lbo = vec![];

        for (byte_offset, _count) in LinebreakOpportunitiesByChar::new(line, ' ') {
            lbo.push(byte_offset);
        }

        lbo
    }

    /// Main routine of the line breaking algorithm.
    /// Splits `text` into lines of length smaller-equal to `len`
    /// and joins them back to text with `line_joiner`.
    /// 
    /// A recommended parameterization is `forced=true`
    /// and `line_joiner='\n'` (U+000A LINE FEED).
    fn linebreak(text: &str, len: usize, forced: bool, line_joiner: &str) -> String {
        assert!(len >= 2);

        let mut lines = vec![];

        for this_line in text.lines() {
            let this_len = this_line.chars().count();

            // trivial case
            if this_len <= len {
                lines.push(this_line);
                continue;
            }

            // find line break opportunities
            let mut linebreaks = vec![];

            let mut prev_byte_offset = 0;
            let mut prev_count = 0;
            let mut count_at_last_break = 0;
            for (byte_offset, count) in LinebreakOpportunitiesByChar::new(this_line, ' ') {
                // if it does not exceed yet, use as line break
                if (prev_count - count_at_last_break) > 0 && (prev_count - count_at_last_break) <= len && (count - count_at_last_break) > len {
                    linebreaks.push(prev_byte_offset);
                    count_at_last_break = prev_count;
                    dbg!(count_at_last_break);
                }

                prev_byte_offset = byte_offset;
                prev_count = count;
            }

            dbg!(&linebreaks);
            if prev_byte_offset == 0 {
                if forced {
                    // find line break opportunities
                    for (byte_offset, _count) in LinebreakOpportunitiesAtN::new(this_line, len) {
                        linebreaks.push(byte_offset);
                    }
                } else {
                    // keep line exceeding `len`
                    lines.push(this_line);
                    continue;
                }
            }

            // line breaking
            let mut prev_byte_offset = 0;
            for byte_offset in linebreaks.iter() {
                lines.push(&this_line[prev_byte_offset..*byte_offset]);
                prev_byte_offset = *byte_offset;
            }
            lines.push(&this_line[prev_byte_offset..]);
        }

        lines.join(line_joiner)
    } 
}

impl traits::Op for LinebreakBefore {
    fn name() -> &'static str { "linebreak-before" }
    fn usage() -> &'static str { "<#1 string text> <#2 int width>" }
    fn description() -> &'static str { "linebreak long lines in (text #1) before they reach (integer #2) codepoints" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let text: &str = args.get(0)?.try_into()?;
        let width: i64 = match args.get(1)?.try_into() {
            Ok(int) => int,
            Err(_) => return Ok(0.0),
        };

        let lp = text.lines().count().saturating_div(3);
        let lines_prio = lp as f32 / (lp as f32 + 1.);
        Ok(if 10 <= width && width < 200 {
            0.73
        } else {
            0.34
        } * lines_prio)
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        let text: &str = args.get(0)?.try_into()?;
        let width: i64 = args.get(1)?.try_into()?;

        let line_joiner = "\n";
        let force_if_no_opportunity = true;

        match width {
            0 => Ok(text.into()),
            1 => {
                let mut lines = String::with_capacity(text.len() * line_joiner.len());
                for chr in text.chars() {
                    lines.push(chr);
                    if chr != '\n' && chr != '\r' { // TODO there are more Unicode codepoints triggering mandatory line breaks
                        lines.push_str(line_joiner);
                    }
                }
                Ok(lines.into())
            },
            _ => {
                let s= Self::linebreak(text, width as usize, force_if_no_opportunity, line_joiner);
                Ok(s.into())
            }
        }
    }
}