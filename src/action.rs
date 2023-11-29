use crate::Context;

/// Command and application action type
///
/// Example
///
/// ```
/// use seahorse::{Action, Context};
///
/// let action: Action = |c: &Context| {
///     println!("{:?}", c.args);
/// };
/// ```
pub type Action = fn(&Context);

pub type ActionWithResult = fn(&Context) -> Result<(), CommandError>;

#[derive(Debug)]
pub struct CommandError {
    pub message: String,
}

pub fn fail(e: CommandError) {
    eprintln!("Error: {}", e.message);
    std::process::exit(-1);
}
