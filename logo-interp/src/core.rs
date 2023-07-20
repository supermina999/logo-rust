#[derive(Debug, Clone, PartialEq)]
pub struct Word(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum LogoValue {
    Word(Word),
    String(String),
    List(Vec<LogoValue>)
}

pub trait LogoConvertible {
    fn to_logo(&self) -> LogoValue;
    fn from_logo(value: LogoValue) -> Result<Self, String> where Self: Sized;
}

impl LogoConvertible for LogoValue {
    fn to_logo(&self) -> LogoValue {
        return self.clone();
    }

    fn from_logo(value: LogoValue) -> Result<Self, String> {
        return Ok(value);
    }
}

impl LogoConvertible for String {
    fn to_logo(&self) -> LogoValue {
        return LogoValue::String(self.clone());
    }

    fn from_logo(value: LogoValue) -> Result<Self, String> {
        match value {
            LogoValue::String(val) => Ok(val),
            _ => Err("Type mismatch".to_string())
        }
    }
}

impl LogoConvertible for Word {
    fn to_logo(&self) -> LogoValue {
        return LogoValue::Word(self.clone());
    }

    fn from_logo(value: LogoValue) -> Result<Self, String> {
        match value {
            LogoValue::Word(val) => Ok(val),
            _ => Err("Type mismatch".to_string())
        }
    }
}

impl LogoConvertible for f64 {
    fn to_logo(&self) -> LogoValue {
        return LogoValue::Word(Word(self.to_string()));
    }

    fn from_logo(value: LogoValue) -> Result<Self, String> {
        match value {
            LogoValue::Word(val) => {
                match val.0.parse::<f64>() {
                    Ok(val) => Ok(val),
                    _ => Err("Type mismatch".to_string())
                }
            },
            _ => Err("Type mismatch".to_string())
        }
    }
}

impl LogoConvertible for i32 {
    fn to_logo(&self) -> LogoValue {
        return LogoValue::Word(Word(self.to_string()));
    }

    fn from_logo(value: LogoValue) -> Result<Self, String> {
        match value {
            LogoValue::Word(val) => {
                match val.0.parse::<i32>() {
                    Ok(val) => Ok(val),
                    _ => Err("Type mismatch".to_string())
                }
            },
            _ => Err("Type mismatch".to_string())
        }
    }
}

impl LogoConvertible for bool {
    fn to_logo(&self) -> LogoValue {
        return LogoValue::Word(Word(self.to_string()));
    }

    fn from_logo(value: LogoValue) -> Result<Self, String> {
        match value {
            LogoValue::Word(val) => {
                match val.0.parse::<bool>() {
                    Ok(val) => Ok(val),
                    _ => Err("Type mismatch".to_string())
                }
            },
            _ => Err("Type mismatch".to_string())
        }
    }
}

impl<T: LogoConvertible> LogoConvertible for Vec<T> {
    fn to_logo(&self) -> LogoValue {
        let mut res = Vec::with_capacity(self.len());
        for value in self {
            res.push(value.to_logo());
        }
        return LogoValue::List(res);
    }

    fn from_logo(value: LogoValue) -> Result<Self, String> {
        match value {
            LogoValue::List(list) => {
                let mut res = Vec::with_capacity(list.len());
                for value in list {
                    res.push(T::from_logo(value)?);
                }
                Ok(res)
            },
            _ => Err("Type mismatch".to_string())
        }
    }
}
