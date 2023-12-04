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
        .action_with_result(|c: &Context| {
            if c.bool_flag("error") {
                Err(Box::new(Error))
            } else {
                Ok(())
            }
        })
        .flag(
            Flag::new("error", FlagType::Bool)
                .description("error flag")
                .alias("e"),
        );

    match app.run_with_result(args) {
        Ok(_) => println!("OK"),
        Err(e) => println!("{}", e),
    };
}

#[derive(Debug, Clone)]
struct Error;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "test error")
    }
}

impl std::error::Error for Error {}
