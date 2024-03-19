use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct XmlEncode {}

impl traits::Op for XmlEncode {
    fn name() -> &'static str { "xml-encode" }
    fn usage() -> &'static str { "<#1 string to-encode>" }
    fn description() -> &'static str { "replace the 5 characters &<>\"' with their pre-defined XML entities in string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s.contains('&') || s.contains('<') || s.contains('>') || s.contains('"') || s.contains('\'') {
            0.43
        } else {
            0.13
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        let encoded = s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('\"', "&quot;").replace('\'', "&apos;");
        Ok(encoded.into())
    }
}
