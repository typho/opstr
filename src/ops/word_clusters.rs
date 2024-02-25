use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output, OutputValue};
use unicode_segmentation::UnicodeSegmentation;

pub struct WordClusters {}

impl traits::OpOne for WordClusters {
    fn name() -> &'static str { "word-clusters" }
    fn description() -> &'static str { "return “Word clusters” according to Unicode Standard Annex #29 “Unicode Text Segmentation”" }

    fn priority(arg: &StrArg) -> f32 {
        let string: &str = arg.into();
        if string.len() >= 20 { 0.598 } else { 0.22 }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg.into();
        let data = string.unicode_words().map(OutputValue::from_str).collect::<Vec<OutputValue>>();
        Ok(Output::HomogeneousList { data, notes: vec![] })
    }
}
