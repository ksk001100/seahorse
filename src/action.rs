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

pub type ActionWithResult = fn(&Context) -> ActionResult;

pub type ActionResult = Result<(), ActionError>;

#[derive(Debug)]
pub struct ActionError {
    pub message: String,
}

pub fn fail(e: ActionError) {
    eprintln!("Error: {}", e.message);
    std::process::exit(-1);
}
