pub mod color;

pub type Action = fn(Vec<String>);

pub struct Command {
    pub name: String,
    pub action: Action,
}

pub struct App {
    pub name: String,
    pub display_name: String,
    pub usage: String,
    pub version: String,
    pub commands: Vec<Command>,
}

impl App {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            display_name: "".to_string(),
            usage: "".to_string(),
            version: "".to_string(),
            commands: Vec::<Command>::new(),
        }
    }

    pub fn run(&self, args: Vec<String>) {
        match args.len() {
            1 => {
                self.help();
                std::process::exit(1);
            }
            _ => (),
        }

        let (cmd_v, args_v) = args[1..].split_at(1);
        let cmd = match cmd_v.first() {
            Some(c) => c,
            None => {
                self.help();
                std::process::exit(1);
            }
        };

        match (cmd.len(), args_v.len()) {
            (0, _) | (_, 0) => {
                self.help();
                std::process::exit(1);
            }
            _ => (),
        }

        match self.select_command(&cmd) {
            Some(command) => (command.action)(args_v.to_vec()),
            None => self.help(),
        }
    }

    fn help(&self) {
        match self.display_name.len() {
            0 => println!("Name:\n   {}\n", self.name),
            _ => println!("Name:\n   {}\n", self.display_name),
        }

        println!("Usage:\n   {}\n", self.usage);
        println!("Version:\n   {}\n", self.version);

        println!("Commands:");
        for c in &self.commands {
            println!("   {} {} [arg]", self.name, c.name)
        }
    }

    fn select_command(&self, cmd: &String) -> Option<&Command> {
        (&self.commands)
            .into_iter()
            .find(|command| &command.name == cmd)
    }
}

#[cfg(test)]
mod tests {
    use super::{App, Command};

    #[test]
    fn app_test() {
        let c = Command {
            name: "hello".to_string(),
            action: |v: Vec<String>| println!("Hello, {:?}", v),
        };
        let mut app = App::new();
        app.name = "test".to_string();
        app.usage = "test [command] [arg]".to_string();
        app.version = "0.0.1".to_string();
        app.commands = vec![c];

        app.run(vec![
            "test".to_string(),
            "hello".to_string(),
            "arg1".to_string(),
            "arg2".to_string(),
            "arg3".to_string(),
            "arg4".to_string(),
        ]);

        assert_eq!(app.name, "test".to_string());
        assert_eq!(app.usage, "test [command] [arg]".to_string());
        assert_eq!(app.version, "0.0.1".to_string());
    }
}
