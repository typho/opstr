use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use unicode_segmentation::UnicodeSegmentation;

pub struct GraphemeClusters {}

impl traits::OpOne for GraphemeClusters {
    fn name() -> &'static str { "grapheme-clusters" }
    fn description() -> &'static str { "return “Grapheme clusters” according to Unicode Standard Annex #29 “Unicode Text Segmentation”" }

    fn priority(arg: &StrArg) -> f32 {
        let string: &str = arg.into();
        if string.len() >= 20 { 0.6 } else { 0.29 }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg.into();
        let data = UnicodeSegmentation::graphemes(string, true).map(OutputValue::from_str).collect::<Vec<OutputValue>>();
        Ok(Output::HomogeneousList { data, notes: vec![] })
    }
}
