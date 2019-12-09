pub struct Command {
    pub name: String,
    pub action: Action,
}

pub type Action = fn(String);
