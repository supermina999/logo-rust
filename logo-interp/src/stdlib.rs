use crate::core::*;

use crate::executor::execute;
use crate::executor_state::*;

pub fn add_stdlib<S: 'static>(es: &mut EState<S>) {
    es.functions.insert("repeat".to_string(), Function::from_proc2(repeat::<S>));
    es.functions.insert("show".to_string(), Function::from_proc1(show::<S>));
    es.functions.insert("sum".to_string(), Function::from_fn2(sum::<S>));
}

fn repeat<S>(state: &mut EState<S>, n: i32, cmd: Vec<LogoValue>) -> Result<(), String> {
    for _ in 0..n {
        execute(state, &cmd)?;
    }
    Ok(())
}

fn sum<S>(_: &mut EState<S>, x: f64, y: f64) -> Result<f64, String> {
    Ok(x + y)
}

fn show<S>(_: &mut EState<S>, val: LogoValue) -> Result<(), String> {
    println!("{:?}", val);
    Ok(())
}
