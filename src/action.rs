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

pub type ActionWithResult = fn(&Context) -> Result;

pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
pub struct Error {
    pub message: String,
}
