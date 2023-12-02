use crate::{Action, ActionWithResult, Context, Flag, FlagType, Help, Result};

/// Application command type
#[derive(Default)]
pub struct Command {
    /// Command name
    pub name: String,
    /// Command description
    pub description: Option<String>,
    /// Command usage
    pub usage: Option<String>,
    /// Command action
    pub action: Option<Action>,
    /// Alternate command action that returns a Result
    pub action_with_result: Option<ActionWithResult>,
    /// Action flags
    pub flags: Option<Vec<Flag>>,
    /// Command alias
    pub alias: Option<Vec<String>>,
    pub commands: Option<Vec<Command>>,
}

impl Command {
    /// Create new instance of `Command`
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Command;
    ///
    /// let command = Command::new("cmd");
    /// ```
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Set description of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Command;
    ///
    /// let command = Command::new("cmd")
    ///     .description("cli sub command");
    /// ```
    pub fn description<T: Into<String>>(mut self, description: T) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set usage of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Command;
    ///
    /// let command = Command::new("cmd")
    ///     .usage("cli cmd [arg]");
    /// ```
    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = Some(usage.into());
        self
    }

    /// Set action of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Command, Context, Action};
    ///
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let command = Command::new("cmd")
    ///     .action(action);
    /// ```
    ///
    /// # Panics
    ///
    /// You cannot set both action and action_with_result.
    ///
    /// ```should_panic
    /// use seahorse::{Action, ActionWithResult, Command, Context};
    ///
    /// let action_with_result: ActionWithResult = |c: &Context| {println!("{:?}", c.args); Ok(())};
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let command = Command::new("cmd")
    ///     .action_with_result(action_with_result)
    ///     .action(action);
    /// ```
    pub fn action(mut self, action: Action) -> Self {
        if self.action_with_result.is_some() {
            panic!(r#"only one of action and action_with_result can be set."#);
        }
        self.action = Some(action);
        self
    }

    /// Set action of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{ActionWithResult, Command, Context};
    ///
    /// let action_with_result: ActionWithResult = |c: &Context| {println!("{:?}", c.args); Ok(())};
    /// let command = Command::new("cmd")
    ///     .action_with_result(action_with_result);
    /// ```
    ///
    /// # Panics
    ///
    /// You cannot set both action and action_with_result.
    ///
    /// ```should_panic
    /// use seahorse::{Action, ActionWithResult, Command, Context};
    ///
    /// let action_with_result: ActionWithResult = |c: &Context| {println!("{:?}", c.args); Ok(())};
    /// let action: Action = |c: &Context| println!("{:?}", c.args);
    /// let command = Command::new("cmd")
    ///     .action(action)
    ///     .action_with_result(action_with_result);
    /// ```
    pub fn action_with_result(mut self, action_with_result: ActionWithResult) -> Self {
        if self.action.is_some() {
            panic!(r#"only one of action and action_with_result can be set."#);
        }
        self.action_with_result = Some(action_with_result);
        self
    }

    /// Set flag of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Command, Flag, FlagType};
    ///
    /// let command = Command::new("cmd")
    ///     .flag(Flag::new("bool", FlagType::Bool))
    ///     .flag(Flag::new("int", FlagType::Int));
    /// ```
    pub fn flag(mut self, flag: Flag) -> Self {
        if let Some(ref mut flags) = self.flags {
            (*flags).push(flag);
        } else {
            self.flags = Some(vec![flag]);
        }
        self
    }

    /// Set alias of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Command;
    ///
    /// let command = Command::new("cmd")
    ///     .alias("c");
    /// ```
    pub fn alias<T: Into<String>>(mut self, name: T) -> Self {
        if let Some(ref mut alias) = self.alias {
            (*alias).push(name.into());
        } else {
            self.alias = Some(vec![name.into()]);
        }
        self
    }

    /// Set sub command of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{App, Command};
    ///
    /// let sub_command = Command::new("world")
    ///     .usage("cli hello world")
    ///     .action(|_| println!("Hello world!"));
    ///
    /// let command = Command::new("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args))
    ///     .command(sub_command);
    ///
    /// let app = App::new("cli")
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
    /// let sub_command1 = Command::new("world")
    ///     .usage("cli hello world")
    ///     .action(|_| println!("Hello world!"));
    ///
    /// let sub_command2 = Command::new("world")
    ///     .usage("cli hello world")
    ///     .action(|_| println!("Hello world!"));
    ///
    /// let command = Command::new("hello")
    ///     .usage("cli hello [arg]")
    ///     .action(|c| println!("{:?}", c.args))
    ///     .command(sub_command1)
    ///     .command(sub_command2);
    ///
    /// let app = App::new("cli")
    ///     .command(command);
    /// ```
    pub fn command(mut self, command: Command) -> Self {
        if let Some(ref mut commands) = self.commands {
            if commands
                .iter()
                .any(|registered| registered.name == command.name)
            {
                panic!(r#"Command name "{}" is already registered."#, command.name);
            }
            (*commands).push(command);
        } else {
            self.commands = Some(vec![command]);
        }
        self
    }

    fn select_command(&self, cmd: &str) -> Option<&Command> {
        match &self.commands {
            Some(commands) => commands.iter().find(|command| match &command.alias {
                Some(alias) => command.name == cmd || alias.iter().any(|a| a == cmd),
                None => command.name == cmd,
            }),
            None => None,
        }
    }

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

    /// Run command
    /// Call this function only from `App`
    pub fn run_with_result(&self, args: Vec<String>) -> Result {
        let args = Self::normalized_args(args);

        match args.split_first() {
            Some((cmd, args_v)) => match self.select_command(cmd) {
                Some(command) => {
                    return command.run_with_result(args_v.to_vec());
                }
                None => match self.action {
                    Some(action) => {
                        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string())
                        {
                            self.help();
                            return Ok(());
                        }
                        action(&Context::new(
                            args.to_vec(),
                            self.flags.clone(),
                            self.help_text(),
                        ));
                        return Ok(());
                    }
                    None => match self.action_with_result {
                        Some(action_with_result) => {
                            if args.contains(&"-h".to_string())
                                || args.contains(&"--help".to_string())
                            {
                                self.help();
                                return Ok(());
                            }
                            return action_with_result(&Context::new(
                                args.to_vec(),
                                self.flags.clone(),
                                self.help_text(),
                            ));
                        }
                        None => {
                            self.help();
                            return Ok(());
                        }
                    },
                },
            },
            None => match self.action {
                Some(action) => {
                    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
                        self.help();
                        return Ok(());
                    }
                    action(&Context::new(
                        args.to_vec(),
                        self.flags.clone(),
                        self.help_text(),
                    ));
                    return Ok(());
                }
                None => match self.action_with_result {
                    Some(action_with_result) => {
                        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string())
                        {
                            self.help();
                            return Ok(());
                        }
                        return action_with_result(&Context::new(
                            args.to_vec(),
                            self.flags.clone(),
                            self.help_text(),
                        ));
                    }
                    None => {
                        self.help();
                        return Ok(());
                    }
                },
            },
        }
    }

    fn flag_help_text(&self) -> String {
        let mut text = String::new();
        text += "Flags:\n";
        let help_flag = "-h, --help";

        if let Some(flags) = &self.flags {
            let int_val = "<int>";
            let float_val = "<float>";
            let string_val = "<string>";

            let flag_helps = &flags.iter().map(|f| {
                let alias = match &f.alias {
                    Some(alias) => alias
                        .iter()
                        .map(|a| format!("-{}", a))
                        .collect::<Vec<String>>()
                        .join(", "),
                    None => String::new(),
                };
                let val = match f.flag_type {
                    FlagType::Int => int_val,
                    FlagType::Float => float_val,
                    FlagType::String => string_val,
                    _ => "",
                };

                let help = if alias.is_empty() {
                    format!("--{} {}", f.name, val)
                } else {
                    format!("{}, --{} {}", alias, f.name, val)
                };

                (help, f.description.clone())
            });

            let flag_name_max_len = flag_helps
                .clone()
                .map(|h| h.0.len())
                .chain(vec![help_flag.len()].into_iter())
                .max()
                .unwrap();

            for flag_help in flag_helps.clone() {
                text += &format!("\t{}", flag_help.0);

                if let Some(usage) = &flag_help.1 {
                    let flag_name_len = flag_help.0.len();
                    text += &format!(
                        "{} : {}\n",
                        " ".repeat(flag_name_max_len - flag_name_len),
                        usage
                    );
                } else {
                    text += "\n";
                }
            }

            text += &format!(
                "\t{}{} : Show help\n",
                help_flag,
                " ".repeat(flag_name_max_len - help_flag.len())
            );
        } else {
            text += &format!("\t{} : Show help\n", help_flag);
        }

        text
    }

    fn command_help_text(&self) -> String {
        let mut text = String::new();

        if let Some(commands) = &self.commands {
            text += "\nCommands:\n";

            let name_max_len = &commands
                .iter()
                .map(|c| {
                    if let Some(alias) = &c.alias {
                        format!("{}, {}", alias.join(", "), c.name).len()
                    } else {
                        c.name.len()
                    }
                })
                .max()
                .unwrap();

            for c in commands {
                let command_name = if let Some(alias) = &c.alias {
                    format!("{}, {}", alias.join(", "), c.name)
                } else {
                    c.name.clone()
                };

                let description = match &c.description {
                    Some(description) => description,
                    None => "",
                };

                text += &format!(
                    "\t{} {}: {}\n",
                    command_name,
                    " ".repeat(name_max_len - command_name.len()),
                    description
                );
            }
        }

        text
    }
}

impl Help for Command {
    fn help_text(&self) -> String {
        let mut text = String::new();

        if let Some(description) = &self.description {
            text += &format!("Description:\n\t{}\n\n", description);
        }

        if let Some(usage) = &self.usage {
            text += &format!("Usage:\n\t{}\n\n", usage);
        }

        text += &self.flag_help_text();
        text += &self.command_help_text();

        text
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, Command, Context, Flag, FlagType};

    #[test]
    fn command_test() {
        let a: Action = |c: &Context| println!("Hello, {:?}", c.args);
        let c = Command::new("hello")
            .description("usre command")
            .usage("test hello user")
            .alias("c")
            .action(a)
            .flag(Flag::new("t", FlagType::Bool));

        assert_eq!(c.name, "hello".to_string());
        assert_eq!(c.usage, Some("test hello user".to_string()));
    }

    #[test]
    fn sub_command_test() {
        let a: Action = |c: &Context| println!("Hello, {:?}", c.args);
        let sub = Command::new("world")
            .description("user command")
            .usage("test hello world user")
            .alias("w")
            .action(a)
            .flag(Flag::new("t", FlagType::Bool));
        let c = Command::new("hello")
            .description("user command")
            .usage("test hello user")
            .alias("h")
            .action(a)
            .flag(Flag::new("t", FlagType::Bool))
            .command(sub);

        assert_eq!(c.name, "hello".to_string());
        assert_eq!(c.usage, Some("test hello user".to_string()));
    }
}
