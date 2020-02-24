use crate::{Action, Command, Context, Flag};
use std::io::{stdout, BufWriter, Write};

/// Application type
enum AppType {
    Multiple,
    Single,
    Empty,
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
        if args.contains(&"--help".to_string()) {
            self.help();
            return;
        }

        let args = Self::normalized_args(args);
        match self.app_type() {
            AppType::Multiple => {
                let (cmd_v, args_v) = match args.len() {
                    1 => {
                        self.help();
                        return;
                    }
                    _ => args[1..].split_at(1),
                };

                let cmd = match cmd_v.first() {
                    Some(c) => c,
                    None => {
                        self.help();
                        return;
                    }
                };

                if cmd.len() < 1 {
                    self.help();
                    return;
                }

                match self.select_command(&cmd) {
                    Some(command) => {
                        command.run(args_v.to_vec());
                    }
                    None => self.help(),
                }
            }
            AppType::Single => {
                let args_v = &args[1..];
                match self.action {
                    Some(action) => action(&Context::new(args_v.to_vec(), self.flags.clone())),
                    None => self.help(),
                }
            }
            AppType::Empty => {
                self.help();
                return;
            }
            AppType::Undefined => {
                // TODO: I want to be able to check if there is a problem with the combination at compile time in the future (compile_error macro...)
                panic!("Action and flags cannot be set if commands are set in App");
            }
        }
    }

    /// Get application type
    fn app_type(&self) -> AppType {
        match (
            &self.commands.is_some(),
            &self.action.is_some(),
            &self.flags.is_some(),
        ) {
            (true, false, false) => AppType::Multiple,
            (false, true, _) => AppType::Single,
            (false, false, false) => AppType::Empty,
            _ => AppType::Undefined,
        }
    }

    /// Application help
    /// Displays information about the application
    fn help(&self) {
        let out = stdout();
        let mut out = BufWriter::new(out.lock());

        writeln!(out, "Name:\n\t{}\n", self.name).unwrap();
        writeln!(out, "Author:\n\t{}\n", self.author).unwrap();

        if let Some(description) = self.description.to_owned() {
            writeln!(out, "Description:\n\t{}\n", description).unwrap();
        }

        writeln!(out, "Usage:\n\t{}", self.usage).unwrap();

        match &self.commands {
            Some(commands) => {
                writeln!(out, "\nCommands:").unwrap();
                for c in commands {
                    writeln!(out, "\t{} : {}", c.name, c.usage).unwrap();

                    match &c.flags {
                        Some(flags) => {
                            for flag in flags {
                                writeln!(out, "\t\t{}", flag.usage).unwrap();
                            }
                        }
                        None => (),
                    }
                }
            }
            None => match &self.flags {
                Some(flags) => {
                    for flag in flags {
                        writeln!(out, "\t{}", flag.usage).unwrap();
                    }
                    write!(out, "\n").unwrap();
                }
                None => (),
            },
        }

        writeln!(out, "Version:\n\t{}\n", self.version).unwrap();
    }

    /// Select command
    /// Gets the Command that matches the string passed in the argument
    fn select_command(&self, cmd: &String) -> Option<&Command> {
        match &self.commands {
            Some(commands) => commands.iter().find(|command| &command.name == cmd),
            None => None,
        }
    }

    /// Split arg with "=" to unify arg notations.
    /// --flag=value => ["--flag", "value"]
    /// --flag value => ["--flag", "value"]
    fn normalized_args(raw_args: Vec<String>) -> Vec<String> {
        raw_args.iter().fold(Vec::<String>::new(), |mut acc, cur| {
            if cur.starts_with('-') && cur.contains('=') {
                let mut splitted_flag: Vec<String> =
                    cur.splitn(2, '=').map(|s| s.to_owned()).collect();
                acc.append(&mut splitted_flag);
            } else {
                acc.push(cur.to_owned());
            }
            acc
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, App, Command, Context, Flag, FlagType};

    #[test]
    fn app_new_only_test() {
        let app = App::new();
        app.run(vec!["cli".to_string()]);

        assert_eq!(app.name, "".to_string());
        assert_eq!(app.usage, "".to_string());
        assert_eq!(app.author, "".to_string());
        assert_eq!(app.description, None);
        assert_eq!(app.version, "".to_string());
    }

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
                    "test hello [args] --string [string value]",
                    FlagType::String,
                ),
                Flag::new("int", "test hello [args] --int [int value]", FlagType::Int),
                Flag::new(
                    "float",
                    "test hello [args] --float [float value]",
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
            .usage("test [arg]")
            .version("0.0.1")
            .action(action)
            .flags(vec![
                Flag::new("bool", "test [args] --bool", FlagType::Bool),
                Flag::new(
                    "string",
                    "test [args] --string [string value]",
                    FlagType::String,
                ),
                Flag::new("int", "test [args] --int [int value]", FlagType::Int),
                Flag::new(
                    "float",
                    "test [args] --float [float value]",
                    FlagType::Float,
                ),
            ]);

        app.run(vec![
            "test".to_string(),
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
        assert_eq!(app.usage, "test [arg]".to_string());
        assert_eq!(app.author, "Author <author@example.com>".to_string());
        assert_eq!(app.description, Some("This is a great tool.".to_string()));
        assert_eq!(app.version, "0.0.1".to_string());
    }

    #[test]
    fn flag_only_app_test() {
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
            .usage("test")
            .version("0.0.1")
            .action(action)
            .flags(vec![
                Flag::new("bool", "test --bool", FlagType::Bool),
                Flag::new("string", "test --string [string value]", FlagType::String),
                Flag::new("int", "test --int [int value]", FlagType::Int),
                Flag::new("float", "test --float [float value]", FlagType::Float),
            ]);

        app.run(vec![
            "test".to_string(),
            "--bool".to_string(),
            "--string".to_string(),
            "string".to_string(),
            "--int".to_string(),
            "100".to_string(),
            "--float".to_string(),
            "1.23".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, "test".to_string());
        assert_eq!(app.author, "Author <author@example.com>".to_string());
        assert_eq!(app.description, Some("This is a great tool.".to_string()));
        assert_eq!(app.version, "0.0.1".to_string());
    }

    #[test]
    fn single_app_equal_notation_test() {
        let action: Action = |c: &Context| {
            assert_eq!(true, c.bool_flag("bool"));
            match c.string_flag("string") {
                Some(flag) => assert_eq!("str=ing".to_string(), flag),
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
            .usage("test [arg]")
            .version("0.0.1")
            .action(action)
            .flags(vec![
                Flag::new("bool", "test [args] --bool", FlagType::Bool),
                Flag::new(
                    "string",
                    "test [args] --string [string value]",
                    FlagType::String,
                ),
                Flag::new("int", "test [args] --int [int value]", FlagType::Int),
                Flag::new(
                    "float",
                    "test [args] --float [float value]",
                    FlagType::Float,
                )
                .alias("f"),
            ]);

        app.run(vec![
            "test".to_string(),
            "args".to_string(),
            "--bool".to_string(),
            "--string=str=ing".to_string(),
            "--int=100".to_string(),
            "-f=1.23".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, "test [arg]".to_string());
        assert_eq!(app.author, "Author <author@example.com>".to_string());
        assert_eq!(app.description, Some("This is a great tool.".to_string()));
        assert_eq!(app.version, "0.0.1".to_string());
    }
}
