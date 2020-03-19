use crate::{Flag, FlagType, FlagValue};

/// `Context` type
///
/// This type is used only for `Action` arguments
pub struct Context {
    /// `Vec<String>` with flags and flag values ​​removed from command line arguments
    pub args: Vec<String>,
    /// `Vec` that stores flag name and flag value as tuple
    flags: Option<Vec<(String, Option<FlagValue>)>>,
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
                        if flag.flag_type != FlagType::Bool {
                            parsed_args.remove(index);
                        }
                    }
                    v.push((flag.name.to_string(), flag.value(&args)))
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
    fn option_flag_value(&self, name: &str) -> Option<&FlagValue> {
        self.flags
            .as_ref()
            .and_then(|flags| flags.iter().find(|flag| flag.0 == name))
            .and_then(|flag| flag.1.as_ref())
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
        match self.option_flag_value(name) {
            Some(FlagValue::Bool(val)) => *val,
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
    ///         Some(s) => println!("{}", s),
    ///         None => println!("Not found string...")
    ///     }
    /// }
    /// ```
    pub fn string_flag(&self, name: &str) -> Option<String> {
        match self.option_flag_value(name) {
            Some(FlagValue::String(val)) => Some(val.to_string()),
            _ => None,
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
    ///         Some(i) => println!("{}", i),
    ///         None => println!("Not found int number...")
    ///     }
    /// }
    /// ```
    pub fn int_flag(&self, name: &str) -> Option<isize> {
        match self.option_flag_value(name) {
            Some(FlagValue::Int(val)) => Some(*val),
            _ => None,
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
    ///         Some(f) => println!("{}", f),
    ///         None => println!("Not found float number...")
    ///     }
    /// }
    /// ```
    pub fn float_flag(&self, name: &str) -> Option<f64> {
        match self.option_flag_value(name) {
            Some(FlagValue::Float(val)) => Some(*val),
            _ => None,
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
            Some(val) => assert_eq!("test".to_string(), val),
            _ => assert!(false),
        }

        match context.int_flag("int") {
            Some(val) => assert_eq!(100, val),
            _ => assert!(false),
        }

        match context.float_flag("float") {
            Some(val) => assert_eq!(1.23, val),
            _ => assert!(false),
        }
    }
}
