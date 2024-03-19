use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct XmlDecode {}

impl traits::Op for XmlDecode {
    fn name() -> &'static str { "xml-decode" }
    fn usage() -> &'static str { "<#1 string to-decode>" }
    fn description() -> &'static str { "replace the 5 pre-defined XML entities with their unescaped characters &<>\"' in string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(1, 1) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        Ok(if s.contains('&') && s.contains(';') {
            0.41
        } else {
            0.0
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let s: &str = args.get(0)?.try_into()?;
        let encoded = s.replace("&lt;", "<").replace("&gt;", ">").replace("&quot;", "\"").replace("&apos;", "'").replace("&amp;", "&");
        Ok(encoded.into())
    }
}
