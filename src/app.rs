use crate::{Action, Command, Context, Flag};

/// Application type
enum AppType {
    Multiple,
    Single,
    Undefined,
}

/// Multiple action application entry point
pub struct App {
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
    /// Application commands
    pub commands: Option<Vec<Command>>,
    /// Application action
    pub action: Option<Action>,
    /// Application flags
    pub flags: Option<Vec<Flag>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: String::default(),
            author: String::default(),
            description: None,
            usage: String::default(),
            version: String::default(),
            commands: None,
            action: None,
            flags: None,
        }
    }
}

impl App {
    /// Create new instance of `App`
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::App;
    ///
    /// let app = App::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set name of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::App;
    ///
    /// let app = App::new();
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
    /// use seahorse::App;
    ///
    /// let app = App::new();
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
    /// use seahorse::App;
    ///
    /// let app = App::new();
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
    /// use seahorse::App;
    ///
    /// let app = App::new();
    /// app.usage("cli [command] [arg]");
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
    /// use seahorse::App;
    ///
    /// let app = App::new();
    /// app.version(env!("CARGO_PKG_VERSION"));
    /// ```
    pub fn version<T: Into<String>>(mut self, version: T) -> Self {
        self.version = version.into();
        self
    }

    /// Set commands of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{App, Command};
    ///
    /// let app = App::new();
    /// let command = Command::new()
    ///     .name("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// app.commands(vec![
    ///     command
    /// ]);
    /// ```
    pub fn commands(mut self, commands: Vec<Command>) -> Self {
        self.commands = Some(commands);
        self
    }

    /// Set action of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Action, App, Context};
    ///
    /// let app = App::new();
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// app.action(action);
    /// ```
    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    /// Set flags of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{App, Flag, FlagType};
    ///
    /// let app = App::new();
    /// app.flags(vec![
    ///     Flag::new("bool", "cli [arg] --bool", FlagType::Bool),
    ///     Flag::new("int", "cli [arg] --int [int]", FlagType::Int)
    /// ]);
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
    /// use seahorse::App;
    ///
    /// let args: Vec<String> = env::args().collect();
    /// let app = App::new();
    /// app.run(args);
    /// ```
    pub fn run(&self, args: Vec<String>) {
        match args.len() {
            1 => {
                self.help();
                return;
            }
            _ => (),
        }

        let (cmd_v, args_v) = match self.app_type() {
            AppType::Multiple => args[1..].split_at(1),
            AppType::Single => args.split_at(1),
            AppType::Undefined => {
                // TODO: I want to be able to check if there is a problem with the combination at compile time in the future (compile_error macro...)
                panic!("Action and flags cannot be set if commands are set in App");
            }
        };

        let cmd = match cmd_v.first() {
            Some(c) => c,
            None => {
                self.help();
                return;
            }
        };

        match (cmd.len(), args_v.len()) {
            (0, _) | (_, 0) => {
                self.help();
                return;
            }
            _ => (),
        }

        match self.select_command(&cmd) {
            Some(command) => {
                command.run(args[1..].to_vec());
            }
            None => match self.action {
                Some(action) => action(&Context::new(args[1..].to_vec(), self.flags.clone())),
                None => self.help(),
            },
        }
    }

    /// Get application type
    fn app_type(&self) -> AppType {
        match (&self.commands, (&self.action, &self.flags)) {
            (Some(_commands), (Some(_action), _flags)) => AppType::Undefined,
            (Some(_commands), (_action, Some(_flags))) => AppType::Undefined,
            (Some(_commands), (_, _)) => AppType::Multiple,
            (_, (_, _)) => AppType::Single,
        }
    }

    /// Application help
    /// Displays information about the application
    fn help(&self) {
        println!("name:\n\t{}\n", self.name);
        println!("Author:\n\t{}\n", self.author);

        if let Some(description) = self.description.to_owned() {
            println!("Description:\n\t{}\n", description);
        }

        println!("Usage:\n\t{}\n", self.usage);
        println!("Version:\n\t{}\n", self.version);

        match &self.commands {
            Some(commands) => {
                println!("Commands:");
                for c in commands {
                    println!("\t{} : {}", c.name, c.usage);

                    match &c.flags {
                        Some(flags) => {
                            for flag in flags {
                                println!("\t\t{}", flag.usage)
                            }
                        }
                        _ => (),
                    }
                }
            }
            None => (),
        }
    }

    /// Select command
    /// Gets the Command that matches the string passed in the argument
    fn select_command(&self, cmd: &String) -> Option<&Command> {
        match &self.commands {
            Some(commands) => commands.iter().find(|command| &command.name == cmd),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, App, Command, Context, Flag, FlagType};

    #[test]
    fn multiple_app_test() {
        let a: Action = |c: &Context| {
            assert_eq!(true, c.bool_flag("bool"));
            match c.string_flag("string") {
                Some(flag) => assert_eq!("string".to_string(), flag),
                None => assert!(false, "string test false..."),
            }
            match c.int_flag("int") {
                Some(flag) => assert_eq!(100, flag),
                None => assert!(false, "int test false..."),
            }
            match c.float_flag("float") {
                Some(flag) => assert_eq!(1.23, flag),
                None => assert!(false, "float test false..."),
            }
        };
        let c = Command::new()
            .name("hello")
            .usage("test hello args")
            .action(a)
            .flags(vec![
                Flag::new("bool", "test hello [args] --bool", FlagType::Bool),
                Flag::new(
                    "string",
                    "test hello [args] --int [int value]",
                    FlagType::String,
                ),
                Flag::new("int", "test hello [args] --int [int value]", FlagType::Int),
                Flag::new(
                    "float",
                    "test hello [args] --int [int value]",
                    FlagType::Float,
                ),
            ]);

        let app = App::new()
            .name("test")
            .author("Author <author@example.com>")
            .description("This is a great tool.")
            .usage("test [command] [arg]")
            .version("0.0.1")
            .commands(vec![c]);

        app.run(vec![
            "test".to_string(),
            "hello".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "string".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, "test [command] [arg]".to_string());
        assert_eq!(app.author, "Author <author@example.com>".to_string());
        assert_eq!(app.description, Some("This is a great tool.".to_string()));
        assert_eq!(app.version, "0.0.1".to_string());
    }

    #[test]
    fn single_app_test() {
        let action: Action = |c: &Context| {
            assert_eq!(true, c.bool_flag("bool"));
            match c.string_flag("string") {
                Some(flag) => assert_eq!("string".to_string(), flag),
                None => assert!(false, "string test false..."),
            }
            match c.int_flag("int") {
                Some(flag) => assert_eq!(100, flag),
                None => assert!(false, "int test false..."),
            }
            match c.float_flag("float") {
                Some(flag) => assert_eq!(1.23, flag),
                None => assert!(false, "float test false..."),
            }
        };

        let app = App::new()
            .name("test")
            .author("Author <author@example.com>")
            .description("This is a great tool.")
            .usage("test [command] [arg]")
            .version("0.0.1")
            .action(action)
            .flags(vec![
                Flag::new("bool", "test hello [args] --bool", FlagType::Bool),
                Flag::new(
                    "string",
                    "test hello [args] --int [int value]",
                    FlagType::String,
                ),
                Flag::new("int", "test hello [args] --int [int value]", FlagType::Int),
                Flag::new(
                    "float",
                    "test hello [args] --int [int value]",
                    FlagType::Float,
                ),
            ]);

        app.run(vec![
            "test".to_string(),
            "hello".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "string".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, "test [command] [arg]".to_string());
        assert_eq!(app.author, "Author <author@example.com>".to_string());
        assert_eq!(app.description, Some("This is a great tool.".to_string()));
        assert_eq!(app.version, "0.0.1".to_string());
    }
}
