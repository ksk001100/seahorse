# seahorse

[![crates.io](https://img.shields.io/crates/v/seahorse.svg)](https://crates.io/crates/seahorse)
![releases count](https://img.shields.io/github/release/ksk001100/seahorse.svg)
![issues count](https://img.shields.io/github/issues/ksk001100/seahorse.svg)
![forks count](https://img.shields.io/github/forks/ksk001100/seahorse.svg)
![license](https://img.shields.io/github/license/ksk001100/seahorse.svg)
![github actions CI](https://github.com/ksk001100/seahorse/workflows/CI/badge.svg?branch=master)

![Logo](https://repository-images.githubusercontent.com/226840735/d3e77500-51a0-11ea-845e-3cc87714278b)

A minimal CLI framework written in Rust

## Features
- Easy to use
- No dependencies

## Documentation
[Here](https://docs.rs/seahorse)

## Usage
To use seahorse, add this to your Cargo.toml:

```toml
[dependencies]
seahorse = "0.5.0"
```

## Example

```bash
$ git clone https://github.com/ksk001100/seahorse
$ cd seahorse
$ cargo run --example single_app
$ cargo run --example multiple_app
```

### Multiple action application

```rust
use std::env;
use seahorse::{App, Command, color, Flag, FlagType, Context};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name("multiple_app")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("multiple_app [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .commands(vec![hello_command()]);

    app.run(args);
}

fn hello_action(c: &Context) {
    let name = &c.args[1];
    if c.bool_flag("bye") {
        println!("Bye, {}", name);
    } else {
        println!("Hello, {}", name);
    }
    
    match c.int_flag("age") {
        Some(age) => println!("{} is {} years old", name, age),
        None => println!("I don't know {}'s age", name)
    }
}

fn hello_command() -> Command {
    Command::new()
        .name("hello")
        .usage("multiple_app hello [name]")
        .action(hello_action)
        .flags(vec![
            Flag::new("bye", "multiple_app hello [name] --bye", FlagType::Bool),
            Flag::new("age", "multiple_app hello [name] --age [age]", FlagType::Int)
        ])
}
```

```bash
$ cargo run
$ cargo run John --bye
$ cargo run John --age 30
```

### Single action application
```rust
use std::env;
use seahorse::{App, color, Context, Flag, FlagType};

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new()
        .name("single_app")
        .author(env!("CARGO_PKG_AUTHORS"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .usage("single_app [args]")
        .version(env!("CARGO_PKG_VERSION"))
        .action(action)
        .flags(vec![
            Flag::new("bye", "single_app args --bye", FlagType::Bool),
        ]);

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
```

```bash
$ cargo run
$ cargo run Bob
$ cargo run Bob --bye
```

## Contributing
Please read [CONTRIBUTING.md](.github/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License
This project is licensed under [MIT license](LICENSE)

## Code of Conduct
Contribution to the seahorse crate is organized under the terms of the Contributor Covenant, the maintainer of seahorse, @ksk001100, promises to intervene to uphold that code of conduct.
