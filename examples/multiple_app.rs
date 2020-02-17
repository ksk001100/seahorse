use std::env;
use seahorse::{App, Command, color, Flag, FlagType, Context};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(color::yellow("multiple_app"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("multiple_app [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .commands(vec![hello_command()]);

    app.run(args);
}

fn hello_action(c: &Context) {
    let name = &c.args[2];
    if c.bool_flag("bye") {
        println!("Bye, {}", name);
    } else {
        println!("Hello, {}", name);
    }

    match c.int_flag("age") {
        Some(age) => println!("{} is {} years old", name, age),
        None => println!("I don't know {}'s age", name)
    }
}

fn hello_command() -> Command {
    Command::new()
        .name("hello")
        .usage("multiple_app hello [name]")
        .action(hello_action)
        .flags(vec![
            Flag::new("bye", "multiple_app hello [name] --bye", FlagType::Bool),
            Flag::new("age", "multiple_app hello [name] --age [age]", FlagType::Int)
        ])
}