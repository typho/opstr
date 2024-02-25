//! The goal of this module is to take `Arguments` and match them against the
//! given structure specification. If met, the corresponding operation is called
//! to generate a result. All results are collected in `Results` and then represented
//! when printing them to the CLI.

use std::collections::HashMap;

use crate::auxiliary;
use crate::errors::Errors;
use crate::input;
use crate::ops;
use crate::config::Configuration;
use crate::output;
use crate::output::OutputValue;

/// Return the list of operations as association of (name, description) entries
/// appropriate for the arguments `args` provided.
pub fn list_ops(conf: &Configuration, args: &input::StrArgs) -> output::Output {
    let ops: HashMap<&'static str, &'static str> = if args.len() == 0 {
        list_all_ops(conf)
    } else {
        list_matching_ops(conf, args)
    };
    let mut association_entries = HashMap::new();
    for (op_name, op_description) in ops {
        association_entries.insert(OutputValue::from_str(op_name), OutputValue::from_str(op_description));
    }
    output::Output::Association {
        data: association_entries,
        notes: vec![],
    }
}

/// Return the list of all operations as association of (name, description) entries.
fn list_all_ops(_conf: &Configuration) -> HashMap<&'static str, &'static str> {
    let mut results = HashMap::new();

    for (fn_name, fn_desc, _, _) in ops::INDEX_ZERO {
        results.insert(fn_name(), fn_desc());
    }
    for (fn_name, fn_desc, _, _) in ops::INDEX_ONE {
        results.insert(fn_name(), fn_desc());
    }
    for (fn_name, fn_desc, _, _) in ops::INDEX_TWO {
        results.insert(fn_name(), fn_desc());
    }
    for (fn_name, fn_desc, _, _) in ops::INDEX_THREE {
        results.insert(fn_name(), fn_desc());
    }
    for (fn_name, fn_desc, _, _) in ops::INDEX_MULTI {
        results.insert(fn_name(), fn_desc());
    }

    results
}

/// Return the list of appropriate operations as association of (name, description) entries.
fn list_matching_ops(_conf: &Configuration, args: &input::StrArgs) -> HashMap<&'static str, &'static str> {
    let mut results = HashMap::new();
    let args_count = args.len();

    match args_count {
        0 => {
            for (fn_name, fn_desc, fn_priority, _) in ops::INDEX_ZERO {
                // CONSTRAINT: priority must be greater 0
                if fn_priority() > 0.0 {
                    results.insert(fn_name(), fn_desc());
                }
            }
        },

        1 => {
            for (fn_name, fn_desc, fn_priority, _) in ops::INDEX_ONE {
                // CONSTRAINT: priority must be greater 0
                if fn_priority(args.first().unwrap()) > 0.0 {
                    results.insert(fn_name(), fn_desc());
                }
            }
        },

        2 => {
            for (fn_name, fn_desc, fn_priority, _) in ops::INDEX_TWO {
                // CONSTRAINT: priority must be greater 0
                if fn_priority(args.first().unwrap(), args.get(1).unwrap()) > 0.0 {
                    results.insert(fn_name(), fn_desc());
                }
            }
        },

        3 => {
            for (fn_name, fn_desc, fn_priority, _) in ops::INDEX_THREE {
                // CONSTRAINT: priority must be greater 0
                if fn_priority(args.first().unwrap(), args.get(1).unwrap(), args.get(2).unwrap()) > 0.0 {
                    results.insert(fn_name(), fn_desc());
                }
            }
        },

        _ => {},
    }


    // nothing matched? then let us try the multi-argument functions!
    for (fn_name, fn_desc, fn_priority, _) in ops::INDEX_MULTI {
        // CONSTRAINT: priority must be greater 0
        if fn_priority(args) > 0.0 {
            results.insert(fn_name(), fn_desc());
        }
    }

    results
}

pub fn run_op(conf: &Configuration, args: &input::StrArgs, op_name: &str) -> Result<(&'static str, output::Output), Errors> {
    // (1) search for args-independent exact match
    if let Some((name, _)) = find_op_by_exact_name(conf, args, op_name) {
        return Ok((name, run_op_by_name(conf, args, op_name)?));
    }

    // (2) Collect operation names and string_similarity
    let mut names_and_similarity = vec![];
    for (fn_name, _, _, _) in ops::INDEX_ZERO {
        names_and_similarity.push((fn_name(), auxiliary::string_similarity(fn_name(), op_name)));
    }
    for (fn_name, _, _, _) in ops::INDEX_ONE {
        names_and_similarity.push((fn_name(), auxiliary::string_similarity(fn_name(), op_name)));
    }
    for (fn_name, _, _, _) in ops::INDEX_TWO {
        names_and_similarity.push((fn_name(), auxiliary::string_similarity(fn_name(), op_name)));
    }
    for (fn_name, _, _, _) in ops::INDEX_THREE {
        names_and_similarity.push((fn_name(), auxiliary::string_similarity(fn_name(), op_name)));
    }
    for (fn_name, _, _, _) in ops::INDEX_MULTI {
        names_and_similarity.push((fn_name(), auxiliary::string_similarity(fn_name(), op_name)));
    }

    // (3) Sort ops by similarity
    names_and_similarity.sort_by_key(|e| (1000.0 * e.1) as i32);

    // (4) Yield message "Did you mean …?"
    if let Some((last, _)) = names_and_similarity.last() {
        eprintln!("Did you mean ‘{}’?", last);
    }

    Err(Errors::UnknownOp(op_name.to_owned()))
}

fn find_op_by_exact_name(_conf: &Configuration, args: &input::StrArgs, op_name: &str) -> Option<(&'static str, f32)> {
    let default_strarg0 = input::StrArg::new("", 0);
    let default_strarg1 = input::StrArg::new("", 1);
    let default_strarg2 = input::StrArg::new("", 2);
    let ds = |i: usize| { match i {
        0 => &default_strarg0,
        1 => &default_strarg1,
        _ => &default_strarg2,
    }};

    for (fn_name, _, fn_priority, _) in ops::INDEX_ZERO {
        // CONSTRAINT: name must match user-provided name
        if fn_name() != op_name {
            continue;
        }
        // run the function
        return Some((fn_name(), fn_priority()));
    }
    for (fn_name, _, fn_priority, _) in ops::INDEX_ONE {
        // CONSTRAINT: name must match user-provided name
        if fn_name() != op_name {
            continue;
        }
        // run the function
        let mut arguments = vec![];
        for i in 0..1 {
            arguments.push(match args.get(i) {
                Some(s) => s,
                None => ds(i),
            });
        }
        return Some((fn_name(), fn_priority(arguments[0])));
    }
    for (fn_name, _, fn_priority, _) in ops::INDEX_TWO {
        // CONSTRAINT: name must match user-provided name
        if fn_name() != op_name {
            continue;
        }
        // run the function
        let mut arguments = vec![];
        for i in 0..2 {
            arguments.push(match args.get(i) {
                Some(s) => s,
                None => ds(i),
            });
        }
        return Some((fn_name(), fn_priority(arguments[0], arguments[1])));
    }
    for (fn_name, _, fn_priority, _) in ops::INDEX_THREE {
        // CONSTRAINT: name must match user-provided name
        if fn_name() != op_name {
            continue;
        }
        // run the function
        let mut arguments = vec![];
        for i in 0..3 {
            arguments.push(match args.get(i) {
                Some(s) => s,
                None => ds(i),
            });
        }
        return Some((fn_name(), fn_priority(arguments[0], arguments[1], arguments[2])));
    }
    for (fn_name, _, fn_priority, _) in ops::INDEX_MULTI {
        // CONSTRAINT: name must match user-provided name
        if fn_name() != op_name {
            continue;
        }
        // run the function
        return Some((fn_name(), fn_priority(args)));
    }

    None
}

fn run_op_by_name(_conf: &Configuration, args: &input::StrArgs, op_name: &str) -> Result<output::Output, Errors> {
    for (fn_name, _, _, fn_impl) in ops::INDEX_ZERO {
        if fn_name() != op_name {
            continue;
        }
        if !args.is_empty() {
            return Err(Errors::ArgumentCountError(0.into(), args.len()));
        }
        return fn_impl();
    }

    for (fn_name, _, _, fn_impl) in ops::INDEX_ONE {
        if fn_name() != op_name {
            continue;
        }
        if args.len() != 1 {
            return Err(Errors::ArgumentCountError(1.into(), args.len()));
        }
        let arg0: &input::StrArg = args.first().unwrap();
        return fn_impl(arg0);
    }

    for (fn_name, _, _, fn_impl) in ops::INDEX_TWO {
        if fn_name() != op_name {
            continue;
        }
        if args.len() != 2 {
            return Err(Errors::ArgumentCountError(2.into(), args.len()));
        }
        let arg0: &input::StrArg = args.first().unwrap();
        let arg1: &input::StrArg = args.get(1).unwrap();
        return fn_impl(arg0, arg1);
    }

    for (fn_name, _, _, fn_impl) in ops::INDEX_THREE {
        if fn_name() != op_name {
            continue;
        }
        if args.len() != 3 {
            return Err(Errors::ArgumentCountError(3.into(), args.len()));
        }
        let arg0: &input::StrArg = args.first().unwrap();
        let arg1: &input::StrArg = args.get(1).unwrap();
        let arg2: &input::StrArg = args.get(2).unwrap();
        return fn_impl(arg0, arg1, arg2);
    }

    for (fn_name, _, _, fn_impl) in ops::INDEX_MULTI {
        if fn_name() != op_name {
            continue;
        }
        // TODO multi fn should provide a function which allows to check the number of arguments
        return fn_impl(args);
    }

    Err(Errors::UnknownOp(op_name.to_owned()))
}

pub fn run_unspecified_op(conf: &Configuration, args: &input::StrArgs) {
    // (1) determine the set of functions returning priority > 0.0
    let mut priority_per_function: Vec<(&'static str, f32)> = vec![];

    match args.len() {
        0 => {
            for (fn_name, _, fn_priority, _) in ops::INDEX_ZERO {
                let name: &'static str = fn_name();
                let priority = fn_priority();
                if priority > 0.0 && !priority.is_nan() {
                    priority_per_function.push((name, priority));
                }
            }
        },
        1 => {
            for (fn_name, _, fn_priority, _) in ops::INDEX_ONE {
                let name: &'static str = fn_name();
                let arg0: &input::StrArg = args.first().unwrap();
                let priority = fn_priority(arg0);
                if priority > 0.0 && !priority.is_nan() {
                    priority_per_function.push((name, priority));
                }
            }
        },
        2 => {
            for (fn_name, _, fn_priority, _) in ops::INDEX_TWO {
                let name: &'static str = fn_name();
                let arg0: &input::StrArg = args.first().unwrap();
                let arg1: &input::StrArg = args.get(1).unwrap();
                let priority = fn_priority(arg0, arg1);
                if priority > 0.0 && !priority.is_nan() {
                    priority_per_function.push((name, priority));
                }
            }
        },
        3 => {
            for (fn_name, _, fn_priority, _) in ops::INDEX_THREE {
                let name: &'static str = fn_name();
                let arg0: &input::StrArg = args.first().unwrap();
                let arg1: &input::StrArg = args.get(1).unwrap();
                let arg2: &input::StrArg = args.get(2).unwrap();
                let priority = fn_priority(arg0, arg1, arg2);
                if priority > 0.0 && !priority.is_nan() {
                    priority_per_function.push((name, priority));
                }
            }
        },
        _ => {},
    };

    // NOTE: INDEX_MULTI must always be run, independent of the number of arguments
    for (fn_name, _, fn_priority, _) in ops::INDEX_MULTI {
        let name: &'static str = fn_name();
        let priority = fn_priority(args);
        if priority > 0.0 && !priority.is_nan() {
            priority_per_function.push((name, priority));
        }
    }

    // (2) sort set by priorities
    priority_per_function.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    // (3) invoke functions in correct order
    for (op_name, _) in priority_per_function {
        println!("----- {op} {rep}", op=op_name, rep="-".repeat((65 - op_name.len()).max(0)));
        match args.len() {
            0 => {
                for (fn_name, _, _, fn_impl) in ops::INDEX_ZERO {
                    let name: &'static str = fn_name();
                    if name != op_name {
                        continue;
                    }

                    match fn_impl() {
                        Ok(output) => output.print(conf),
                        Err(e) => eprintln!("ERROR({}): {}", name, e),
                    }
                }
            },
            1 => {
                for (fn_name, _, _, fn_impl) in ops::INDEX_ONE {
                    let name: &'static str = fn_name();
                    if name != op_name {
                        continue;
                    }

                    let arg0: &input::StrArg = args.first().unwrap();
                    match fn_impl(arg0) {
                        Ok(output) => output.print(conf),
                        Err(e) => eprintln!("ERROR({}): {}", name, e),
                    }
                }
            },
            2 => {
                for (fn_name, _, _, fn_impl) in ops::INDEX_TWO {
                    let name: &'static str = fn_name();
                    if name != op_name {
                        continue;
                    }

                    let arg0: &input::StrArg = args.first().unwrap();
                    let arg1: &input::StrArg = args.get(1).unwrap();
                    match fn_impl(arg0, arg1) {
                        Ok(output) => output.print(conf),
                        Err(e) => eprintln!("ERROR({}): {}", name, e),
                    }
                }
            },
            3 => {
                for (fn_name, _, _, fn_impl) in ops::INDEX_THREE {
                    let name: &'static str = fn_name();
                    if name != op_name {
                        continue;
                    }

                    let arg0: &input::StrArg = args.first().unwrap();
                    let arg1: &input::StrArg = args.get(1).unwrap();
                    let arg2: &input::StrArg = args.get(2).unwrap();
                    match fn_impl(arg0, arg1, arg2) {
                        Ok(output) => output.print(conf),
                        Err(e) => eprintln!("ERROR({}): {}", name, e),
                    }
                }
            },
            _ => {},
        };

        for (fn_name, _, _, fn_impl) in ops::INDEX_MULTI {
            let name: &'static str = fn_name();
            if name != op_name {
                continue;
            }

            match fn_impl(args) {
                Ok(output) => output.print(conf),
                Err(e) => eprintln!("ERROR({}): {}", name, e),
            }
        }
    }
}
