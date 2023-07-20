#[derive(Debug, PartialEq)]
pub enum LogoValue {
    Word(String),
    String(String),
    List(Vec<LogoValue>)
}
