use crate::auxiliary;
use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::{Output,OutputValue};

pub struct CodepointLookup {}

impl traits::OpOne for CodepointLookup {
    fn name() -> &'static str { "codepoint-lookup" }
    fn description() -> &'static str { "given the Unicode name as string #1 (e.g. “LATIN SMALL LETTER A”), return its UTF-8 representation (or an empty string, if unknown)" }
    fn priority(_arg: &StrArg) -> f32 { 0.26 }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg.into();
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