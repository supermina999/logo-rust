use std::collections::HashMap;
use std::mem::swap;
use crate::core::{LogoValue, Word};
use crate::executor_state::*;
use crate::parser;

pub fn execute_str<S>(state: &mut EState<S>, proc_source: &str, source: &str) -> Result<(), String> {
    state.logo_procedures = parser::parse_procedures(proc_source)?;
    execute(state, parser::parse(source)?)
}

pub fn execute<S>(state: &mut EState<S>, source: Vec<LogoValue>) -> Result<(), String> {
    let transformed_source = math_transform(source)?;
    let mut it = transformed_source.iter();
    while it.len() > 0 {
        match execute_expr(state, &mut it)? {
            Some(val) => return Err(format!("Don't know what to do with {}", val)),
            None => {}
        }
    }
    Ok(())
}

fn execute_expr<'a, S>(state: &mut EState<S>, it: &mut impl Iterator<Item = &'a LogoValue>) -> Result<Option<LogoValue>, String>
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
            let proc_result = execute(state, logo_proc.code);
            restore_vars(state, backup);
            return match proc_result {
                Err(err) => {
                    if err == "Output" {
                        let output = state.output.clone();
                        state.output = None;
                        return Ok(output);
                    }
                    Err(err)
                },
                Ok(()) => Ok(None)
            }
        }
        return Err(format!("Don't know what to do with {}", cmd))
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

fn math_transform(source: Vec<LogoValue>) -> Result<Vec<LogoValue>, String> {
    let mut tree = BracketTree::parse(source)?;
    process_math_signs(&mut tree, &HashMap::from([
        ("*".to_string(), "product".to_string()),
        ("/".to_string(), "quotient".to_string()),
    ]))?;
    process_math_signs(&mut tree, &HashMap::from([
        ("+".to_string(), "sum".to_string()),
        ("-".to_string(), "difference".to_string()),
    ]))?;
    process_math_signs(&mut tree, &HashMap::from([
        (">".to_string(), "greater?".to_string()),
        ("<".to_string(), "less?".to_string()),
        ("=".to_string(), "equal?".to_string()),
    ]))?;
    Ok(tree.to_list())
}

fn process_math_signs(tree: &mut BracketTree, signs: &HashMap<String, String>) -> Result<(), String> {
    tree.process(&|nodes: Vec<BracketTreeChild>| -> Result<Vec<BracketTreeChild>, String> {
        let mut result = Vec::new();
        let mut it = nodes.into_iter();
        while let Some(node) = it.next() {
            if let BracketTreeChild::Value(val) = &node {
                if let LogoValue::Word(word) = val {
                    if let Some(sing_op) = signs.get(word.0.as_str()) {
                        let prev = match result.pop() {
                            Some(val) => val,
                            None => return Err(format!("Missing first argument for {}", word.0))
                        };
                        let next = match it.next() {
                            Some(val) => val,
                            None => return Err(format!("Missing second argument for {}", word.0))
                        };
                        let subtree = BracketTree{
                            children: vec![
                                BracketTreeChild::Value(LogoValue::Word(Word(sing_op.clone()))),
                                prev,
                                next
                            ]
                        };
                        result.push(BracketTreeChild::Tree(Box::new(subtree)));
                        continue;
                    }
                }
            }
            result.push(node);
        }
        Ok(result)
    })
}

struct BracketTree {
    children: Vec<BracketTreeChild>
}

enum BracketTreeChild {
    Value(LogoValue),
    Tree(Box<BracketTree>)
}

impl BracketTree {
    fn new() -> Self {
        BracketTree {children: Vec::new()}
    }

    fn parse(list: Vec<LogoValue>) -> Result<Self, String> {
        let mut stack = Vec::new();
        stack.push(BracketTree::new());
        for el in list {
            if let LogoValue::Word(word) = &el {
                if word.0 == "(" {
                    stack.push(BracketTree::new());
                    continue;
                }
                else if word.0 == ")" {
                    if stack.len() == 1 {
                        return Err("Missing corresponding opening bracket for ')'".to_string());
                    }
                    let last_stack = stack.pop().unwrap();
                    stack.last_mut().unwrap().children.push(BracketTreeChild::Tree(Box::new(last_stack)));
                    continue;
                }
            }
            stack.last_mut().unwrap().children.push(BracketTreeChild::Value(el));
        }
        if stack.len() > 1 {
            return Err("Missing corresponding closing bracket for '('".to_string());
        }
        Ok(stack.pop().unwrap())
    }

    fn into_list(self, list: &mut Vec<LogoValue>) {
        for child in self.children {
            match child {
                BracketTreeChild::Value(val) => {
                    list.push(val)
                },
                BracketTreeChild::Tree(tree) => {
                    tree.into_list(list);
                }
            }
        }
    }

    fn to_list(self) -> Vec<LogoValue> {
        let mut result = Vec::new();
        self.into_list(&mut result);
        result
    }

    fn process(&mut self, f: &impl Fn(Vec<BracketTreeChild>) -> Result<Vec<BracketTreeChild>, String>) -> Result<(), String> {
        let mut tmp = Vec::new();
        swap(&mut tmp, &mut self.children);
        self.children = f(tmp)?;
        for child in &mut self.children {
            if let BracketTreeChild::Tree(tree) = child {
                tree.process(f)?;
            }
        }
        Ok(())
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

    execute_str(&mut state, "", "add 5 repeat 4 [add 1] add 10").unwrap();
    assert_eq!(state.state.total, 19);

    state.state.total = 0;
    execute_str(&mut state, "", "make 'hi' 5 add :hi add thing 'hi'").unwrap();
    assert_eq!(state.state.total, 10);

    state.state.total = 0;
    execute_str(&mut state, "to add4 add 4 end \
        to add_double :x add :x add :x end \
        to double :x output sum :x :x end",
 "add4 add_double 6 add double 3").unwrap();
    assert_eq!(state.state.total, 22);
}

#[test]
fn test_execution_math() {
    use crate::stdlib::*;

    struct S {
        result: i32
    }
    let mut state = EState::new(S{result: 0});
    add_stdlib(&mut state);
    state.functions.insert("return".to_string(), Function::from_proc1(|s: &mut EState<S>, x: i32| -> Result<(), String> {
        s.state.result = x;
        Ok(())
    }));

    execute_str(&mut state, "", "return 2 + 3").unwrap();
    assert_eq!(state.state.result, 5);

    execute_str(&mut state, "", "return product 2 3 + sum 4 5").unwrap();
    assert_eq!(state.state.result, 24);

    execute_str(&mut state, "", "return (product 2 3) + (sum 4 5)").unwrap();
    assert_eq!(state.state.result, 15);

    execute_str(&mut state, "", "return 3 + 4 * 5 + 2").unwrap();
    assert_eq!(state.state.result, 25);

    execute_str(&mut state, "", "return (3 + 4) * (5 + 2)").unwrap();
    assert_eq!(state.state.result, 49);

    execute_str(&mut state, "", "return (1 + (3 + 4)) * ((5 + 2) + 2)").unwrap();
    assert_eq!(state.state.result, 72);
}

#[test]
fn test_execution_comparison() {
    use crate::stdlib::*;

    struct S {
        result: bool
    }
    let mut state = EState::new(S{result: false});
    add_stdlib(&mut state);
    state.functions.insert("return".to_string(), Function::from_proc1(|s: &mut EState<S>, x: bool| -> Result<(), String> {
        s.state.result = x;
        Ok(())
    }));

    execute_str(&mut state, "", "return 2 = 2").unwrap();
    assert_eq!(state.state.result, true);

    execute_str(&mut state, "", "return 2 = 4 / 2").unwrap();
    assert_eq!(state.state.result, true);

    execute_str(&mut state, "", "return 1 = pi / pi").unwrap();
    assert_eq!(state.state.result, true);

    execute_str(&mut state, "", "return 1/3 = 2/6").unwrap();
    assert_eq!(state.state.result, true);

    execute_str(&mut state, "", "return 1/4 < 1/5").unwrap();
    assert_eq!(state.state.result, false);

    execute_str(&mut state, "", "return (ln 1) > 0").unwrap();
    assert_eq!(state.state.result, false);

    execute_str(&mut state, "", "return (1 / 0) > 0").unwrap();
    assert_eq!(state.state.result, false);
}
