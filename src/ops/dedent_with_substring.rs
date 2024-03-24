use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::output::Output;
use crate::range;

pub struct DedentWithSubstring {}

impl traits::Op for DedentWithSubstring {
    fn name() -> &'static str { "dedent-with-substring" }
    fn usage() -> &'static str { "<#1 string lines> <#2 string prefix-to-remove>" }
    fn description() -> &'static str { "remove prefix string #2 at the beginning of every line of string #1" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexIndex(2, 2) }

    fn priority(args: &Args) -> Result<f32, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let indentation: &str = args.get(1)?.try_into()?;

        let prio1 = if string.lines().count() > 1 { 1.0 } else { 0.2 };
        let prio2 = if indentation.chars().all(char::is_whitespace) { 0.78 } else { 0.2 };
        Ok(prio1 * prio2)
    }

    fn run(args: &Args) -> Result<Output, LibError> {
        let string: &str = args.get(0)?.try_into()?;
        let indentation: &str = args.get(1)?.try_into()?;

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
