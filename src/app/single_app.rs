use crate::{Action, Context, Flag};

pub struct SingleApp {
    pub name: String,
    pub display_name: String,
    pub usage: String,
    pub version: String,
    pub action: Action,
    pub flags: Option<Vec<Flag>>
}

impl Default for SingleApp {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            display_name: "".to_string(),
            usage: "".to_string(),
            version: "".to_string(),
            action: |c: &Context| println!("{:?}", c.args),
            flags: None,
        }
    }
}

impl SingleApp {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    pub fn display_name<T: Into<String>>(mut self, display_name: T) -> Self {
        self.display_name = display_name.into();
        self
    }

    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = usage.into();
        self
    }

    pub fn version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = version.into();
        self
    }

    pub fn action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    pub fn flags(mut self, flags: Vec<Flag>) -> Self {
        self.flags = Some(flags);
        self
    }

    pub fn run(&self, args: Vec<String>) {
        match args.len() {
            1 => self.help(),
            _ => (self.action)(&Context::new(args[1..].to_vec(), self.flags.clone()))
        }
    }

    fn help(&self) {
        match self.display_name.len() {
            0 => println!("Name:\n   {}\n", self.name),
            _ => println!("Name:\n   {}\n", self.display_name),
        }

        println!("Usage:\n   {}\n", self.usage);
        println!("Version:\n   {}\n", self.version);
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, Context, SingleApp, Flag, FlagType};

    #[test]
    fn single_app_test() {
        let a: Action = |c: &Context| {
            if c.bool_flag("bool") {
                assert!(true, "bool test true");
            } else {
                assert!(false, "bool test false");
            }
        };
        let app = SingleApp::new()
            .name("test")
            .usage("test [url]")
            .version("0.0.1")
            .action(a)
            .flags(vec![
                Flag::new("bool", "test [url] --bool", FlagType::Bool)
            ]);

        app.run(vec![
            "test".to_string(),
            "http://google.com".to_string(),
            "--bool".to_string()
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, "test [url]".to_string());
        assert_eq!(app.version, "0.0.1".to_string());
    }
}
