use std::env;
use seahorse::{App, Command, color, Flag, FlagType, Context};

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = color::magenta("
     ██████╗██╗     ██╗
    ██╔════╝██║     ██║
    ██║     ██║     ██║
    ██║     ██║     ██║
    ╚██████╗███████╗██║
    ╚═════╝╚══════╝╚═╝");

    let app = App::new()
        .name(name)
        .usage("multiple_app [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .commands(vec![hello_command()]);

    app.run(args);
}

fn hello_action(c: &Context) {
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

fn hello_command() -> Command {
    Command::new()
        .name("hello")
        .usage("multiple_app hello [name]")
        .action(hello_action)
        .flags(vec![
            Flag::new("bool", "multiple_app hello [name] --bool", FlagType::Bool),
            Flag::new("string", "multiple_app hello [name] --string [string]", FlagType::String),
        ])
}
