use crate::{Action, Context, Flag};

pub struct Command {
    pub name: String,
    pub usage: String,
    pub action: Action,
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
    pub fn new() -> Self {
        Self::default()
    }

    pub fn name<T: Into<String>>(mut self, name: T) -> Self {
        self.name = name.into();
        self
    }

    pub fn usage<T: Into<String>>(mut self, usage: T) -> Self {
        self.usage = usage.into();
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

    pub fn run(&self, v: Vec<String>) {
        (self.action)(&Context::new(v.clone(), self.flags.clone()))
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
            .flags(vec![Flag::new("t", "t", FlagType::Bool)]);

        &c.flags.unwrap()[0].value(vec!["--hoge".to_string()]);

        assert_eq!(c.name, "hello".to_string());
        assert_eq!(c.usage, "test hello user".to_string());
    }
}
