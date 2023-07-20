use crate::core::{LogoValue, Word};
use crate::executor_state::*;
use crate::parser;

pub fn execute_str<S>(state: &mut EState<S>, source: &str) -> Result<(), String> {
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

        let fun = state.functions.get(word).clone();
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
        return Err(format!("Don't know what to do with {:?}", cmd))
    }

    return Ok(Some(cmd.clone()));
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
    let result = execute_str(&mut state, "add 5 repeat 4 [add 1] add 10");
    assert!(result.is_ok());
    assert_eq!(state.state.total, 19);
}
