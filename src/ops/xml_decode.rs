use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct XmlDecode {}

impl traits::OpOne for XmlDecode {
    fn name() -> &'static str { "xml-decode" }
    fn description() -> &'static str { "replace the 5 pre-defined XML entities with theire unescaped characters &<>\"' in string #1" }

    fn priority(arg: &StrArg) -> f32 {
        let s: &str = arg.into();
        if s.contains('&') {
            0.41
        } else {
            0.0
        }
    }

    fn run(arg: &StrArg) -> Result<Output, Errors> {
        let s: &str = arg.into();
        let encoded = s.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'").replace("&amp;", "&");
        Ok(encoded.into())
    }
}
