use seahorse::{App, Context, Flag, FlagType};
use std::env;
use std::fmt;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("cli")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("multiple_app [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(|c: &Context| {
            if c.bool_flag("error") {
                Err(Box::new(MyCustomError))
            } else {
                Ok(())
            }
        })
        .flag(
            Flag::new("error", FlagType::Bool)
                .description("error flag")
                .alias("e"),
        );

    match app.run(args) {
        Ok(_) => println!("OK"),
        Err(e) => println!("{:?}", e),
    };
}

#[derive(Debug, Clone)]
struct MyCustomError;

impl fmt::Display for MyCustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "test error")
    }
}

impl std::error::Error for MyCustomError {}
