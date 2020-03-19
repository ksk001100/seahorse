use seahorse::{color, App, Command, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(color::yellow("multiple_app"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("multiple_app [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(|c: &Context| println!("{:?} : {}", c.args, c.bool_flag("bool")))
        .flag(Flag::new("bool", "multiple_app [args] --bool(-b)", FlagType::Bool).alias("b"))
        .command(add_command())
        .command(hello_command());

    app.run(args);
}

fn hello_action(c: &Context) {
    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }

    match c.int_flag("age") {
        Some(age) => println!("{:?} is {} years old", c.args, age),
        None => println!("I don't know {:?}'s age", c.args),
    }
}

fn hello_command() -> Command {
    Command::new("hello")
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

fn add_action(c: &Context) {
    let sum: i32 = c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum();
    println!("{}", sum);
}

fn add_command() -> Command {
    Command::new("add")
        .usage("multiple_app add [num...]")
        .action(add_action)
}
