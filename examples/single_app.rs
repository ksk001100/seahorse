use seahorse::{color, App, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = color::magenta(
        "
     ██████╗██╗     ██╗
    ██╔════╝██║     ██║
    ██║     ██║     ██║
    ██║     ██║     ██║
    ╚██████╗███████╗██║
     ╚═════╝╚══════╝╚═╝",
    );

    let app = App::new()
        .name(name)
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("single_app [args]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(action)
        .flag(Flag::new("bye", "single_app args --bye(-b)", FlagType::Bool).alias("b"));

    app.run(args);
}

fn action(c: &Context) {
    let name = &c.args[0];
    if c.bool_flag("bye") {
        println!("Bye, {:?}", name);
    } else {
        println!("Hello, {:?}", name);
    }
}
