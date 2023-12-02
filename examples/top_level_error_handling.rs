use seahorse::{App, Context, Error, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new("cli")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("multiple_app [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .action_with_result(|c: &Context| {
            if c.bool_flag("error") {
                Err(Error {
                    message: "ERROR...".to_string(),
                })
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
        Err(e) => match e {
            Error { message } => println!("{}", message),
        },
    };
}
