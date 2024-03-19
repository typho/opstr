use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct GuaranteeSuffix {}

impl traits::Op for GuaranteeSuffix {
    fn name() -> &'static str { "guarantee-suffix" }
    fn usage() -> &'static str { "<#1 string base> <#2 string suffix>" }
    fn description() -> &'static str { "if string #1 does not end with string #2, append it" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let base: &str = args.get(0)?.try_into()?;
        let suffix: &str = args.get(1)?.try_into()?;
        Ok(if base.len() > suffix.len() {
            0.53
        } else {
            0.32
        })
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let base: &str = args.get(0)?.try_into()?;
        let suffix: &str = args.get(1)?.try_into()?;

        let mut string: String = base.to_owned();
        if string.ends_with(suffix) {
            Ok(string.into())
        } else {
            string.push_str(suffix);
            Ok(string.into())
        }
    }
}
