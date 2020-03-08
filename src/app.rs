use crate::{Action, Command, Context, Flag};
use std::io::{stdout, BufWriter, Write};

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
    /// let app = App::new()
    ///     .name("cli");
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
    /// let app = App::new()
    ///     .author(env!("CARGO_PKG_AUTHORS"));
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
    /// let app = App::new()
    ///     .description(env!("CARGO_PKG_DESCRIPTION"));
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

    /// Set command of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{App, Command};
    ///
    /// let command = Command::new()
    ///     .name("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// let app = App::new()
    ///     .command(command);
    /// ```
    ///
    /// # Panics
    ///
    /// You cannot set a command named as same as registered ones.
    ///
    /// ```should_panic
    /// use seahorse::{App, Command};
    ///
    /// let command1 = Command::new()
    ///     .name("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// let command2 = Command::new()
    ///     .name("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args));
    ///
    /// let app = App::new()
    ///     .command(command1)
    ///     .command(command2);
    /// ```
    pub fn command(mut self, command: Command) -> Self {
        if let Some(ref mut commands) = self.commands {
            if commands
                .iter()
                .any(|registered| registered.name == command.name)
            {
                panic!(format!(
                    r#"Command name "{}" is already registered."#,
                    command.name
                ));
            }
            (*commands).push(command);
        } else {
            self.commands = Some(vec![command]);
        }
        self
    }

    /// Set action of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Action, App, Context};
    ///
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let app = App::new()
    ///     .action(action);
    /// ```
    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
        self
    }

    /// Set flag of the app
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{App, Flag, FlagType};
    ///
    /// let app = App::new()
    ///     .flag(Flag::new("bool", "cli [arg] --bool", FlagType::Bool))
    ///     .flag(Flag::new("int", "cli [arg] --int [int]", FlagType::Int));
    /// ```
    pub fn flag(mut self, flag: Flag) -> Self {
        if let Some(ref mut flags) = self.flags {
            (*flags).push(flag);
        } else {
            self.flags = Some(vec![flag]);
        }
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
        let (cmd_v, args_v) = match args.len() {
            1 => args.split_at(1),
            _ => args[1..].split_at(1),
        };

        let cmd = match cmd_v.first() {
            Some(c) => c,
            None => {
                self.help();
                return;
            }
        };

        match self.select_command(&cmd) {
            Some(command) => command.run(args_v.to_vec()),
            None => match self.action {
                Some(action) => action(&Context::new(args[1..].to_vec(), self.flags.clone())),
                None => self.help(),
            },
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

        if let Some(flags) = &self.flags {
            for flag in flags {
                writeln!(out, "\t{}", flag.usage).unwrap();
            }
            writeln!(out).unwrap();
        }

        if let Some(commands) = &self.commands {
            writeln!(out, "\nCommands:").unwrap();

            let name_max_len = &commands.iter().map(|c| c.name.len()).max().unwrap();
            let whitespace = " ".repeat(name_max_len + 3);

            for c in commands {
                writeln!(
                    out,
                    "\t{} {}: {}",
                    c.name,
                    " ".repeat(name_max_len - c.name.len()),
                    c.usage
                )
                .unwrap();

                if let Some(flags) = &c.flags {
                    for flag in flags {
                        writeln!(out, "\t{}{}", whitespace, flag.usage).unwrap();
                    }
                }

                writeln!(out).unwrap();
            }
        }

        writeln!(out, "Version:\n\t{}\n", self.version).unwrap();
    }

    /// Select command
    /// Gets the Command that matches the string passed in the argument
    fn select_command(&self, cmd: &str) -> Option<&Command> {
        match &self.commands {
            Some(commands) => commands.iter().find(|command| command.name == cmd),
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
            .flag(Flag::new(
                "bool",
                "test hello [args] --bool",
                FlagType::Bool,
            ))
            .flag(Flag::new(
                "string",
                "test hello [args] --string [string value]",
                FlagType::String,
            ))
            .flag(Flag::new(
                "int",
                "test hello [args] --int [int value]",
                FlagType::Int,
            ))
            .flag(Flag::new(
                "float",
                "test hello [args] --float [float value]",
                FlagType::Float,
            ));

        let app = App::new()
            .name("test")
            .author("Author <author@example.com>")
            .description("This is a great tool.")
            .usage("test [command] [arg]")
            .version("0.0.1")
            .command(c);

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
            .flag(Flag::new(
                "bool",
                "test hello [args] --bool",
                FlagType::Bool,
            ))
            .flag(Flag::new(
                "string",
                "test hello [args] --string [string value]",
                FlagType::String,
            ))
            .flag(Flag::new(
                "int",
                "test hello [args] --int [int value]",
                FlagType::Int,
            ))
            .flag(Flag::new(
                "float",
                "test hello [args] --float [float value]",
                FlagType::Float,
            ));

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
            .flag(Flag::new(
                "bool",
                "test hello [args] --bool",
                FlagType::Bool,
            ))
            .flag(Flag::new(
                "string",
                "test hello [args] --string [string value]",
                FlagType::String,
            ))
            .flag(Flag::new(
                "int",
                "test hello [args] --int [int value]",
                FlagType::Int,
            ))
            .flag(Flag::new(
                "float",
                "test hello [args] --float [float value]",
                FlagType::Float,
            ));

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
            .flag(Flag::new(
                "bool",
                "test hello [args] --bool",
                FlagType::Bool,
            ))
            .flag(Flag::new(
                "string",
                "test hello [args] --string [string value]",
                FlagType::String,
            ))
            .flag(Flag::new(
                "int",
                "test hello [args] --int [int value]",
                FlagType::Int,
            ))
            .flag(
                Flag::new(
                    "float",
                    "test hello [args] --float [float value]",
                    FlagType::Float,
                )
                .alias("f"),
            );

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
