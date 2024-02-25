use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct GuaranteePrefix {}

impl traits::OpTwo for GuaranteePrefix {
    fn name() -> &'static str { "guarantee-prefix" }
    fn description() -> &'static str { "if string #1 does not start with string #2, prepend it" }

    fn priority(arg1: &StrArg, arg2: &StrArg) -> f32 {
        let string: &str = arg1.into();
        let prefix: &str = arg2.into();
        if string.len() > prefix.len() {
            0.52
        } else {
            0.31
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let base: &str = arg1.into();
        let prefix: &str = arg2.into();
        if base.starts_with(prefix) {
            Ok(base.into())
        } else {
            let mut result: String = prefix.to_owned();
            result.push_str(base);
            Ok(result.into())
        }
    }
}
