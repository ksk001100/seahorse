use crate::{Flag, FlagType, FlagValue};

/// `Context` type
///
/// This type is used only for `Action` arguments
pub struct Context {
    /// `Vec<String>` with flags and flag values ​​removed from command line arguments
    pub args: Vec<String>,
    /// `Vec` that stores flag name and flag value as tuple
    flags: Option<Vec<(String, Result<FlagValue, String>)>>,
    help_text: String,
}

impl Context {
    /// Create new instance of `Context`
    /// Parse processing using `Vec<String>` command line argument and `Vec<Flag>` as arguments
    pub fn new(args: Vec<String>, flags: Option<Vec<Flag>>, help_text: String) -> Self {
        let mut v = Vec::new();
        let mut parsed_args = args.clone();
        let flags_val = match flags {
            Some(flags) => {
                for flag in flags {
                    if let Some(index) = flag.option_index(&parsed_args) {
                        parsed_args.remove(index);

                        let val = if flag.flag_type != FlagType::Bool {
                            if parsed_args.is_empty() {
                                None
                            } else {
                                Some(parsed_args.remove(index))
                            }
                        } else {
                            None
                        };
                        v.push((flag.name.to_string(), flag.value(val)))
                    }
                }
                Some(v)
            }
            None => None,
        };

        Self {
            args: parsed_args,
            flags: flags_val,
            help_text,
        }
    }

    /// Get flag value
    fn result_flag_value<'a>(&self, name: &str) -> Option<Result<FlagValue, String>> {
        let flag = self
            .flags
            .as_ref()
            .and_then(|flags| flags.iter().find(|flag| flag.0 == name));

        match flag {
            Some(f) => Some(f.1.to_owned()),
            None => None,
        }
    }

    /// Get bool flag
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Context;
    ///
    /// fn action(c: &Context) {
    ///     if c.bool_flag("bool") {
    ///         println!("True!");
    ///     } else {
    ///         println!("False!");
    ///     }
    /// }
    /// ```
    pub fn bool_flag(&self, name: &str) -> bool {
        let r = self.result_flag_value(name);
        match r {
            Some(Ok(FlagValue::Bool(val))) => val,
            _ => false,
        }
    }

    /// Get string flag
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Context;
    ///
    /// match context.string_flag("string") {
    ///     Some(r) => match r {
    ///         Ok(s) => println!("{}", s),
    ///         Err(e) => println!("{}", e)
    ///     },
    ///     None => println!("Not found string...")
    /// }
    /// ```
    pub fn string_flag(&self, name: &str) -> Option<Result<String, String>> {
        match self.result_flag_value(name) {
            Some(r) => match r {
                Ok(FlagValue::String(val)) => Some(Ok(val)),
                Err(e) => Some(Err(e)),
                _ => Some(Err("".to_string())),
            },
            None => None,
        }
    }

    /// Get int flag
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Context;
    ///
    /// match context.int_flag("int") {
    ///     Some(r) => match r {
    ///         Ok(i) => println!("{}", i),
    ///         Err(e) => println!("{}", e)
    ///     }
    ///     None => println!("Not found int number...")
    /// }
    /// ```
    pub fn int_flag(&self, name: &str) -> Option<Result<isize, String>> {
        match self.result_flag_value(name) {
            Some(r) => match r {
                Ok(FlagValue::Int(val)) => Some(Ok(val)),
                Err(e) => Some(Err(e.to_owned())),
                _ => Some(Err("".to_string())),
            },
            None => None,
        }
    }

    /// Get float flag
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Context;
    ///
    /// match context.float_flag("float") {
    ///     Some(r) => match r {
    ///         Ok(f) => println!("{}", f),
    ///         Err(e) => println!("{}", e)
    ///     }
    ///     None => println!("Not found float number...")
    /// }
    /// ```
    pub fn float_flag(&self, name: &str) -> Option<Result<f64, String>> {
        match self.result_flag_value(name) {
            Some(r) => match r {
                Ok(FlagValue::Float(val)) => Some(Ok(val)),
                Err(e) => Some(Err(e.to_owned())),
                _ => Some(Err("".to_string())),
            },
            None => None,
        }
    }

    /// Display help
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Context;
    ///
    /// fn action(c: &Context) {
    ///     c.help();
    /// }
    /// ```
    pub fn help(&self) {
        println!("{}", self.help_text);
    }
}
#[cfg(test)]
mod tests {
    use crate::{Context, Flag, FlagType};

    #[test]
    fn context_test() {
        let args = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "test".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ];
        let flags = vec![
            Flag::new("bool", FlagType::Bool),
            Flag::new("string", FlagType::String),
            Flag::new("int", FlagType::Int),
            Flag::new("float", FlagType::Float),
        ];
        let context = Context::new(args, Some(flags), "".to_string());

        assert_eq!(true, context.bool_flag("bool"));

        match context.string_flag("string") {
            Ok(val) => assert_eq!("test".to_string(), val),
            _ => assert!(false),
        }

        match context.int_flag("int") {
            Ok(val) => assert_eq!(100, val),
            _ => assert!(false),
        }

        match context.float_flag("float") {
            Ok(val) => assert_eq!(1.23, val),
            _ => assert!(false),
        }
    }

    #[test]
    #[should_panic]
    fn argument_fail() {
        let args = vec![
            "cli".to_string(),
            "command".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
        ];
        let flags = vec![
            Flag::new("bool", "", FlagType::Bool),
            Flag::new("string", "", FlagType::String),
        ];

        Context::new(args, Some(flags));
    }
}
