use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct XmlEncode {}

impl traits::OpOne for XmlEncode {
    fn name() -> &'static str { "xml-encode" }
    fn description() -> &'static str { "replace the 5 characters &<>\"' with their pre-defined XML entities in string #1" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s.contains('&') || s.contains('<') || s.contains('>') || s.contains('"') || s.contains('\'') {
            0.43
        } else {
            0.13
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let s: &str = arg.into();
        let encoded = s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('\"', "&quot;").replace('\'', "&apos;");
        Ok(encoded.into())
    }
}
