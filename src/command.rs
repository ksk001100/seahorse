use crate::Action;

pub struct Command {
    pub name: String,
    pub usage: String,
    pub action: Action,
}

impl Command {
    pub fn new<T: Into<String>>(name: T, usage: T, action: Action) -> Self {
        Self {
            name: name.into(),
            usage: usage.into(),
            action,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Command, Action};

    #[test]
    fn command_test() {
        let a: Action = |v: Vec<String>| println!("Hello, {:?}", v);
        let c = Command::new("hello", "test hello user", a);

        assert_eq!(c.name, "hello".to_string());
        assert_eq!(c.usage, "test hello user".to_string());
    }
}