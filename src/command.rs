use crate::{Action, Context, Flag, FlagType, Help};

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
    /// Action flags
    pub flags: Option<Vec<Flag>>,
    /// Command alias
    pub alias: Option<Vec<String>>,
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
    pub fn action(mut self, action: Action) -> Self {
        self.action = Some(action);
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

    /// Run command
    /// Call this function only from `App`
    pub fn run(&self, args: Vec<String>) {
        if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
            self.help();
            return;
        }
        match self.action {
            Some(action) => action(&Context::new(args, self.flags.clone(), self.help_text())),
            None => self.help(),
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

            for flag_help in flag_helps.clone().into_iter() {
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
}
