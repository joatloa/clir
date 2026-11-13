#[derive(Debug)]
pub enum ParseError {
    MissingCommand,
    UnknownCommand(String),
    TooManyArguments(Vec<String>),
    MissingRequiredArguments(Vec<String>),
    MissingRequiredOptions(Vec<String>),
    MissingRequiredArgumentsForOption(usize, Vec<String>), // index of the option, arguments
    InvalidOptionFlag(String),
}
