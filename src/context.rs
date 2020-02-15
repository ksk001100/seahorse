use crate::{Flag, FlagValue, FlagType};

pub struct Context {
    pub args: Vec<String>,
    flags: Option<Vec<(String, Option<FlagValue>)>>
}

impl Context {
    pub fn new(args: Vec<String>, flags: Option<Vec<Flag>>) -> Self {
        let mut v = Vec::new();
        let mut parsed_args = args.clone();
        let flags_val = match flags {
            Some(flags) => {
                for flag in flags {
                    if parsed_args.contains(&format!("--{}", flag.name)) {
                        let index = parsed_args.iter().position(|arg| *arg == format!("--{}", flag.name)).unwrap();
                        parsed_args.remove(index);
                        if flag.flag_type != FlagType::Bool {
                            parsed_args.remove(index);
                        }
                    }
                    v.push((flag.name.to_string(), flag.value(args.clone())))
                }
                Some(v)
            }
            None => None
        };

        Self { args: parsed_args, flags: flags_val }
    }

    fn option_flag_value(&self, name: &str) -> Option<&FlagValue> {
        self.flags.as_ref()
            .and_then(|flags| flags.into_iter().find(|flag| flag.0 == name.to_string()))
            .and_then(|flag| flag.1.as_ref())
    }

    pub fn bool_flag(&self, name: &str) -> bool {
        match self.option_flag_value(name) {
            Some(FlagValue::Bool(val)) => {
                (*val).clone()
            },
            _ => false
        }
    }

    pub fn string_flag(&self, name: &str) -> Option<String> {
        match self.option_flag_value(name) {
            Some(FlagValue::String(val)) => Some((&val).to_string()),
            _ => None
        }
    }

    pub fn int_flag(&self, name: &str) -> Option<isize> {
        match self.option_flag_value(name) {
            Some(FlagValue::Int(val)) => Some(val.clone()),
            _ => None
        }
    }

    pub fn float_flag(&self, name: &str) -> Option<f64> {
        match self.option_flag_value(name) {
            Some(FlagValue::Float(val)) => Some(val.clone()),
            _ => None
        }
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
            Flag::new("bool", "", FlagType::Bool),
            Flag::new("string", "", FlagType::String),
            Flag::new("int", "", FlagType::Int),
            Flag::new("float", "", FlagType::Float)
        ];
        let context = Context::new(args, Some(flags));

        assert_eq!(true, context.bool_flag("bool"));

        match context.string_flag("string") {
            Some(val) => assert_eq!("test".to_string(), val),
            _ => assert!(false)
        }

        match context.int_flag("int") {
            Some(val) => assert_eq!(100, val),
            _ => assert!(false)
        }

        match context.float_flag("float") {
            Some(val) => assert_eq!(1.23, val),
            _ => assert!(false)
        }
    }
}
