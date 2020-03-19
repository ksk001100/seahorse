use crate::{Action, Context, Flag};

/// Application command type
#[derive(Default)]
pub struct Command {
    /// Command name
    pub name: String,
    /// Command usage
    pub usage: Option<String>,
    /// Command action
    pub action: Option<Action>,
    /// Action flags
    pub flags: Option<Vec<Flag>>,
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

    /// Run command
    /// Call this function only from `App`
    pub fn run(&self, v: Vec<String>, help_text: String) {
        match self.action {
            Some(action) => action(&Context::new(v, self.flags.clone(), help_text)),
            None => println!("{}", help_text),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, Command, Context, Flag, FlagType};

    #[test]
    fn command_test() {
        let a: Action = |c: &Context| println!("Hello, {:?}", c.args);
        let c = Command::new("hello")
            .usage("test hello user")
            .action(a)
            .flag(Flag::new("t", "t", FlagType::Bool));

        &c.flags.unwrap()[0].value(&vec!["--hoge".to_string()]);

        assert_eq!(c.name, "hello".to_string());
        assert_eq!(c.usage, Some("test hello user".to_string()));
    }
}
