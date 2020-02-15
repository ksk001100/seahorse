# seahorse

[![crates.io](https://img.shields.io/crates/v/seahorse.svg)](https://crates.io/crates/seahorse)
![](https://img.shields.io/github/release/ksk001100/seahorse.svg)
![](https://img.shields.io/github/issues/ksk001100/seahorse.svg)
![](https://img.shields.io/github/forks/ksk001100/seahorse.svg)
![](https://img.shields.io/github/license/ksk001100/seahorse.svg)

A minimal CLI framework written in Rust

## Using

```toml
[dependencies]
seahorse = "0.4.1"
```

## Example

### Multiple action app
```rust
use std::env;
use seahorse::{
    App,
    Action,
    Command,
    Context,
    Flag,
    FlagType,
    color
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let action: Action = |c: &Context| {
        let name = &c.args[2];
        if c.bool_flag("bye") {
            println!("Bye, {}", name);
        } else {
            println!("Hello, {}", name);
        }

        match c.string_flag("other") {
            Some(val) => println!("{}", val),
            _ => println!("Not other...")
        }

        match c.int_flag("age") {
            Some(val) => println!("{} is {} years old", name, val),
            _ => println!("I don't know how old {} is...", name)
        }
    };
    let display_name = color::magenta("
     ██████╗██╗     ██╗
    ██╔════╝██║     ██║
    ██║     ██║     ██║
    ██║     ██║     ██║
    ╚██████╗███████╗██║
    ╚═════╝╚══════╝╚═╝");
    let command = Command::new()
        .name("hello")
        .usage("cli_tool hello [name]")
        .action(action)
        .flags(vec![
            Flag::new("bye", "cli_tool hello [name] --bye", FlagType::Bool),
            Flag::new("other", "cli_tool hello [name] --other [string]", FlagType::String),
            Flag::new("age", "cli_tool hello [name] --age [int]", FlagType::Int),
        ]);

    let app = App::new()
        .name("cli_tool")
        .display_name(display_name)
        .usage("cli_tool [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .commands(vec![command]);

    app.run(args);
}
```

```bash
$ cli_tool hello John --age 10 --other test
Hello, John
test
John is 10 years old 
```

### Single action app
```rust
use std::env;
use seahorse::{SingleApp, Action, color, Context, Flag, FlagType};

fn main() {
    let args: Vec<String> = env::args().collect();
    let action: Action = |c: &Context| {
        let name = &c.args[0];
        if c.bool_flag("bye") {
            println!("Bye, {:?}", name);
        } else {
            println!("Hello, {:?}", name);
        }
    };
    let display_name = color::magenta("
     ██████╗██╗     ██╗
    ██╔════╝██║     ██║
    ██║     ██║     ██║
    ██║     ██║     ██║
    ╚██████╗███████╗██║
    ╚═════╝╚══════╝╚═╝");

    let app = SingleApp::new()
        .name("cli_tool")
        .display_name(display_name)
        .usage("cli_tool [args]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(action)
        .flags(vec![
            Flag::new("bye", "cli_tool args --bye", FlagType::Bool),
        ]);

    app.run(args);
}
```

```bash
$ cli_tool John
Hello, "John"

$ cli_tool John --bye
Bye, "John"
```