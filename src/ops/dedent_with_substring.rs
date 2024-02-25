use crate::errors::Errors;
use crate::input::StrArg;
use crate::ops::traits;
use crate::output::Output;

pub struct DedentWithSubstring {}

impl traits::OpTwo for DedentWithSubstring {
    fn name() -> &'static str { "dedent-with-substring" }
    fn description() -> &'static str { "remove prefix string #2 at the beginning of every line of string #1" }

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
            if line.starts_with(indentation) {
                match line.get(indentation.len()..) {
                    Some(suffix) => out.push_str(suffix),
                    None => panic!("error in dedent-with-substring: removing a UTF-8 suffix from a UTF-8 string must always be possible"),
                };
            }
            if i != lines_count - 1 {
                out.push_str(terminator);
            }
        }

        Ok(out.into())
    }
}
