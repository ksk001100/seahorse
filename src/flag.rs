/// `Flag` type.
///
/// Option flag struct
#[derive(Clone)]
pub struct Flag {
    /// Flag name
    pub name: &'static str,
    /// Flag usage
    pub usage: &'static str,
    /// Flag type
    pub flag_type: FlagType,
}

/// `FlagType` enum
#[derive(PartialOrd, PartialEq, Clone)]
pub enum FlagType {
    Bool,
    String,
    Int,
    Float,
}

/// `FlagValue` enum
pub enum FlagValue {
    Bool(bool),
    String(String),
    Int(isize),
    Float(f64),
}

impl Flag {
    /// Create new instance of `Flag`
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Flag, FlagType};
    ///
    /// let bool_flag = Flag::new("bool", "cli cmd [arg] --bool", FlagType::Bool);
    /// let float_flag = Flag::new("float", "cli cmd [arg] --float [float]", FlagType::Float);
    /// ```
    pub fn new(name: &'static str, usage: &'static str, flag_type: FlagType) -> Self {
        Self {
            name,
            usage,
            flag_type,
        }
    }

    /// Get flag position from `Vec<String>` command line argument
    fn option_index(&self, v: &Vec<String>) -> Option<usize> {
        v.iter().position(|r| r == &format!("--{}", self.name))
    }

    /// Get flag value
    pub fn value(&self, v: &Vec<String>) -> Option<FlagValue> {
        match self.flag_type {
            FlagType::Bool => Some(FlagValue::Bool(v.contains(&format!("--{}", self.name)))),
            FlagType::String => {
                match self.option_index(&v) {
                    Some(index) => Some(FlagValue::String(v[index + 1].to_owned())),
                    None => None
                }
            }
            FlagType::Int => {
                match self.option_index(&v) {
                    Some(index) => Some(FlagValue::Int(v[index + 1].parse::<isize>().unwrap())),
                    None => None
                }
            }
            FlagType::Float => {
                match self.option_index(&v) {
                    Some(index) => Some(FlagValue::Float(v[index + 1].parse::<f64>().unwrap())),
                    None => None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Flag, FlagType, FlagValue};

    #[test]
    fn bool_flag_test() {
        let bool_flag = Flag::new("bool", "", FlagType::Bool);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--bool".to_string(),
        ];

        match bool_flag.value(&v) {
            Some(FlagValue::Bool(val)) => assert!(val),
            _ => assert!(false)
        }
    }

    #[test]
    fn string_flag_test() {
        let string_flag = Flag::new("string", "", FlagType::String);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--string".to_string(),
            "test".to_string(),
        ];

        match string_flag.value(&v) {
            Some(FlagValue::String(val)) => assert_eq!("test".to_string(), val),
            _ => assert!(false)
        }
    }

    #[test]
    fn int_flag_test() {
        let int_flag = Flag::new("int", "", FlagType::Int);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--int".to_string(),
            "100".to_string(),
        ];

        match int_flag.value(&v) {
            Some(FlagValue::Int(val)) => assert_eq!(100, val),
            _ => assert!(false)
        }
    }

    #[test]
    fn float_flag_test() {
        let float_flag = Flag::new("float", "", FlagType::Float);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ];

        match float_flag.value(&v) {
            Some(FlagValue::Float(val)) => assert_eq!(1.23, val),
            _ => assert!(false)
        }
    }
}
