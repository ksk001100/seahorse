use std::error;
use std::fmt;

struct ArgumentError {
    name: String,
}

impl fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Argument error: {}", self.name)
    }
}

impl error::Error for ArgumentError {
    fn description(&self) -> &str {}

    fn cause(&self) -> Option<&error::Error> {}
}

impl ArgumentError {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self { name: name.into() }
    }
}
