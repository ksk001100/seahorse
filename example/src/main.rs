use seahorse::{color, App, Command};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let c = Command::new("hello", "app hello [args]", test_action);
    let app = App::new()
        .name("app")
        .display_name(color::red("app"))
        .usage("app [command] [args]")
        .version("0.0.1")
        .commands(vec![c]);

    app.run(args);
}

fn test_action(v: Vec<String>) {
    println!("Hello, {:?}", v);
}
