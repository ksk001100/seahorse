use seahorse::{App, Context, Flag, FlagType};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = "multiple_flags";

    let app = App::new(name)
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("multiple_flags [args]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(action)
        .flag(
            Flag::new("verbose", FlagType::Bool)
                .description("Increase verbosity level by repeat --verbose(-v) multiple times")
                .alias("v")
                .multiple(),
        )
        .flag(
            Flag::new("header", FlagType::String)
                .description("Set header of the request, argument can be repeated")
                .alias("H")
                .multiple(),
        )
        .flag(
            Flag::new("offset", FlagType::Uint)
                .description("Counter offset, argument can be repeated")
                .alias("o")
                .multiple(),
        );

    app.run(args);
}

fn action(c: &Context) {
    // Count the number of times the flag was passed
    let verbosity_level = c.bool_flag_vec("verbose").iter().flatten().count();

    println!("Verbosity level: {}", verbosity_level);

    // Print only the first 'header' flag passed
    println!("Headers: {:?}", c.string_flag("header"));

    // To access all 'header' flags passed, if the flag is not marked as multiple the
    // vector will only contain one element, the rest will be ignored.
    for header in c.string_flag_vec("header") {
        println!("Header: {:?}", header);
    }

    // Access all 'offset' flags passed
    for offset in c.uint_flag_vec("offset") {
        println!("offset: {:?}", offset);
    }
}
