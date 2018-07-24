extern crate serde_json;
extern crate valico;
extern crate failure;

use valico::json_schema::Scope;
use valico::json_schema::validators::ValidationState;
use failure::{Error, err_msg};
use serde_json::Value;

static RTJ_SCHEMA: &'static str = include_str!("rtj-schema.json");

pub fn validate(rtj_s: &str) -> Result<Option<ValidationState>, Error> {
    let rtj_j = serde_json::from_str(&rtj_s)?;
    let schema_j = serde_json::from_str(RTJ_SCHEMA)?;

    let mut scope = Scope::new();
    let schema = scope.compile_and_return(schema_j, true)
        .map_err(|e| err_msg(format!("{:?}", e)))?;

    let rtj_doc = extract_doc(rtj_j)?;
    let validation = schema.validate(&rtj_doc);
    if validation.is_valid() {
       Ok(None) 
    } else {
        //Err(err_msg(format!("{:#?}", validation)))
        //Err(validation.errors)
        Ok(Some(validation))
    }
}

fn extract_doc(mut rtj: Value) -> Result<Value, Error> {
    rtj.get_mut("document")
        .map(|d| d.take())
        .ok_or(err_msg("Missing document node"))
}

pub fn pretty_error2(state: &ValidationState) -> String {
    use std::fmt::Write;
    let mut buf = String::new();
    let s = serde_json::to_string_pretty(&state).unwrap(); // TODO
    writeln!(buf, "{}", s).unwrap(); // TODO
    buf
}

pub fn pretty_error(state: &ValidationState) -> String {
    let mut buf = String::new();
    pretty_error_(state, &mut buf, 0);
    buf
}

// TODO recursion bad?
fn pretty_error_(state: &ValidationState, buf: &mut String, rec: usize) {
    use valico::json_schema::errors::*;
    use std::fmt::Write;
    use std::iter;

    assert!(state.missing.is_empty()); // TODO

    let indent_spaces = 2;
    let t: String = iter::repeat(' ').take(rec * indent_spaces).collect();

    for err in &state.errors {
        let write_err = |buf: &mut String| {
            let _ = writeln!(buf, "{}fragment: {}", t, err.get_fragment().join("/"));
            let _ = writeln!(buf, "{}code: {}", t, err.get_code());
            let _ = writeln!(buf, "{}path: {}", t, err.get_path());
            let _ = writeln!(buf, "{}title: {}", t, err.get_title());
            if let Some(deetz) = err.get_detail() {
                let _ = writeln!(buf, "{}detail: {}", t, deetz);
            }
        };
        if let Some(err) = err.downcast::<AnyOf>() {
            write_err(buf);
            let _ = writeln!(buf, "{}--[\n", t);
            for s in &err.states {
                pretty_error_(s, buf, rec + 1);
            }
            let _ = writeln!(buf, "{}]--", t);
            let _ = writeln!(buf, "{}^ resuming at...", t);
            write_err(buf);
            let _ = writeln!(buf);
        } else if let Some(err) = err.downcast::<OneOf>() {
            write_err(buf);
            let _ = writeln!(buf, "{}--[\n", t);
            for s in &err.states {
                pretty_error_(s, buf, rec + 1);
            }
            let _ = writeln!(buf, "{}]--", t);
            let _ = writeln!(buf, "{}^ resuming at...", t);
            write_err(buf);
            let _ = writeln!(buf);
        } else {
            write_err(buf);
            let _ = writeln!(buf);
        }
    }
}

pub fn prune_state(st: &mut ValidationState) {
    let max_depth = find_max_depth(st);

    delete_short_traces(st, max_depth, 0);
}

fn find_max_depth(st: &ValidationState) -> usize {
    find_max_depth_(st, 0)
}

fn find_max_depth_(st: &ValidationState, d: usize) -> usize {
    use valico::json_schema::errors::*;
    st.errors.iter().map(|err| {
        if let Some(err) = err.downcast::<AnyOf>() {
            err.states.iter().map(|s| find_max_depth_(s, d + 1)).max()
        } else if let Some(err) = err.downcast::<OneOf>() {
            err.states.iter().map(|s| find_max_depth_(s, d + 1)).max()
        } else {
            Some(d)
        }.unwrap_or(d)
    }).max().unwrap_or(d)
}

fn delete_short_traces(st: &mut ValidationState, max_d: usize, d: usize) -> usize {
    use valico::json_schema::errors::*;
    let errors = ::std::mem::replace(&mut st.errors, vec![]);
    use std::iter::*;
    let mut this_max_d = d;
    let errors = {
        let errors = errors.into_iter().map(|mut err| {
            let next_d = if let Some(err) = err.downcast_mut::<AnyOf>() {
                err.states.iter_mut().map(|s| delete_short_traces(s, max_d, d + 1)).max()
            } else {
                None
            }.unwrap_or(d);

            let next_d2 = if let Some(err) = err.downcast_mut::<OneOf>() {
                err.states.iter_mut().map(|s| delete_short_traces(s, max_d, d + 1)).max()
            } else {
                None
            }.unwrap_or(d);

            (err, ::std::cmp::max(next_d, next_d2))
        }).filter_map(|(err, next_d)| {
            if next_d > this_max_d {
                this_max_d = next_d;
            }
            if next_d + 1 >= max_d {
                Some(err)
            } else {
                None
            }
        });
        errors.collect()
    };
    ::std::mem::replace(&mut st.errors, errors);
    this_max_d
}
