use crate::error::FlagError;

/// `Flag` type.
///
/// Option flag struct
#[derive(Clone, Debug)]
pub struct Flag {
    /// Flag name
    pub name: String,
    /// Flag description
    pub description: Option<String>,
    /// Flag type
    pub flag_type: FlagType,
    /// Flag alias
    pub alias: Option<Vec<String>>,
}

/// `FlagType` enum
#[derive(PartialEq, Clone, Debug)]
pub enum FlagType {
    Bool,
    String,
    Int,
    Float,
}

/// `FlagValue` enum
#[derive(PartialEq, Clone, Debug)]
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
    /// let bool_flag = Flag::new("bool", FlagType::Bool);
    /// let float_flag = Flag::new("float", FlagType::Float);
    /// ```
    pub fn new<T: Into<String>>(name: T, flag_type: FlagType) -> Self {
        let name = name.into();
        if name.starts_with('-') {
            panic!(
                r#""{}" is invalid flag name. Flag name cannnot start with "-"."#,
                name
            )
        }
        if name.contains('=') {
            panic!(
                r#""{}" is invalid flag name. Flag name cannnot contain "="."#,
                name
            )
        }
        if name.contains(' ') {
            panic!(
                r#""{}" is invalid flag name. Flag name cannnot contain whitespaces."#,
                name
            )
        }

        Self {
            name,
            description: None,
            flag_type,
            alias: None,
        }
    }

    /// Set description of the flag
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Flag, FlagType};
    ///
    /// let bool_flag = Flag::new("bool", FlagType::Bool)
    ///     .description("cli cmd Hello --bool");
    /// ```
    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set alias of the flag
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Flag, FlagType};
    ///
    /// let bool_flag = Flag::new("bool", FlagType::Bool)
    ///     .alias("b");
    ///
    /// let string_flag = Flag::new("string", FlagType::String)
    ///     .alias("s")
    ///     .alias("str");
    /// ```
    pub fn alias<T: Into<String>>(mut self, name: T) -> Self {
        if let Some(ref mut alias) = self.alias {
            (*alias).push(name.into());
        } else {
            self.alias = Some(vec![name.into()]);
        }
        self
    }

    /// Get flag position from command line argument
    pub fn option_index(&self, v: &[String]) -> Option<usize> {
        match &self.alias {
            Some(alias) => v.iter().position(|r| {
                r == &format!("--{}", &self.name) || alias.iter().any(|a| r == &format!("-{}", a))
            }),
            None => v.iter().position(|r| r == &format!("--{}", &self.name)),
        }
    }

    /// Get flag value
    pub fn value(&self, v: Option<String>) -> Result<FlagValue, FlagError> {
        match self.flag_type {
            FlagType::Bool => Ok(FlagValue::Bool(true)),
            FlagType::String => match v {
                Some(s) => Ok(FlagValue::String(s)),
                None => Err(FlagError::ArgumentError),
            },
            FlagType::Int => match v {
                Some(i) => match i.parse::<isize>() {
                    Ok(i) => Ok(FlagValue::Int(i)),
                    Err(_) => Err(FlagError::ValueTypeError),
                },
                None => Err(FlagError::ArgumentError),
            },
            FlagType::Float => match v {
                Some(f) => match f.parse::<f64>() {
                    Ok(f) => Ok(FlagValue::Float(f)),
                    Err(_) => Err(FlagError::ValueTypeError),
                },
                None => Err(FlagError::ArgumentError),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Flag, FlagType, FlagValue};

    #[test]
    fn opiton_index() {
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "-a".to_string(),
            "--bool".to_string(),
            "-c".to_string(),
        ];
        {
            let f = Flag::new("bool", FlagType::Bool);
            assert_eq!(f.option_index(&v), Some(3));
        }
        {
            let f = Flag::new("age", FlagType::Bool).alias("a");
            assert_eq!(f.option_index(&v), Some(2));
        }
        {
            let f = Flag::new("dance", FlagType::Bool);
            assert_eq!(f.option_index(&v), None);
        }
    }

    #[test]
    #[should_panic]
    fn construct_fail_1() {
        Flag::new("bo=ol", FlagType::Bool);
    }

    #[test]
    #[should_panic]
    fn construct_fail_2() {
        Flag::new("------bool", FlagType::Bool);
    }

    #[test]
    #[should_panic]
    fn construct_fail_3() {
        Flag::new("cool flag", FlagType::Bool);
    }

    #[test]
    fn bool_flag_test() {
        let bool_flag = Flag::new("bool", FlagType::Bool);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--bool".to_string(),
        ];

        match bool_flag.value(Some(v[3].to_owned())) {
            Ok(FlagValue::Bool(val)) => assert!(val),
            _ => assert!(false),
        }
    }

    #[test]
    fn string_flag_test() {
        let string_flag = Flag::new("string", FlagType::String);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--string".to_string(),
            "test".to_string(),
        ];

        match string_flag.value(Some(v[4].to_owned())) {
            Ok(FlagValue::String(val)) => assert_eq!("test".to_string(), val),
            _ => assert!(false),
        }
    }

    #[test]
    fn int_flag_test() {
        let int_flag = Flag::new("int", FlagType::Int);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--int".to_string(),
            "100".to_string(),
        ];

        match int_flag.value(Some(v[4].to_owned())) {
            Ok(FlagValue::Int(val)) => assert_eq!(100, val),
            _ => assert!(false),
        }
    }

    #[test]
    fn float_flag_test() {
        let float_flag = Flag::new("float", FlagType::Float);
        let v = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ];

        match float_flag.value(Some(v[4].to_owned())) {
            Ok(FlagValue::Float(val)) => assert_eq!(1.23, val),
            _ => assert!(false),
        }
    }
}
