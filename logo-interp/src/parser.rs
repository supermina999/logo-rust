use std::collections::{HashMap, HashSet};
use crate::core::*;

lazy_static! {
    static ref TERMINATOR_CHARS: HashSet<char>
        = HashSet::from(['[', ']', '(', ')', '+', '-', '*', '/', '=']);
}

fn is_terminator_char(ch: char) -> bool {
    return ch.is_whitespace() || TERMINATOR_CHARS.contains(&ch);
}

pub fn parse(source: &str) -> Result<Vec<LogoValue>, String> {
    #[derive(PartialEq)]
    enum Mode {
        None,
        Word,
        DoubleQuoteString,
        SingleQuoteString,
    }
    let mut mode = Mode::None;
    let mut pending_word = String::new();

    let mut list_stack: Vec<Vec<LogoValue>> = Vec::new();
    list_stack.push(Vec::new());
    for ch in source.chars() {
        if (mode == Mode::Word || mode == Mode::DoubleQuoteString) && is_terminator_char(ch) {
            if mode == Mode::Word {
                list_stack.last_mut().unwrap().push(LogoValue::Word(Word(pending_word)));
            }
            else {
                list_stack.last_mut().unwrap().push(LogoValue::String(pending_word));
            }
            pending_word = String::new();
            mode = Mode::None;
        }
        if mode == Mode::SingleQuoteString && ch == '\'' {
            list_stack.last_mut().unwrap().push(LogoValue::String(pending_word));
            pending_word = String::new();
            mode = Mode::None;
            continue;
        }

        if mode != Mode::None {
            pending_word.push(ch);
            continue;
        }

        if ch.is_whitespace() {}
        else if ch == '[' {
            list_stack.push(Vec::new());
        }
        else if ch == ']' {
            let last_list = list_stack.pop().unwrap();
            match list_stack.last_mut() {
                Some(stack) => stack.push(LogoValue::List(last_list)),
                None => return Err("Not matched closing bracket".to_string())
            }
        }
        else if ch == '"' {
            mode = Mode::DoubleQuoteString;
        }
        else if ch == '\'' {
            mode = Mode::SingleQuoteString;
        }
        else if TERMINATOR_CHARS.contains(&ch) {
            list_stack.last_mut().unwrap().push(LogoValue::Word(Word(ch.to_string())));
        }
        else {
            mode = Mode::Word;
            pending_word = String::from(ch);
        }
    }
    match mode {
        Mode::None => {},
        Mode::Word => list_stack.last_mut().unwrap().push(LogoValue::Word(Word(pending_word))),
        Mode::DoubleQuoteString => list_stack.last_mut().unwrap().push(LogoValue::String(pending_word)),
        Mode::SingleQuoteString => {
            return Err(String::from("Missing closing quote"))
        }
    }
    if list_stack.len() > 1 {
        return Err(String::from("Missing closing bracket"));
    }
    return Ok(list_stack.pop().unwrap());
}

pub fn parse_procedures(source: &str) -> Result<HashMap<String, LogoProcedure>, String> {
    let mut result = HashMap::new();
    let mut name = String::new();
    let mut arg_names = Vec::new();
    let mut code = Vec::new();
    let values = parse(source)?;
    #[derive(PartialEq)]
    enum Mode {
        None,
        Name,
        Params,
        Body
    }
    let mut mode = Mode::None;

    for value in values {
        if mode == Mode::None {
            if let LogoValue::Word(word) = &value {
                if word.0.to_lowercase() == "to" {
                    mode = Mode::Name;
                    continue;
                }
            }
        }
        if mode == Mode::Name {
            if let LogoValue::Word(word) = &value {
                name = word.0.to_lowercase();
                mode = Mode::Params;
                continue;
            }
        }
        if mode == Mode::Params {
            if let LogoValue::Word(word) = &value {
                if let Some(arg_name) = word.0.strip_prefix(":") {
                    arg_names.push(arg_name.to_lowercase());
                    continue;
                }
            }
            mode = Mode::Body;
        }
        if mode == Mode::Body {
            if let LogoValue::Word(word) = &value {
                if word.0.to_lowercase() == "end" {
                    mode = Mode::None;
                    result.insert(name, LogoProcedure {arg_names, code});
                    name = String::new();
                    arg_names = Vec::new();
                    code = Vec::new();
                    continue;
                }
            }
            code.push(value);
            continue;
        }
        return Err("Invalid procedure syntax".to_string());
    }

    if mode != Mode::None {
        return Err("Invalid procedure syntax".to_string());
    }

    Ok(result)
}

#[test]
fn test_loop_parsing() {
    let result = parse("repeat 12  [rt 30 repeat 4 [fd   50 rt 90]]");
    let expected = vec![
        LogoValue::Word(Word("repeat".to_string())),
        LogoValue::Word(Word("12".to_string())),
        LogoValue::List(vec![
            LogoValue::Word(Word("rt".to_string())),
            LogoValue::Word(Word("30".to_string())),
            LogoValue::Word(Word("repeat".to_string())),
            LogoValue::Word(Word("4".to_string())),
            LogoValue::List(vec![
                LogoValue::Word(Word("fd".to_string())),
                LogoValue::Word(Word("50".to_string())),
                LogoValue::Word(Word("rt".to_string())),
                LogoValue::Word(Word("90".to_string())),
            ])
        ])
    ];
    assert_eq!(result, Ok(expected));
}

#[test]
fn test_strings() {
    let result = parse("\"hello world 'long string' blah");
    let expected = vec![
        LogoValue::String("hello".to_string()),
        LogoValue::Word(Word("world".to_string())),
        LogoValue::String("long string".to_string()),
        LogoValue::Word(Word("blah".to_string())),
    ];
    assert_eq!(result, Ok(expected))
}

#[test]
fn test_errors() {
    let result = parse("[[]");
    assert_eq!(result, Err("Missing closing bracket".to_string()));
    let result = parse("[]]");
    assert_eq!(result, Err("Not matched closing bracket".to_string()));
    let result = parse("blah 'long string");
    assert_eq!(result, Err("Missing closing quote".to_string()));
}
