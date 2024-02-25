use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;
use unicode_segmentation::UnicodeSegmentation;

const COMBINING_LONG_STROKE_OVERLAY: char = '\u{0336}';

pub struct StrikeThrough {}

impl traits::OpOne for StrikeThrough {
    fn name() -> &'static str { "strike-through" }
    fn description() -> &'static str { "add U+0336 COMBINING LONG STROKE OVERLAY before each codepoint resulting in strike-through text" }
    fn priority(_arg: &StrArg) -> f32 { 0.382 }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let s: &str = arg.into();
        let mut result = String::new();
        for grapheme in UnicodeSegmentation::graphemes(s, true) {
            result.push_str(grapheme);
            result.push(COMBINING_LONG_STROKE_OVERLAY);
        }
        Ok(result.into())
    }
}
