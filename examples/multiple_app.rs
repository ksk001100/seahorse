use seahorse::{color, App, Command, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name(color::yellow("multiple_app"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("multiple_app [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .command(hello_command());

    app.run(args);
}

fn hello_action(c: &Context) {
    let name = &c.args[0];
    if c.bool_flag("bye") {
        println!("Bye, {}", name);
    } else {
        println!("Hello, {}", name);
    }

    match c.int_flag("age") {
        Some(age) => println!("{} is {} years old", name, age),
        None => println!("I don't know {}'s age", name),
    }
}

fn hello_command() -> Command {
    Command::new()
        .name("hello")
        .usage("multiple_app hello [name]")
        .action(hello_action)
        .flag(Flag::new("bye", "multiple_app hello [name] --bye(-b)", FlagType::Bool).alias("b"))
        .flag(
            Flag::new(
                "age",
                "multiple_app hello [name] --age(-a, -ag) [age]",
                FlagType::Int,
            )
            .alias("a")
            .alias("ag"),
        )
}
