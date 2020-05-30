use crate::error::FlagError;
use crate::{Flag, FlagType, FlagValue};

/// `Context` type
///
/// This type is used only for `Action` arguments
pub struct Context {
    /// `Vec<String>` with flags and flag values ​​removed from command line arguments
    pub args: Vec<String>,
    /// `Vec` that stores flag name and flag value as tuple
    flags: Option<Vec<(String, Result<FlagValue, FlagError>)>>,
    help_text: String,
}

impl Context {
    /// Create new instance of `Context`
    /// Parse processing using `Vec<String>` command line argument and `Vec<Flag>` as arguments
    pub fn new(args: Vec<String>, flags: Option<Vec<Flag>>, help_text: String) -> Self {
        let mut v = Vec::new();
        let mut parsed_args = args;
        let flags_val = match flags {
            Some(flags) => {
                for flag in flags {
                    if let Some(index) = flag.option_index(&parsed_args) {
                        parsed_args.remove(index);

                        let val = if flag.flag_type != FlagType::Bool {
                            if parsed_args.len() <= index {
                                None
                            } else {
                                Some(parsed_args.remove(index))
                            }
                        } else {
                            None
                        };
                        v.push((flag.name.to_string(), flag.value(val)))
                    } else {
                        v.push((flag.name.to_string(), Err(FlagError::NotFound)))
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
    fn result_flag_value(&self, name: &str) -> Result<FlagValue, FlagError> {
        let flag = self
            .flags
            .as_ref()
            .and_then(|flags| flags.iter().find(|flag| flag.0 == name));

        match flag {
            Some(f) => match &f.1 {
                Ok(val) => Ok(val.to_owned()),
                Err(e) => Err(e.to_owned()),
            },
            None => Err(FlagError::Undefined),
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
            Ok(FlagValue::Bool(val)) => val,
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
    /// fn action(c: &Context) {
    ///     match c.string_flag("string") {
    ///         Ok(s) => println!("{}", s),
    ///         Err(e) => println!("{}", e)
    ///     }
    /// }
    /// ```
    pub fn string_flag(&self, name: &str) -> Result<String, FlagError> {
        let r = self.result_flag_value(name)?;
        match r {
            FlagValue::String(val) => Ok(val),
            _ => Err(FlagError::TypeError),
        }
    }

    /// Get int flag
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Context;
    ///
    /// fn action(c: &Context) {
    ///     match c.int_flag("int") {
    ///         Ok(i) => println!("{}", i),
    ///         Err(e) => println!("{}", e)
    ///     }
    /// }
    /// ```
    pub fn int_flag(&self, name: &str) -> Result<isize, FlagError> {
        let r = self.result_flag_value(name)?;
        match r {
            FlagValue::Int(val) => Ok(val),
            _ => Err(FlagError::TypeError),
        }
    }

    /// Get float flag
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Context;
    ///
    /// fn action(c: &Context) {
    ///     match c.float_flag("float") {
    ///         Ok(f) => println!("{}", f),
    ///         Err(e) => println!("{}", e)
    ///     }
    /// }
    /// ```
    pub fn float_flag(&self, name: &str) -> Result<f64, FlagError> {
        let r = self.result_flag_value(name)?;
        match r {
            FlagValue::Float(val) => Ok(val),
            _ => Err(FlagError::TypeError),
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
    use crate::error::FlagError;
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
            "--invalid_float".to_string(),
            "invalid".to_string(),
        ];
        let flags = vec![
            Flag::new("bool", FlagType::Bool),
            Flag::new("string", FlagType::String),
            Flag::new("int", FlagType::Int),
            Flag::new("float", FlagType::Float),
            Flag::new("invalid_float", FlagType::Float),
            Flag::new("not_specified", FlagType::String),
        ];
        let context = Context::new(args, Some(flags), "".to_string());

        assert_eq!(context.bool_flag("bool"), true);
        assert_eq!(context.string_flag("string"), Ok("test".to_string()));
        assert_eq!(context.int_flag("int"), Ok(100));
        assert_eq!(context.float_flag("float"), Ok(1.23));

        // string value arg, string flag, used as int
        assert_eq!(context.int_flag("string"), Err(FlagError::TypeError));
        // string value arg, float flag, used as float
        assert_eq!(
            context.float_flag("invalid_float"),
            Err(FlagError::ValueTypeError)
        );
        // use a flag whose name is not defined as flag
        assert_eq!(
            context.string_flag("not_registered"),
            Err(FlagError::Undefined)
        );
        // use a flag but it's value not passed
        assert_eq!(
            context.string_flag("not_specified"),
            Err(FlagError::NotFound)
        );
    }
}
