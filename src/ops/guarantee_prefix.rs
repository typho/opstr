use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct GuaranteePrefix {}

impl traits::Op for GuaranteePrefix {
    fn name() -> &'static str { "guarantee-prefix" }
    fn usage() -> &'static str { "<#1 string base> <#2 string prefix>" }
    fn description() -> &'static str { "if string #1 does not start with string #2, prepend it" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let base: &str = args.get(0)?.try_into()?;
        let prefix: &str = args.get(0)?.try_into()?;
        Ok(if base.len() > prefix.len() {
            0.52
        } else {
            0.31
        })
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let base: &str = args.get(0)?.try_into()?;
        let prefix: &str = args.get(0)?.try_into()?;
        if base.starts_with(prefix) {
            Ok(base.into())
        } else {
            let mut result: String = prefix.to_owned();
            result.push_str(base);
            Ok(result.into())
        }
    }
}
