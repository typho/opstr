use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct IndentWithSubstring {}

impl traits::OpTwo for IndentWithSubstring {
    fn name() -> &'static str { "indent-with-substring" }
    fn description() -> &'static str { "concatenate string #2 with every non-empty line in string #1, keep other lines" }

    fn priority(arg1: &StrArg, arg2: &StrArg) -> f32 {
        let string: &str = arg1.into();
        let indentation: &str = arg2.into();

        let prio1 = if string.lines().count() > 1 { 1.0 } else { 0.2 };
        let prio2 = if indentation.chars().all(char::is_whitespace) { 0.78 } else { 0.2 };
        prio1 * prio2
    }

    fn run(arg1: &StrArg, arg2: &StrArg) -> Result<Output, Errors> {
        let string: &str = arg1.into();
        let indentation: &str = arg2.into();

        let terminator = if string.contains("\r\n") { "\r\n" } else { "\n" };
        let lines_count = string.lines().count();

        let mut out = String::new();
        for (i, line) in string.lines().enumerate() {
            if !line.is_empty() {
                out.push_str(indentation);
                out.push_str(line);
            }
            if i != lines_count - 1 {
                out.push_str(terminator);
            }
        }

        Ok(out.into())
    }
}
