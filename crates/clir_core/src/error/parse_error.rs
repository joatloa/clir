#[derive(Debug)]
pub enum ParseError {
    InvalidCommand(String),
    TooManyArguments(Vec<String>),
    MissingRequiredArguments(Vec<String>),
    MissingRequiredOptions(Vec<String>),
    MissingRequiredArgumentsForOption(usize, Vec<String>), // index of the option, arguments
    InvalidOptionFlag(String),
}
