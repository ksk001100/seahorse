use crate::{Action, Context, Flag};

/// Application command type
pub struct Command {
    /// Command name
    pub name: String,
    /// Command usage
    pub usage: String,
    /// Command action
    pub action: Action,
    /// Action flags
    pub flags: Option<Vec<Flag>>,
}

impl Default for Command {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            usage: "".to_string(),
            action: |c: &Context| println!("{:?}", c.args),
            flags: None,
        }
    }
}

impl Command {
    /// Create new instance of `Command`
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Command;
    ///
    /// let command = Command::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Set name of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Command;
    ///
    /// let command = Command::new()
    ///     .name("cmd");
    /// ```
    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    /// Set usage of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::Command;
    ///
    /// let command = Command::new()
    ///     .usage("cli cmd [arg]");
    /// ```
    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = usage.into();
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
    /// let command = Command::new()
    ///     .action(action);
    /// ```
    pub fn action(mut self, action: Action) -> Self {
        self.action = action;
        self
    }

    /// Set flag of the command
    ///
    /// Example
    ///
    /// ```
    /// use seahorse::{Command, Flag, FlagType};
    ///
    /// let command = Command::new()
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
    pub fn run(&self, v: Vec<String>) {
        (self.action)(&Context::new(v, self.flags.clone()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Action, Command, Context, Flag, FlagType};

    #[test]
    fn command_test() {
        let a: Action = |c: &Context| println!("Hello, {:?}", c.args);
        let c = Command::new()
            .name("hello")
            .usage("test hello user")
            .action(a)
            .flag(Flag::new("t", "t", FlagType::Bool));

        &c.flags.unwrap()[0].value(&vec!["--hoge".to_string()]);

        assert_eq!(c.name, "hello".to_string());
        assert_eq!(c.usage, "test hello user".to_string());
    }
}
