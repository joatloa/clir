use std::collections::HashMap;

use crate::ParsedArgs;

// ----- ParsedArg
#[derive(Debug)]
pub enum ParsedArg {
    None,
    Value(String),
    Variadic(Vec<String>),
}
impl ParsedArg {
    pub fn is_none(&self) -> bool {
        matches!(self, ParsedArg::None)
    }
    pub fn as_value(self) -> String {
        let Self::Value(v) = self else {
            panic!("ERROR: Cast `ParsedArg` as value failed");
        };

        v
    }
    pub fn as_variadic(self) -> Vec<String> {
        let Self::Variadic(v) = self else {
            panic!("ERROR: Cast `ParsedArg` as variadic failed");
        };

        v
    }
}

// ----- ParsedOpt
#[derive(Debug)]
pub enum ParsedOpt {
    None,
    Flag(bool),
    Value(String),
    Args(ParsedArgs),
}

impl ParsedOpt {
    pub fn is_none(&self) -> bool {
        matches!(self, ParsedOpt::None)
    }
    pub fn as_flag(self) -> bool {
        let Self::Flag(v) = self else {
            panic!("ERROR: Cast `ParsedOpt` as flag failed");
        };

        v
    }
    pub fn as_value(self) -> String {
        let Self::Value(v) = self else {
            panic!("ERROR: Cast `ParsedOpt` as value failed");
        };

        v
    }
    pub fn as_args(self) -> HashMap<String, ParsedArg> {
        let Self::Args(v) = self else {
            panic!("ERROR: Cast `ParsedOpt` as args failed");
        };

        v
    }
}
