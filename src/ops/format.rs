use crate::config::Configuration;
use crate::errors::LibError;
use crate::input::Args;
use crate::ops::traits;
use crate::ops::traits::Op;
use crate::output::Output;
use crate::range;

use std::fmt;

use rt_format::ParsedFormat;
use rt_format::Specifier;
use rt_format::argument::FormatArgument;

pub struct Format {}

impl Format {
    fn function_for_chars(args: &[&str]) -> Result<String, LibError> {
        match args.len() {
            0 => Err(LibError::ArgumentCountError(Self::acceptable_number_of_arguments(), 0, None)),
            1 => Ok(args[0].into()),
            _ => {
                let template: &str = args[0];
                if !template.contains("{") {
                    return Ok(template.to_owned());
                }

                // prepare arguments
                let mut fmt_args = vec![];
                for arg in args.iter().skip(1) {
                    fmt_args.push(FmtArg(arg.to_string()));
                }

                let repr = match ParsedFormat::parse(template, &fmt_args, &rt_format::NoNamedArguments) {
                    Ok(val) => val.to_string(),
                    Err(failing_pos) => return Err(LibError::ArgValueError(0, format!("format string is invalid at zero-based position {}", failing_pos))),
                };

                Ok(repr)
            }
        }
    }
}

impl traits::Op for Format {
    fn name() -> &'static str { "format" }
    fn usage() -> &'static str { "<#1 string format-with-placeholders> [<#2 string arg> 0 or more times]" }
    fn description() -> &'static str { "replace {placeholders} in string #1 with consecutive arguments #2, #3, …" }
    fn acceptable_number_of_arguments() -> range::Range { range::Range::IndexOpen(1) }

    fn priority(args: &Args, _conf: &Configuration) -> Result<f32, LibError> {
        let template: &str = args.get(0)?.try_into()?;
        let occurences_start = template.matches('{').count().max(5);
        let occurences_end = template.matches('}').count().max(5);

        if occurences_start < (args.len() - 1) {
            // NOTE: there are not sufficient placeholders for the arguments
            return Ok(0.0);
        }

        let mut score = 0.75 + (0.05 * occurences_start as f32);
        if occurences_start != occurences_end {
            score *= 0.5;
        }
        Ok(score)
    }

    fn run(args: &Args, _conf: &Configuration) -> Result<Output, LibError> {
        match args.len() {
            0 => Ok("".into()),
            1 => {
                let arg: &str = args.get(0)?.try_into()?;
                Ok(arg.into())
            },
            _ => {
                let mut arguments = vec![];
                for arg in args.iter() {
                    let s: &str = arg.try_into()?;
                    arguments.push(s);
                }
        
                Ok(Self::function_for_chars(&arguments)?.into())
            }
        }
    }
}

#[derive(Debug)]
pub struct FmtArg(String);

impl<'s> FormatArgument for FmtArg {
    fn supports_format(&self, _specifier: &Specifier) -> bool { true }

    // NOTE do not use write! instead of fmt(…) here,
    //      as this would ignore the specifier

    fn fmt_display(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }

    fn fmt_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }

    fn fmt_octal(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<i64>() {
            Ok(int) => fmt::Display::fmt(&int, f),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_lower_hex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<i64>() {
            Ok(int) => fmt::Display::fmt(&int, f),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_upper_hex(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<i64>() {
            Ok(int) => fmt::Display::fmt(&int, f),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_binary(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<i64>() {
            Ok(int) => fmt::Display::fmt(&int, f),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_lower_exp(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<i64>() {
            Ok(int) => fmt::Display::fmt(&int, f),
            Err(_) => Err(std::fmt::Error),
        }
    }

    fn fmt_upper_exp(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.parse::<i64>() {
            Ok(int) => fmt::Display::fmt(&int, f),
            Err(_) => Err(std::fmt::Error),
        }
    }
}