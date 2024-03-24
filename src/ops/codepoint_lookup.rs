use crate::auxiliary;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::{Output,OutputValue};
use crate::range;

pub struct CodepointLookup {}

impl traits::Op for CodepointLookup {
    fn name() -> &'static str { "codepoint-lookup" }
    fn usage() -> &'static str { "<#1 string unicode-codepoint-name>" }
    fn description() -> &'static str { "given the Unicode name as string #1 (e.g. “LATIN SMALL LETTER A”), return its UTF-8 representation (or an empty string, if unknown)" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }
    fn priority(_args: &Args) -> Result<f32, LibError> { Ok(0.26) }

    fn run(args: &Args) -> Result<Output, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let name = string.trim();
        let character = auxiliary::unicode_name_to_codepoint(name);

        match character {
            Some(chr) => Ok(Output::Scalar {
                data: OutputValue::SingleLineText(chr.to_string()), notes: vec![]
            }),
            None => Ok(Output::Scalar {
                data: OutputValue::SingleLineText("".to_owned()),
                notes: vec!["Unicode name not found in UnicodeData.txt of Unicode 15.0".to_owned()],
            })
        }
        
    }
}