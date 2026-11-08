use crate::parse::cli::{ParsedArgs, ParsedOpts};

pub mod cli;

pub trait FromParsedArgs {
    fn from_parsed(parsed: ParsedArgs) -> Self;
}

pub trait FromParsedOpts {
    fn from_parsed(parsed: ParsedOpts) -> Self;
}
