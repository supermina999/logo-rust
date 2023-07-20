use crate::core::{LogoValue, Word};
use crate::executor_state::*;
use crate::parser;

pub fn execute_str<S>(state: &mut EState<S>, proc_source: &str, source: &str) -> Result<(), String> {
    state.logo_procedures = parser::parse_procedures(proc_source)?;
    execute(state, &parser::parse(source)?)
}

pub fn execute<S>(state: &mut EState<S>, source: &Vec<LogoValue>) -> Result<(), String> {
    let mut it = source.iter();
    while it.len() > 0 {
        match execute_expr(state, &mut it)? {
            Some(val) => return Err(format!("Don't know what to do with {:?}", val)),
            None => {}
        }
    }
    Ok(())
}

pub fn execute_expr<'a, S>(state: &mut EState<S>, it: &mut impl Iterator<Item = &'a LogoValue>) -> Result<Option<LogoValue>, String>
{
    let cmd = it.next();
    if cmd.is_none() {
        return Ok(None)
    }
    let cmd = cmd.unwrap();
    if let LogoValue::Word(word) = cmd {
        let word = &word.0;
        if let Ok(_) = word.parse::<f64>() {
            return Ok(Some(LogoValue::Word(Word(word.clone()))))
        }

        let word = word.to_lowercase();
        if let Some(var_name) = word.strip_prefix(":") {
            if !state.vars.contains_key(var_name) {
                return Err("No such variable".to_string());
            }
            return Ok(Some(state.vars[var_name].clone()));
        }

        let fun = state.functions.get(word.as_str());
        if let Some(fun) = fun {
            let f = fun.f.clone();
            let mut args = Vec::with_capacity(fun.args as usize);
            for _ in 0..fun.args {
                let arg = execute_expr(state, it)?;
                match arg {
                    Some(arg) => args.push(arg),
                    None => return Err(format!("Missing argument for {}", word))
                }
            }
            return (f)(state, args);
        }

        if let Some(logo_proc) = state.logo_procedures.get(word.as_str()) {
            let logo_proc = logo_proc.clone();
            let backup = backup_vars(state, &logo_proc.arg_names);
            for arg_name in &logo_proc.arg_names {
                let expr_result = execute_expr(state, it);
                if let Err(_) = &expr_result {
                    restore_vars(state, backup);
                    return Err(expr_result.err().unwrap());
                }
                let expr_result = expr_result.unwrap();
                if expr_result.is_none() {
                    restore_vars(state, backup);
                    return Err(format!("Missing argument for {}", word));
                }
                state.vars.insert(arg_name.clone(), expr_result.unwrap());
            }
            let proc_result = execute(state, &logo_proc.code);
            restore_vars(state, backup);
            proc_result?;
            return Ok(None);
        }
        return Err(format!("Don't know what to do with {:?}", cmd))
    }

    return Ok(Some(cmd.clone()));
}

fn backup_vars<S>(state: &EState<S>, var_names: &Vec<String>) -> Vec<(String, Option<LogoValue>)> {
    let mut result = Vec::with_capacity(var_names.len());
    for var_name in var_names {
        let val = state.vars.get(var_name);
        match val {
            Some(val) => result.push((var_name.clone(), Some(val.clone()))),
            None => result.push((var_name.clone(), None))
        }
    }
    result
}

fn restore_vars<S>(state: &mut EState<S>, backup: Vec<(String, Option<LogoValue>)>) {
    for (var_name, val) in backup {
        match val {
            Some(val) => {
                state.vars.insert(var_name, val);
            },
            None => {
                state.vars.remove(var_name.as_str());
            }
        }
    }
}

#[test]
fn test_execution() {
    use crate::stdlib::*;

    struct S {
        total: i32
    }
    let mut state = EState::new(S{total: 0});
    add_stdlib(&mut state);
    state.functions.insert("add".to_string(), Function::from_proc1(|s: &mut EState<S>, x: i32| -> Result<(), String> {
        s.state.total += x;
        Ok(())
    }));

    let result = execute_str(&mut state, "", "add 5 repeat 4 [add 1] add 10");
    assert!(result.is_ok());
    assert_eq!(state.state.total, 19);

    state.state.total = 0;
    let result = execute_str(&mut state, "", "make 'hi' 5 add :hi add thing 'hi'");
    assert!(result.is_ok());
    assert_eq!(state.state.total, 10);

    state.state.total = 0;
    let result = execute_str(&mut state, "to add4 add 4 end to add_double :x add :x add :x end",
                             "add4 add_double 6");
    assert!(result.is_ok());
    assert_eq!(state.state.total, 16);
}
