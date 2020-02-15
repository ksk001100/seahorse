use std::env;
use seahorse::{App, Command, color, Flag, FlagType, Context};

fn main() {
    let args: Vec<String> = env::args().collect();
    let display_name = color::magenta("
     ██████╗██╗     ██╗
    ██╔════╝██║     ██║
    ██║     ██║     ██║
    ██║     ██║     ██║
    ╚██████╗███████╗██║
    ╚═════╝╚══════╝╚═╝");

    let app = App::new()
        .name("cli_tool")
        .display_name(display_name)
        .usage("cli_tool [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .commands(vec![command()]);

    app.run(args);
}

fn action(c: &Context) {
    let name = &c.args[2];
    if c.bool_flag("bool") {
        println!("true");
    } else {
        println!("false");
    }

    match c.string_flag("string") {
        Some(s) => println!("{}", s),
        None => println!("string none...")
    }

    println!("Hello, {}", name);
}

fn command() -> Command {
    Command::new()
        .name("hello")
        .usage("cli_tool hello [name]")
        .action(action)
        .flags(vec![
            Flag::new("bool", "cli_tool hello [name] --bool", FlagType::Bool),
            Flag::new("string", "cli_tool hello [name] --string [string]", FlagType::String),
        ])
}
