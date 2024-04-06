//! The goal of this module is to take `Arguments` and match them against the
//! given structure specification. If met, the corresponding operation is called
//! to generate a result. All results are collected in `Results` and then represented
//! when printing them to the CLI.

use std::collections::HashMap;

use crate::auxiliary;
use crate::errors::LibError;
use crate::input;
use crate::ops;
use crate::config::Configuration;
use crate::output::{Output,OutputValue};
use crate::range;
use crate::ColorScheme;

/// Return the list of all operations as `Output::Association` of (name, description) entries.
pub fn list_all_ops(_conf: &Configuration) -> Output {
    let mut results = HashMap::new();

    for (fn_name, fn_desc, _, _, _, _) in ops::INDEX {
        results.insert(OutputValue::from_str(fn_name()), OutputValue::from_str(fn_desc()));
    }

    Output::Association { data: results, notes: vec![] }
}

/// Return the ordered list of appropriate operations as `Output::Association` of (name, description) entries.
pub fn list_matching_ops(conf: &Configuration, args: &input::Args) -> Vec<(&'static str, &'static str)> {
    let mut fns = vec![];

    for (fn_name, fn_desc, _fn_usage, fn_args, fn_priority, _fn_impl) in ops::INDEX {
        // Only consider functions where `acceptable_number_of_arguments` returns a range
        // where the given number of arguments is within
        if !fn_args().has(args.len()) {
            continue;
        }
        // CONSTRAINT: priority must be greater 0
        if let Ok(prio) = fn_priority(args, conf) {
            if prio > 0.0 && !prio.is_nan() {
                fns.push((fn_name(), fn_desc(), prio));
            }
        }
    }

    // sort by priority
    fns.sort_by_key(|e| (-1000.0 * e.2) as i32);

    // strip away third element
    fns.iter().map(|e| { (e.0, e.1) }).collect::<Vec<(&'static str, &'static str)>>()
}

/// Return `(op_name, Output)` as result of running the specified operation `op_name` with `Args`
pub fn run_op(conf: &Configuration, args: &input::Args, op_name: &str) -> Result<(&'static str, Output), LibError> {
    // (1) search for an args-independent exact match
    if let Some((name, usage, acceptable_range)) = find_op_by_exact_name(conf, args, op_name) {
        if !acceptable_range.has(args.len()) {
            return Err(LibError::ArgumentCountError(acceptable_range, args.len(), Some(usage.to_owned())));
        }
        return Ok((name, run_op_by_name(conf, args, op_name)?));
    }

    // (2) Collect operation names and string_similarity
    let mut names_and_similarity = vec![];
    for (fn_name, _, _, _, _, _) in ops::INDEX {
        names_and_similarity.push((fn_name(), auxiliary::string_similarity(fn_name(), op_name)));
    }

    // (3) Sort ops by similarity
    names_and_similarity.sort_by_key(|e| (1000.0 * e.1) as i32);

    // (4) Yield message "Did you mean …?"
    if let Some((last, _)) = names_and_similarity.last() {
        eprintln!("Did you mean ‘{}’?", last);
    }

    Err(LibError::UnknownOp(op_name.to_owned()))
}

fn find_op_by_exact_name(_conf: &Configuration, _args: &input::Args, op_name: &str) -> Option<(&'static str, &'static str, range::Range)> {
    for (fn_name, _fn_desc, fn_usage, fn_args, _fn_priority, _fn_impl) in ops::INDEX {
        // CONSTRAINT: name must match user-provided name
        if fn_name() != op_name {
            continue;
        }
        // run the function
        return Some((fn_name(), fn_usage(), fn_args()));
    }

    None
}

fn run_op_by_name(conf: &Configuration, args: &input::Args, op_name: &str) -> Result<Output, LibError> {
    for (fn_name, _fn_desc, _fn_usage, _fn_args, _fn_priority, fn_impl) in ops::INDEX {
        if fn_name() != op_name {
            continue;
        }
        return fn_impl(args, conf);
    }

    Err(LibError::UnknownOp(op_name.to_owned()))
}

/// Run all operations appropriate for the provided `Args`. Write the result to stdout & stderr, so we return `()`
pub fn run_matching_ops(conf: &Configuration, args: &input::Args) -> Result<(), LibError> {
    // (1) determine the set of functions accepting this number of arguments and returning priority > 0.0
    let mut priority_per_function: Vec<(&'static str, f32)> = vec![];

    // NOTE: INDEX_MULTI must always be run, independent of the number of arguments
    for (fn_name, _fn_desc, _fn_usage, fn_args, fn_priority, _fn_impl) in ops::INDEX {
        let name: &'static str = fn_name();
        if !fn_args().has(args.len()) {
            continue;
        }
        if let Ok(prio) = fn_priority(args, conf) {
            if prio > 0.0 && !prio.is_nan() {
                priority_per_function.push((name, prio));
            }
        }
    }

    // (2) sort set by priorities
    priority_per_function.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap().reverse());

    // (3) invoke functions in correct order
    for (op_name, _) in priority_per_function {
        conf.color_scheme.op_section(op_name)?;

        for (fn_name, _fn_desc, _fn_usage, _fn_args, _fn_priority, fn_impl) in ops::INDEX {
            let name: &'static str = fn_name();
            if name != op_name {
                continue;
            }

            match fn_impl(args, conf) {
                Ok(output) => { output.print(conf)?; },
                Err(e) => {
                    conf.color_scheme.error_label("ERROR")?;
                    eprintln!(": {}", e);
                },
            }
        }
    }

    Ok(())
}

/// Run all operations appropriate for the provided `Args`. Write the result to stdout & stderr, so we return `()`
pub fn list_color_schemes(_conf: &Configuration) -> Result<Output, LibError> {
    let mut cs_list = vec![];
    for cs in ColorScheme::all_as_slice() {
        cs_list.push(OutputValue::from_str(&cs.to_string()));
    }
    Ok(Output::HomogeneousList { data: cs_list, notes: vec![] })
}
