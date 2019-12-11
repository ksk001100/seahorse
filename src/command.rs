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