use std::error;
use std::fmt;

#[derive(PartialEq, Clone, Debug)]
pub enum FlagError {
    NotFound,
    Undefined,
    TypeError,
    ValueTypeError,
    ArgumentError,
}

impl fmt::Display for FlagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlagError::NotFound => f.write_str("NotFound"),
            FlagError::Undefined => f.write_str("Undefined"),
            FlagError::TypeError => f.write_str("TypeError"),
            FlagError::ValueTypeError => f.write_str("ValueTypeError"),
            FlagError::ArgumentError => f.write_str("ArgumentError"),
        }
    }
}

impl error::Error for FlagError {
    fn description(&self) -> &str {
        match *self {
            FlagError::NotFound => "Flag not found",
            FlagError::Undefined => "Flag undefined",
            FlagError::TypeError => "Flag type mismatch",
            FlagError::ValueTypeError => "Value type mismatch",
            FlagError::ArgumentError => "Illegal argument",
        }
    }
}
