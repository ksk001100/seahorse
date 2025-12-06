use seahorse::{error::FlagError, App, Context, Flag, FlagType};
use std::env;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("multiple_app")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("multiple_app [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(|c: &Context| {
            println!("{:?} : {}", c.args, c.bool_flag("bool"));
            Ok(())
        })
        .flag(
            Flag::new("bool", FlagType::Bool)
                .description("bool flag")
                .alias("b"),
        )
        .command(add_command())
        .command(hello_command());

    if let Err(e) = app.run(args) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn hello_action(c: &Context) -> Result<(), Box<dyn Error>> {
    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }

    match c.int_flag("age") {
        Ok(age) => println!("{:?} is {} years old", c.args, age),
        Err(e) => match e {
            FlagError::TypeError => println!("age flag type error"),
            FlagError::ValueTypeError => println!("value type error"),
            FlagError::Undefined => println!("undefined age flag"),
            FlagError::ArgumentError => println!("age flag argument error"),
            FlagError::NotFound => println!("not found age flag"),
        },
    }

    match c.string_flag("neko") {
        Ok(neko) => println!("neko say {}", neko),
        Err(e) => match e {
            FlagError::TypeError => println!("neko flag type error"),
            FlagError::ValueTypeError => println!("value type error"),
            FlagError::Undefined => println!("undefined neko flag"),
            FlagError::ArgumentError => println!("neko flag argument error"),
            FlagError::NotFound => println!("not found neko flag"),
        },
    }
    Ok(())
}

fn hello_command() -> App {
    App::new("hello")
        .description("hello command")
        .usage("multiple_app hello(he, h) [name]")
        .alias("h")
        .alias("he")
        .action(hello_action)
        .flag(
            Flag::new("bye", FlagType::Bool)
                .description("bye flag")
                .alias("b"),
        )
        .flag(
            Flag::new("age", FlagType::Int)
                .description("age flag")
                .alias("a")
                .alias("ag"),
        )
        .command(world_command())
}

fn add_action(c: &Context) -> Result<(), Box<dyn Error>> {
    let sum: i32 = c
        .args
        .iter()
        .map(|n| n.parse::<i32>().map_err(|e| Box::new(e) as Box<dyn Error>))
        .collect::<Result<Vec<i32>, Box<dyn Error>>>()?
        .into_iter()
        .sum();
    println!("{}", sum);
    Ok(())
}

fn add_command() -> App {
    App::new("add")
        .description("add command")
        .usage("multiple_app add [num...]")
        .action(add_action)
}

fn world_command() -> App {
    App::new("world")
        .description("hello world command")
        .usage("nested_multiple_app hello(he, h) world(w)")
        .alias("w")
        .action(|_| {
            println!("Hello world");
            Ok(())
        })
}
