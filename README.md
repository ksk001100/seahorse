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
- Typed flags(Bool, String, Int, Float)

## Documentation
[Here](https://docs.rs/seahorse)

## Usage
To use seahorse, add this to your Cargo.toml:

```toml
[dependencies]
seahorse = "1.1"
```

## Example

### Run example

```bash
$ git clone https://github.com/ksk001100/seahorse
$ cd seahorse
$ cargo run --example single_app -- --help
$ cargo run --example multiple_app -- --help
```

### Quick Start

```bash
$ cargo new cli
$ cd cli
$ echo 'seahorse = "*"' >> Cargo.toml
```

```rust
use seahorse::{App};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [args]")
        .action(|c| println!("Hello, {:?}", c.args));

    app.run(args);
}
```

```bash
$ cargo build --release
$ ./target/release/cli --help
$ ./target/release/cli John
```

### Multiple command application
```rust
use seahorse::{App, Context, Command};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [name]")
        .action(default_action)
        .command(add_command())
        .command(sub_command());

    app.run(args);
}

fn default_action(c: &Context) {
    println!("Hello, {:?}", c.args);
}

fn add_action(c: &Context) {
    let sum: i32 = c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum();
    println!("{}", sum);
}

fn add_command() -> Command {
    Command::new("add")
        .description("add command")
        .alias("a")
        .usage("cli add(a) [nums...]")
        .action(add_action)
}

fn sub_action(c: &Context) {
    let sum: i32 = c.args.iter().map(|n| n.parse::<i32>().unwrap() * -1).sum();
    println!("{}", sum);
}

fn sub_command() -> Command {
    Command::new("sub")
        .description("sub command")
        .alias("s")
        .usage("cli sub(s) [nums...]")
        .action(sub_action)
}
```

```bash
$ cli John
Hello, ["John"]

$ cli add 32 10 43
85

$ cli sub 12 23 89
-124
```

### Branch processing by flag

```rust
use seahorse::{App, Command, Context, Flag, FlagType, error::FlagError};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let app = App::new(env!("CARGO_PKG_NAME"))
        .description(env!("CARGO_PKG_DESCRIPTION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .version(env!("CARGO_PKG_VERSION"))
        .usage("cli [name]")
        .action(default_action)
        .flag(
            Flag::new("bye", FlagType::Bool)
                .description("Bye flag")
                .alias("b"),
        )
        .flag(
            Flag::new("age", FlagType::Int)
                .description("Age flag")
                .alias("a"),
        )
        .command(calc_command());

    app.run(args);
}

fn default_action(c: &Context) {
    if c.bool_flag("bye") {
        println!("Bye, {:?}", c.args);
    } else {
        println!("Hello, {:?}", c.args);
    }

    if let Ok(age) = c.int_flag("age") {
        println!("{:?} is {} years old", c.args, age);
    }
}

fn calc_action(c: &Context) {
    match c.string_flag("operator") {
        Ok(op) => {
            let sum: i32 = match &*op {
                "add" => c.args.iter().map(|n| n.parse::<i32>().unwrap()).sum(),
                "sub" => c.args.iter().map(|n| n.parse::<i32>().unwrap() * -1).sum(),
                _ => panic!("undefined operator..."),
            };

            println!("{}", sum);
        }
        Err(e) => match e {
            FlagError::Undefined => panic!("undefined operator..."), 
            FlagError::ArgumentError => panic!("argument error..."), 
            FlagError::NotFound => panic!("not found flag..."), 
            FlagError::ValueTypeError => panic!("value type mismatch..."), 
            FlagError::TypeError => panic!("flag type mismatch..."), 
        },
    }
}

fn calc_command() -> Command {
    Command::new("calc")
        .description("calc command")
        .alias("cl, c")
        .usage("cli calc(cl, c) [nums...]")
        .action(calc_action)
        .flag(
            Flag::new("operator", FlagType::String)
                .description("Operator flag(ex. cli calc --operator add 1 2 3)")
                .alias("op"),
        )
}
```

```bash
$ cli John
Hello, ["John"]

$ cli John --bye
Bye, ["John"]

$ cli John --age 10
Hello, ["John"]
["John"] is 10 years old

$ cli John -b -a=40
Bye, ["John"]
["John"] is 40 years old

$ cli calc --operator add 1 2 3 4 5
15

$ cli calc -op sub 10 6 3 2
-21
```

## Contributing
Please read [CONTRIBUTING.md](.github/CONTRIBUTING.md) for details on our code of conduct, and the process for submitting pull requests to us.

## License
This project is licensed under [MIT license](LICENSE)

## Code of Conduct
Contribution to the seahorse crate is organized under the terms of the Contributor Covenant, the maintainer of seahorse, @ksk001100, promises to intervene to uphold that code of conduct.
