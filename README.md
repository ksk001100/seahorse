# seahorse

A minimal CLI framework written in Rust

## Using

```toml
[dependencies]
seahorse = "0.2.0"
```

## Example

```rust
use std::env;
use seahorse::{App, Command, color};

fn main() {
    let args: Vec<String> = env::args().collect();
    let display_name = color::magenta("
     ██████╗██╗     ██╗
    ██╔════╝██║     ██║
    ██║     ██║     ██║
    ██║     ██║     ██║
    ╚██████╗███████╗██║
    ╚═════╝╚══════╝╚═╝");
    let command = Command {
        name: "hello",
        usage: "cli_tool hello user",
        action: |v: Vec<String>| println!("Hello, {:?}", v)
    };

    let mut app = App::new()
        .name("cli_tool")
        .display_name("display_name")
        .usage("cli_tool [command] [arg]")
        .version(env!("CARGO_PKG_VERSION"))
        .commands(vec![command]);

    app.run(args);
}
```