use rand::{Rng, thread_rng};
use crate::core::*;
use crate::executor::execute;
use crate::executor_state::*;

pub fn add_stdlib<S: 'static>(es: &mut EState<S>) {
    es.functions.insert("repeat".to_string(), Function::from_proc2(repeat));
    es.functions.insert("show".to_string(), Function::from_proc1(show));

    es.functions.insert("abs".to_string(), Function::from_fn1(abs));
    es.functions.insert("arctan".to_string(), Function::from_fn1(arctan));
    es.functions.insert("cos".to_string(), Function::from_fn1(cos));
    es.functions.insert("difference".to_string(), Function::from_fn2(difference));
    es.functions.insert("exp".to_string(), Function::from_fn1(exp));
    es.functions.insert("greater?".to_string(), Function::from_fn2(greater));
    es.functions.insert("less?".to_string(), Function::from_fn2(less));
    es.functions.insert("int".to_string(), Function::from_fn1(int));
    es.functions.insert("log".to_string(), Function::from_fn1(log));
    es.functions.insert("ln".to_string(), Function::from_fn1(ln));
    es.functions.insert("minus".to_string(), Function::from_fn1(minus));
    es.functions.insert("pi".to_string(), Function::from_fn(pi));
    es.functions.insert("power".to_string(), Function::from_fn2(power));
    es.functions.insert("product".to_string(), Function::from_fn2(product));
    es.functions.insert("quotient".to_string(), Function::from_fn2(quotient));
    es.functions.insert("remainder".to_string(), Function::from_fn2(remainder));
    es.functions.insert("random".to_string(), Function::from_fn1(random));
    es.functions.insert("round".to_string(), Function::from_fn1(round));
    es.functions.insert("sin".to_string(), Function::from_fn1(sin));
    es.functions.insert("sqrt".to_string(), Function::from_fn1(sqrt));
    es.functions.insert("sum".to_string(), Function::from_fn2(sum));
    es.functions.insert("tan".to_string(), Function::from_fn1(tan));

    es.functions.insert("bf".to_string(), Function::from_fn1(bf));
    es.functions.insert("butfirst".to_string(), Function::from_fn1(bf));
    es.functions.insert("bl".to_string(), Function::from_fn1(bl));
    es.functions.insert("butlast".to_string(), Function::from_fn1(bl));
    es.functions.insert("count".to_string(), Function::from_fn1(count));
    es.functions.insert("empty?".to_string(), Function::from_fn1(empty));
    es.functions.insert("equal?".to_string(), Function::from_fn2(equal));
    es.functions.insert("identical?".to_string(), Function::from_fn2(equal));
    es.functions.insert("first".to_string(), Function::from_fn1(first));
    es.functions.insert("fput".to_string(), Function::from_fn2(fput));
    es.functions.insert("item".to_string(), Function::from_fn2(item));
    es.functions.insert("last".to_string(), Function::from_fn1(last));
    es.functions.insert("list".to_string(), Function::from_fn2(list));
    es.functions.insert("list?".to_string(), Function::from_fn1(is_list));
    es.functions.insert("lput".to_string(), Function::from_fn2(lput));
    es.functions.insert("member?".to_string(), Function::from_fn2(member));
    es.functions.insert("number?".to_string(), Function::from_fn1(number));
    es.functions.insert("pick".to_string(), Function::from_fn1(pick));
    es.functions.insert("word?".to_string(), Function::from_fn1(word));

    es.functions.insert("and".to_string(), Function::from_fn2(and));
    es.functions.insert("or".to_string(), Function::from_fn2(or));
    es.functions.insert("not".to_string(), Function::from_fn1(not));
    es.functions.insert("if".to_string(), Function::from_proc2(if_fn));
    es.functions.insert("ifelse".to_string(), Function::from_proc3(if_else_fn));

    es.functions.insert("make".to_string(), Function::from_proc2(make));
    es.functions.insert("clearname".to_string(), Function::from_proc1(clearname));
    es.functions.insert("clearnames".to_string(), Function::from_proc(clearnames));
    es.functions.insert("name?".to_string(), Function::from_fn1(name));
    es.functions.insert("names".to_string(), Function::from_fn(names));
    es.functions.insert("thing".to_string(), Function::from_fn1(thing));

    es.functions.insert("output".to_string(), Function::from_proc1(output));
}

fn repeat<S>(state: &mut EState<S>, n: i32, cmd: Vec<LogoValue>) -> Result<(), String> {
    for _ in 0..n {
        execute(state, cmd.clone())?;
    }
    Ok(())
}

fn show<S>(_: &mut EState<S>, val: LogoValue) -> Result<(), String> {
    println!("{}", val);
    Ok(())
}

fn abs<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(val.abs())
}

fn arctan<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(val.atan().to_degrees())
}

fn cos<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(val.to_radians().cos())
}

fn difference<S>(_: &mut EState<S>, a: f64, b: f64) -> Result<f64, String> {
    Ok(a - b)
}

fn exp<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(val.exp())
}

fn greater<S>(_: &mut EState<S>, a: f64, b: f64) -> Result<bool, String> {
    Ok(a > b)
}

fn less<S>(_: &mut EState<S>, a: f64, b: f64) -> Result<bool, String> {
    Ok(a < b)
}

fn int<S>(_: &mut EState<S>, val: f64) -> Result<i32, String> {
    Ok(val as i32)
}

fn log<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(val.log(10f64))
}

fn ln<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(val.ln())
}

fn minus<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(-val)
}

fn pi<S>(_: &mut EState<S>) -> Result<f64, String> {
    Ok(std::f64::consts::PI)
}

fn power<S>(_: &mut EState<S>, a: f64, b: f64) -> Result<f64, String> {
    Ok(a.powf(b))
}

fn product<S>(_: &mut EState<S>, a: f64, b: f64) -> Result<f64, String> {
    Ok(a * b)
}

fn quotient<S>(_: &mut EState<S>, a: f64, b: f64) -> Result<f64, String> {
    Ok(a / b)
}

fn remainder<S>(_: &mut EState<S>, a: i32, b: i32) -> Result<i32, String> {
    Ok(a % b)
}

fn random<S>(_: &mut EState<S>, val: i32) -> Result<i32, String> {
    if val < 1 {
        return Err("Input to random must be greater than 0".to_string());
    }
    Ok((rand::thread_rng().gen::<u32>() % val as u32) as i32)
}

fn round<S>(_: &mut EState<S>, val: f64) -> Result<i32, String> {
    Ok(val.round() as i32)
}

fn sin<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(val.to_radians().sin())
}

fn sqrt<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(val.sqrt())
}

fn sum<S>(_: &mut EState<S>, x: f64, y: f64) -> Result<f64, String> {
    Ok(x + y)
}

fn tan<S>(_: &mut EState<S>, val: f64) -> Result<f64, String> {
    Ok(val.to_radians().tan())
}


fn bf<S>(_: &mut EState<S>, mut val: Vec<LogoValue>) -> Result<Vec<LogoValue>, String> {
    if val.len() == 0 {
        return Err("Can't remove an element from an empty list".to_string());
    }
    val.remove(0);
    Ok(val)
}

fn bl<S>(_: &mut EState<S>, mut val: Vec<LogoValue>) -> Result<Vec<LogoValue>, String> {
    if val.len() == 0 {
        return Err("Can't remove an element from an empty list".to_string());
    }
    val.pop();
    Ok(val)
}

fn count<S>(_: &mut EState<S>, val: Vec<LogoValue>) -> Result<i32, String> {
    Ok(val.len() as i32)
}

fn empty<S>(_: &mut EState<S>, val: Vec<LogoValue>) -> Result<bool, String> {
    Ok(val.is_empty())
}

fn equal<S>(_: &mut EState<S>, a: LogoValue, b: LogoValue) -> Result<bool, String> {
    Ok(a == b)
}

fn first<S>(_: &mut EState<S>, val: Vec<LogoValue>) -> Result<LogoValue, String> {
    if val.len() == 0 {
        return Err("Can't get an element from an empty list".to_string());
    }
    Ok(val.first().unwrap().clone())
}

fn fput<S>(_: &mut EState<S>, a: LogoValue, mut b: Vec<LogoValue>) -> Result<Vec<LogoValue>, String> {
    b.insert(0, a);
    Ok(b)
}

fn item<S>(_: &mut EState<S>, idx: i32, val: Vec<LogoValue>) -> Result<LogoValue, String> {
    if idx < 0 || idx >= val.len() as i32 {
        return Err("No such item".to_string());
    }
    Ok(val[idx as usize].clone())
}

fn last<S>(_: &mut EState<S>, val: Vec<LogoValue>) -> Result<LogoValue, String> {
    if val.len() == 0 {
        return Err("Can't get an element from an empty list".to_string());
    }
    Ok(val.last().unwrap().clone())
}

fn list<S>(_: &mut EState<S>, mut a: Vec<LogoValue>, mut b: Vec<LogoValue>) -> Result<Vec<LogoValue>, String> {
    a.append(&mut b);
    Ok(a)
}

fn is_list<S>(_: &mut EState<S>, a: LogoValue) -> Result<bool, String> {
    if let LogoValue::List(_) = a {
        Ok(true)
    }
    else {
        Ok(false)
    }
}

fn lput<S>(_: &mut EState<S>, a: LogoValue, mut b: Vec<LogoValue>) -> Result<Vec<LogoValue>, String> {
    b.push(a);
    Ok(b)
}

fn member<S>(_: &mut EState<S>, a: LogoValue, b: Vec<LogoValue>) -> Result<bool, String> {
    for b_el in b {
        if a == b_el {
            return Ok(true)
        }
    }
    Ok(false)
}

fn number<S>(_: &mut EState<S>, a: LogoValue) -> Result<bool, String> {
    if let LogoValue::Word(word) = a {
        if let Ok(_) = word.0.parse::<f32>() {
            Ok(true)
        }
        else {
            Ok(false)
        }
    }
    else {
        Ok(false)
    }
}

fn pick<S>(_: &mut EState<S>, val: Vec<LogoValue>) -> Result<LogoValue, String> {
    if val.len() == 0 {
        return Err("Can't get an element from an empty list".to_string());
    }
    Ok(val[thread_rng().gen::<usize>() % val.len()].clone())
}

fn word<S>(_: &mut EState<S>, a: LogoValue) -> Result<bool, String> {
    if let LogoValue::Word(_) = a {
        Ok(true)
    }
    else {
        Ok(false)
    }
}


fn and<S>(_: &mut EState<S>, a: bool, b: bool) -> Result<bool, String> {
    Ok(a && b)
}

fn or<S>(_: &mut EState<S>, a: bool, b: bool) -> Result<bool, String> {
    Ok(a || b)
}

fn not<S>(_: &mut EState<S>, a: bool) -> Result<bool, String> {
    Ok(!a)
}

fn if_fn<S>(state: &mut EState<S>, a: bool, cmd: Vec<LogoValue>) -> Result<(), String> {
    if a {
        execute(state, cmd)?;
    }
    Ok(())
}

fn if_else_fn<S>(state: &mut EState<S>, a: bool, cmd_true: Vec<LogoValue>, cmd_false: Vec<LogoValue>) -> Result<(), String> {
    if a {
        execute(state, cmd_true)?;
    }
    else {
        execute(state, cmd_false)?;
    }
    Ok(())
}


fn make<S>(state: &mut EState<S>, name: String, val: LogoValue) -> Result<(), String> {
    state.vars.insert(name.to_lowercase(), val);
    Ok(())
}

fn clearname<S>(state: &mut EState<S>, name: String) -> Result<(), String> {
    state.vars.remove(name.to_lowercase().as_str());
    Ok(())
}

fn clearnames<S>(state: &mut EState<S>) -> Result<(), String> {
    state.vars.clear();
    Ok(())
}

fn name<S>(state: &mut EState<S>, name: String) -> Result<bool, String> {
    Ok(state.vars.contains_key(name.to_lowercase().as_str()))
}

fn names<S>(state: &mut EState<S>) -> Result<Vec<LogoValue>, String> {
    let mut result = Vec::with_capacity(state.vars.len());
    for key in state.vars.keys() {
        result.push(LogoValue::String(key.clone()));
    }
    Ok(result)
}

fn thing<S>(state: &mut EState<S>, name: String) -> Result<LogoValue, String> {
    let name = name.to_lowercase();
    if !state.vars.contains_key(name.as_str()) {
        return Err("No such variable".to_string());
    }
    Ok(state.vars[name.as_str()].clone())
}


fn output<S>(state: &mut EState<S>, val: LogoValue) -> Result<(), String> {
    state.output = Some(val);
    Err("Output".to_string())
}
