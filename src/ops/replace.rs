use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct Replace {}

impl Replace {
    fn function_for_str(arg1: &str, arg2: &str, arg3: &str) -> String {
        arg1.replace(arg2, arg3)
    }
}

impl traits::OpThree for Replace {
    fn name() -> &'static str { "replace" }
    fn description() -> &'static str { "replace string #2 with string #3 in string #1" }

    fn priority(base_argument: &StrArg, search_argument: &StrArg, _replace_argument: &StrArg) -> f32 {
        let base_arg: &str = base_argument.into();
        let search_arg: &str = search_argument.into();

        let factor = match base_arg.matches(search_arg).count() {
            0 => 0.03,
            1..=6 => 1.0,
            _ => 0.7,
        };

        if search_arg.len() > base_arg.len() {
            0.1 * factor
        } else if search_arg.len() * 2 > base_arg.len() {
            0.3 * factor
        } else {
            let diff = base_arg.len().abs_diff(search_arg.len()) as f32;
            0.8 * (diff / (diff + 1.0)) * factor
        }
    }

    fn run(arg1: &StrArg, arg2: &StrArg, arg3: &StrArg) -> Result<Output, Errors> {
        let s1 = arg1.into();
        let s2 = arg2.into();
        let s3 = arg3.into();

        Ok(Self::function_for_str(s1, s2, s3).into())
    }
}
