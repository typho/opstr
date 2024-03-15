use clap::Parser;
use std::ffi::OsString;
use std::process;
use opstr::Configuration;
use opstr::Errors;

/// Analyze strings and apply string operations
#[derive(Debug, Parser)]
/*#[command(name = "opstr")]
#[command(author = "tajpulo <tajpulo@typho.org>")]
#[command(version = "0.5")]
#[command(about = "Read document as tree and apply Lua functions to nodes")]*/
#[command(author, version, about, long_about = None, trailing_var_arg = true)]
struct Opts {
    #[clap(short, long, help = "name of the operation to apply")]
    op: Option<String>,
    #[clap(short, long, help = "provide the list of operations matching the provided keywords")]
    list_ops: bool,
    #[clap(long, help = "representation output syntax")]
    syntax: Option<String>,
    #[clap(long, help = "use uppercase letters for representation of hexadecimal values")]
    hex_upper: Option<bool>,
    #[clap(long, help = "radix {2, 10, or 16} to use for integers")]
    radix: Option<u8>,
    #[clap(long, help = "if a list of elements/table is returned, return the {item}-th element/row, else ignore this option")]
    item: Option<isize>,
    #[clap(long, help = "if an association/table is returned, return key `column` or only column name `column`, else ignore this option")]
    column: Option<String>,
    #[clap(long, help = "color scheme like none, default, or regularandbold")]
    color_scheme: Option<String>,
    #[clap(long, help = "locale to be used for locale-dependent operations")]
    locale: Option<String>,
    #[clap(trailing_var_arg = true, allow_hyphen_values = true)]
    args: Vec<OsString>,
}

fn main() -> Result<(), Errors> {
    // (1) Prepare configuration
    let opts = Opts::parse();
    let mut args = vec![];

    // TODO accept "-" as stdin

    for (i, arg) in opts.args.iter().enumerate() {
        if let Some(utf8_string) = arg.to_str() {
            args.push(opstr::StrArg::new( utf8_string, i ));
        }
    }

    let mut conf = Configuration::default();
    conf.overwrite_with_env()?;
    conf.overwrite_with_clap(opts.radix, opts.item, opts.column, opts.hex_upper, opts.color_scheme, opts.locale, opts.syntax)?;

    if opts.list_ops {
        // list all operations
        opstr::list_ops(&conf, &args).print(&conf)?;

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
        opstr::run_unspecified_op(&conf, &args)?;
    }

    Ok(())
}