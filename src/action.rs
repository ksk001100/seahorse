use crate::Context;
use std::error::Error;
use std::result::Result;

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

pub type ActionWithResult = fn(&Context) -> Result<(), Box<dyn Error>>;
