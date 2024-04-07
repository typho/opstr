//! The executable providing opstr on a command-line interface.
//!
//! Assume the strings are provided as CLI arguments. We can specify the operation
//! and the operation implementation will be applied to it.

use std::ffi::OsString;
use std::path;
use std::process;

use clap::Parser;
use opstr::Args;
use opstr::Configuration;
use opstr::LibError;

use std::io::Read;
use std::fs;

/// Analyze strings and apply string operations
#[derive(Debug, Parser)]
#[command(name = "opstr")]
#[command(author = "tajpulo <tajpulo@typho.org>")]
#[command(version = "1.1.0")]
#[command(about = "Operate on strings")]
#[command(author, version, about, long_about = None, trailing_var_arg = true)]
struct Opts {
    #[clap(short = 'p', long, help = "name of the operation to apply")]
    op: Option<String>,
    #[clap(short, long, help = "provide the list of operations matching the provided keywords")]
    list_ops: bool,
    #[clap(long, help = "provide the list of selectable syntaxes")]
    list_syntax: bool,
    #[clap(long, help = "dump which arguments & configuration you consider and terminate (helpful for debugging)")]
    dump: bool,
    #[clap(long, help = "representation output syntax")]
    syntax: Option<String>,
    #[clap(long, help = "use uppercase letters for representation of hexadecimal values")]
    hex_upper: Option<bool>,
    #[clap(long, help = "radix {2, 10, or 16} to use for integers")]
    radix: Option<u8>,
    #[clap(long, help = "if a list of elements/table is returned, return the zero-based {item}-th element/row, else ignore this option")]
    item: Option<isize>,
    #[clap(long, help = "if an association/table is returned, return key `column` or only column name `column`, else ignore this option")]
    column: Option<String>,
    #[clap(long, help = "provide the list of selectable color schemes")]
    list_color_scheme: bool,
    #[clap(long, help = "color scheme like none, default, or regularandbold")]
    color_scheme: Option<String>,
    #[clap(long, help = "locale to be used for locale-dependent operations")]
    locale: Option<String>,
    #[clap(long, help = "replace the argument with this one-based ID with stdin content")]
    stdin_as_arg: Vec<usize>,
    #[clap(long, help = "interpret the argument with this one-based ID as filepath and insert its file content there")]
    file_as_arg: Vec<usize>,
    #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<OsString>,
}

fn main() -> Result<(), LibError> {
    // (1) Prepare configuration
    let opts = Opts::parse();
    let mut arguments = vec![];

    'outer: for (i, arg) in opts.args.iter().enumerate() {
        // handle stdin arguments
        for stdin_i in opts.stdin_as_arg.iter() {
            if *stdin_i == i + 1 {
                // read stdin
                let mut buffer = Vec::new();
                let stdin = std::io::stdin();
                let mut handle = stdin.lock();
                handle.read_to_end(&mut buffer)?;

                // decode stdin content
                let stdin_string = match String::from_utf8(buffer) {
                    Ok(s) => s,
                    Err(_) => return Err(LibError::CLIValueError("stdin-as-arg", "expected UTF-8 content at stdin, got non-UTF-8 bytes".to_owned())),
                };

                // add as argument
                arguments.push(opstr::Arg::from_str(&stdin_string, i));
                continue 'outer;
            }
        }

        // handle file arguments
        for stdin_i in opts.file_as_arg.iter() {
            if *stdin_i == i + 1 {
                // read file content
                let filepath = match arg.to_str() {
                    Some(s) => s,
                    None => return Err(LibError::CLIValueError("file-as-arg", "expected UTF-8 filepath, got non-UTF-8 bytes".to_owned())),
                };

                if !path::Path::new(filepath).exists() {
                    return Err(LibError::CLIValueError("file-as-arg", format!("file '{}' not found", filepath)));
                }

                // decode file content
                let file_content = match fs::read_to_string(filepath) {
                    Ok(s) => s,
                    Err(_) => return Err(LibError::CLIValueError("file-as-arg", format!("expected UTF-8 content in file '{}', got non-UTF-8 bytes", filepath))),
                };

                // add as argument
                arguments.push(opstr::Arg::from_str(&file_content, i));
                continue 'outer;
            }
        }
        
        if let Some(utf8_string) = arg.to_str() {
            arguments.push(opstr::Arg::from_str(utf8_string, i));
        } else {
            return Err(LibError::ArgValueError(i, "expected a valid UTF-8 string, got non-UTF-8 bytes".to_owned()))
        }
    }

    let args = Args::from(&arguments);

    let mut conf = Configuration::default();
    conf.overwrite_with_env()?;
    conf.overwrite_with_clap(opts.radix, opts.item, opts.column, opts.hex_upper, opts.color_scheme, opts.locale, opts.syntax)?;

    if opts.dump {
        println!("{:?}", args);
        println!("{:?}", conf);
        return Ok(());

    } else if opts.list_color_scheme {
        return Ok(opstr::list_color_schemes(&conf)?.print(&conf)?);
    }

    if opts.list_ops {
        // list all operations
        opstr::list_all_ops(&conf).print(&conf)?
        
    } else if let Some(op_name) = opts.op {
        // apply the mentioned operation
        match opstr::run_op(&conf, &args, &op_name) {
            Ok((_fn_name, fn_output)) => fn_output.print(&conf)?,
            Err(err) => {
                conf.color_scheme.error_label("ERROR")?;
                eprintln!(": {}", err);
                process::exit(2);
            },
        };

    } else {
        opstr::run_matching_ops(&conf, &args)?;
    }

    Ok(())
}
