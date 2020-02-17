use crate::{Action, Context, Flag};

/// Single action application entry point
pub struct SingleApp {
    /// Application name
    pub name: String,
    /// Application author
    pub author: String,
    /// Application description
    pub description: Option<String>,
    /// Application usage
    pub usage: String,
    /// Application version
    pub version: String,
    /// Application action
    pub action: Action,
    /// Action flags
    pub flags: Option<Vec<Flag>>
}

impl Default for SingleApp {
    fn default() -> Self {
        Self {
            name: String::default(),
            author: String::default(),
            description: None,
            usage: String::default(),
            version: String::default(),
            action: |c: &Context| println!("{:?}", c.args),
            flags: None,
        }
    }
}

impl SingleApp {
    /// Create new instance of `SingleApp`
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::SingleApp;
    ///
    /// let app = SingleApp::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set name of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::SingleApp;
    ///
    /// let app = SingleApp::new();
    /// app.name("cli");
    /// ```
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    /// Set author of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::SingleApp;
    ///
    /// let app = SingleApp::new();
    /// app.author(env!("CARGO_PKG_AUTHORS"));
    /// ```
    pub fn author<T: Into<String>>(mut self, author: T) -> Self {
        self.author = author.into();
        self
    }

    /// Set description of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::SingleApp;
    ///
    /// let app = SingleApp::new();
    /// app.description(env!("CARGO_PKG_DESCRIPTION"));
    /// ```
    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set usage of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::SingleApp;
    ///
    /// let app = SingleApp::new();
    /// app.usage("cli [arg]");
    /// ```
    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = usage.into();
        self
    }

    /// Set version of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::SingleApp;
    ///
    /// let app = SingleApp::new();
    /// app.version(env!("CARGO_PKG_VERSION"));
    /// ```
    pub fn version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = version.into();
        self
    }

    /// Set action of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{SingleApp, Action, Context};
    ///
    /// let app = SingleApp::new();
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// app.action(action);
    /// ```
    pub fn action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    /// Set flags of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{SingleApp, Flag, FlagType};
    ///
    /// let app = SingleApp::new();
    /// let bool_flag = Flag::new("bool", "cli [arg] --bool", FlagType::Bool);
    /// let int_flag = Flag::new("int", "cli [arg] --int [int number]", FlagType::Int);
    ///
    /// app.flags(vec![bool_flag, int_flag]);
    /// ```
    pub fn flags(mut self, flags: Vec<Flag>) -> Self {
        self.flags = Some(flags);
        self
    }

    /// Run app
    ///
    /// Example
    ///
    /// ```
    /// use std::env;
    /// use seahorse::SingleApp;
    ///
    /// let args: Vec<String> = env::args().collect();
    /// let app = SingleApp::new();
    /// app.run(args);
    /// ```
    pub fn run(&self, args: Vec<String>) {
        match args.len() {
            1 => self.help(),
            _ => (self.action)(&Context::new(args[1..].to_vec(), self.flags.clone()))
        }
    }

    /// Application help
    /// Displays information about the application
    fn help(&self) {
        println!("Name:\n\t{}\n", self.name);
        println!("Author:\n\t{}\n", self.author);

        if let Some(description) = self.description.to_owned() {
            println!("Description:\n\t{}\n", description);
        }

        println!("Usage:\n\t{}", self.usage);

        match &self.flags {
            Some(flags) => {
                for flag in flags {
                    println!("\t{}", flag.usage);
                }
                print!("\n");
            }
            _ => print!("\n")
        }

        println!("Version:\n\t{}\n", self.version);
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
        assert_eq!(app.description, None);
        assert_eq!(app.usage, "test [url]".to_string());
        assert_eq!(app.version, "0.0.1".to_string());
    }
}
