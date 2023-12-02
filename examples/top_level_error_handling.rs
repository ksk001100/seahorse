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

// Generation of an error is completely separate from how it is displayed.
// There's no need to be concerned about cluttering complex logic with the display style.
//
// Note that we don't store any extra info about the errors. This means we can't state
// which string failed to parse without modifying our types to carry that information.
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "test error")
    }
}

impl std::error::Error for Error {}
