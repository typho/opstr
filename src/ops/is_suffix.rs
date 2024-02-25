use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct IsSuffix {}

impl IsSuffix {
    fn function_for_str(arg1: &str, arg2: &str) -> bool {
        arg1.ends_with(arg2)
    }

    fn function_for_byteslice(arg1: &[u8], arg2: &[u8]) -> bool {
        if arg2.len() > arg1.len() {
            false
        } else {
            &arg1[arg1.len() - arg2.len()..] == arg2
        }
    }
}

impl traits::OpTwo for IsSuffix {
    fn name() -> &'static str { "is-suffix" }
    fn description() -> &'static str { "does string #1 end with string #2?" }

    fn priority(arg1: &StrArg, arg2: &StrArg) -> f32 {
        let s1: &str = arg1.into();
        let s2: &str = arg2.into();
        if s1.len() < s2.len() {
            return 0.0;
        }

        let p = if s1.ends_with(s2) { 1. } else { 0.6 };
        (match usize::min(s1.len(), s2.len()) {
            0 => 0.0,
            1 => 0.1,
            2 => 0.3,
            _ => 0.4,
        }) * p
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        Ok(Self::function_for_str(arg1.into(), arg2.into()).into())
    }
}
