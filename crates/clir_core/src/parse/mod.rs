use std::collections::HashMap;

pub mod cli;
pub mod types;

pub use types::{ParsedArg, ParsedOpt};

pub type ParsedArgs = HashMap<String, ParsedArg>;
pub trait FromParsedArgs {
    fn from_parsed(parsed: ParsedArgs) -> Self;
}

pub type ParsedOpts = HashMap<String, ParsedOpt>;
pub trait FromParsedOpts {
    fn from_parsed(parsed: ParsedOpts) -> Self;
}
