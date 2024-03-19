use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

use unicode_segmentation::UnicodeSegmentation;

const COMBINING_LONG_STROKE_OVERLAY: char = '\u{0336}';

pub struct StrikeThrough {}

impl traits::Op for StrikeThrough {
    fn name() -> &'static str { "strike-through" }
    fn usage() -> &'static str { "<#1 string text>" }
    fn description() -> &'static str { "add U+0336 COMBINING LONG STROKE OVERLAY before each codepoint resulting in strike-through text" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }
    fn priority(_args: &Args) -> Result<f32, Errors> { Ok(0.382) }

    fn run(args: &Args) -> Result<Output, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        let mut result = String::new();
        for grapheme in UnicodeSegmentation::graphemes(s, true) {
            result.push_str(grapheme);
            result.push(COMBINING_LONG_STROKE_OVERLAY);
        }
        Ok(result.into())
    }
}
