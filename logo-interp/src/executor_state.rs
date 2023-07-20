use std::collections::HashMap;
use crate::core::*;


pub struct Function<State> {
    f: Box<dyn Fn(&mut State, Vec<LogoValue>) -> Result<Option<LogoValue>, String>>,
    args: i32
}

impl<State: 'static> Function<State> {
    fn from_proc(f: fn(&mut State) -> Result<(), String>) -> Self {
        return Function{f: Box::new(move |state: &mut State, _: Vec<LogoValue>| -> Result<Option<LogoValue>, String> {
            f(state)?;
            return Ok(None);
        }), args: 0};
    }
    fn from_fn<Out: LogoConvertible + 'static>(f: fn(&mut State) -> Result<Out, String>) -> Self {
        return Function{f: Box::new(move |state: &mut State, _: Vec<LogoValue>| -> Result<Option<LogoValue>, String> {
            return Ok(Some(f(state)?.to_logo()));
        }), args: 0};
    }

    fn from_proc1<T1: LogoConvertible + 'static>
    (f: fn(&mut State, T1) -> Result<(), String>) -> Self {
        return Function{f: Box::new(move |state: &mut State, mut args: Vec<LogoValue>| -> Result<Option<LogoValue>, String> {
            let arg1 = T1::from_logo(args.pop().unwrap())?;
            f(state, arg1)?;
            return Ok(None);
        }), args: 1};
    }
    fn from_fn1<T1: LogoConvertible + 'static, Out: LogoConvertible + 'static>
    (f: fn(&mut State, T1) -> Result<Out, String>) -> Self {
        return Function{f: Box::new(move |state: &mut State, mut args: Vec<LogoValue>| -> Result<Option<LogoValue>, String> {
            let arg1 = T1::from_logo(args.pop().unwrap())?;
            return Ok(Some(f(state, arg1)?.to_logo()));
        }), args: 1};
    }

    fn from_proc2<T1: LogoConvertible + 'static, T2: LogoConvertible + 'static>
    (f: fn(&mut State, T1, T2) -> Result<(), String>) -> Self {
        return Function{f: Box::new(move |state: &mut State, mut args: Vec<LogoValue>| -> Result<Option<LogoValue>, String> {
            let arg2 = T2::from_logo(args.pop().unwrap())?;
            let arg1 = T1::from_logo(args.pop().unwrap())?;
            f(state, arg1, arg2)?;
            return Ok(None);
        }), args: 2};
    }
    fn from_fn2<T1: LogoConvertible + 'static, T2: LogoConvertible + 'static, Out: LogoConvertible + 'static>
    (f: fn(&mut State, T1, T2) -> Result<Out, String>) -> Self {
        return Function{f: Box::new(move |state: &mut State, mut args: Vec<LogoValue>| -> Result<Option<LogoValue>, String> {
            let arg2 = T2::from_logo(args.pop().unwrap())?;
            let arg1 = T1::from_logo(args.pop().unwrap())?;
            return Ok(Some(f(state, arg1, arg2)?.to_logo()));
        }), args: 2};
    }

    fn from_proc3<T1: LogoConvertible + 'static, T2: LogoConvertible + 'static, T3: LogoConvertible + 'static>
    (f: fn(&mut State, T1, T2, T3) -> Result<(), String>) -> Self {
        return Function{f: Box::new(move |state: &mut State, mut args: Vec<LogoValue>| -> Result<Option<LogoValue>, String> {
            let arg3 = T3::from_logo(args.pop().unwrap())?;
            let arg2 = T2::from_logo(args.pop().unwrap())?;
            let arg1 = T1::from_logo(args.pop().unwrap())?;
            f(state, arg1, arg2, arg3)?;
            return Ok(None);
        }), args: 3};
    }
    fn from_fn3<T1: LogoConvertible + 'static, T2: LogoConvertible + 'static, T3: LogoConvertible + 'static, Out: LogoConvertible + 'static>
    (f: fn(&mut State, T1, T2, T3) -> Result<Out, String>) -> Self {
        return Function{f: Box::new(move |state: &mut State, mut args: Vec<LogoValue>| -> Result<Option<LogoValue>, String> {
            let arg3 = T3::from_logo(args.pop().unwrap())?;
            let arg2 = T2::from_logo(args.pop().unwrap())?;
            let arg1 = T1::from_logo(args.pop().unwrap())?;
            return Ok(Some(f(state, arg1, arg2, arg3)?.to_logo()));
        }), args: 3};
    }
}

pub struct ExecutorState<State> {
    functions: HashMap<String, Function<State>>
}

impl<State> ExecutorState<State> {
    fn new() -> Self {
        return ExecutorState {functions: HashMap::new()};
    }
}

#[test]
fn test_executor_function() {
    let mut state = 5;
    let mut executor_state = ExecutorState::<i32>::new();

    let sum = |_: &mut i32, x: f64, y: f64| -> Result<f64, String> {
        Ok(x + y)
    };
    executor_state.functions.insert("sum".to_string(), Function::from_fn2(sum));

    let sum_fn = &executor_state.functions[&"sum".to_string()];
    assert_eq!(sum_fn.args, 2);
    let res = (sum_fn.f)(&mut state,
                         vec![LogoValue::Word(Word("2".to_string())), LogoValue::Word(Word("3".to_string()))]);
    assert!(res.is_ok());
    assert!(res.as_ref().unwrap().is_some());
    assert_eq!(res.unwrap().unwrap(), LogoValue::Word(Word("5".to_string())));
}
