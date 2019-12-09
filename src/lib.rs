pub mod color;
pub mod command;

pub struct App {
    pub name: String,
    pub display_name: String,
    pub usage: String,
    pub version: String,
    pub commands: Vec<command::Command>,
}

impl App {
    pub fn new() -> Self {
        Self {
            name: "".to_string(),
            display_name: "".to_string(),
            usage: "".to_string(),
            version: "".to_string(),
            commands: Vec::<command::Command>::new(),
        }
    }

    pub fn run(&self, args: Vec<String>) {
        let (cmd, arg) = match args.len() {
            3 => ((&args[1]).clone(), (&args[2]).clone()),
            _ => (String::new(), String::new()),
        };

        match (cmd.len(), arg.len()) {
            (0, _) | (_, 0) => {
                self.help();
                std::process::exit(1);
            }
            _ => (),
        }

        match self.select_command(&cmd) {
            Some(command) => (command.action)(arg),
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

    fn select_command(&self, cmd: &String) -> Option<&command::Command> {
        (&self.commands)
            .into_iter()
            .find(|command| &command.name == cmd)
    }
}
