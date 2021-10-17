use seahorse::{App, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = "custom_help_app";

    // Customizing our help display with raw string literal
    // We could just as easily use include_str!() or other choices
    let custom_help = format!(
        r#"
Hi! I am custom help!

Author: {}
Description: {}
Version: {}

Usage: custom_help_app [args]
-b  to instead say bye to [args]
"#,
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_VERSION")
    );

    let app = App::with_custom_help(name, &custom_help)
        .action(action)
        .flag(
            Flag::new("bye", FlagType::Bool)
                .description("custom_help_app args --bye(-b)")
                .alias("b"),
        );

    app.run(args);
}

fn action(c: &Context) {
    // show the help if no args given
    if c.args.is_empty() {
        return c.help();
    }

    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }
}
