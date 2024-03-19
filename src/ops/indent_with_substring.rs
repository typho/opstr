use crate::errors::Errors;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct IndentWithSubstring {}

impl traits::Op for IndentWithSubstring {
    fn name() -> &'static str { "indent-with-substring" }
    fn usage() -> &'static str { "<#1 string lines> <#2 string prefix-to-attach>" }
    fn description() -> &'static str { "concatenate string #2 with every non-empty line in string #1, keep other lines" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        let indentation: &str = args.get(1)?.try_into()?;

        let prio1 = if string.lines().count() > 1 { 1.0 } else { 0.2 };
        let prio2 = if indentation.chars().all(char::is_whitespace) { 0.78 } else { 0.2 };
        Ok(prio1 * prio2)
    }

    fn run(args: &Args) -> Result<Output, Errors> {
        let string: &str = args.get(0)?.try_into()?;
        let indentation: &str = args.get(1)?.try_into()?;

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
