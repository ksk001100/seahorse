use std::fmt::Display;

/// Get the text with black text
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::black("Hello");
/// println!("{}", text);
/// ```
pub fn black<T: Display>(t: T) -> String {
    format!("\x1b[30m{}\x1b[0m", t)
}

/// Get the text with red text
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::red("Hello");
/// println!("{}", text);
/// ```
pub fn red<T: Display>(t: T) -> String {
    format!("\x1b[31m{}\x1b[0m", t)
}

/// Get the text with green text
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::green("Hello");
/// println!("{}", text);
/// ```
pub fn green<T: Display>(t: T) -> String {
    format!("\x1b[32m{}\x1b[0m", t)
}

/// Get the text with yellow text
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::yellow("Hello");
/// println!("{}", text);
/// ```
pub fn yellow<T: Display>(t: T) -> String {
    format!("\x1b[33m{}\x1b[0m", t)
}

/// Get the text with blue text
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::blue("Hello");
/// println!("{}", text);
/// ```
pub fn blue<T: Display>(t: T) -> String {
    format!("\x1b[34m{}\x1b[0m", t)
}

/// Get the text with magenta text
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::magenta("Hello");
/// println!("{}", text);
/// ```
pub fn magenta<T: Display>(t: T) -> String {
    format!("\x1b[35m{}\x1b[0m", t)
}

/// Get the text with cyan text
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::cyan("Hello");
/// println!("{}", text);
/// ```
pub fn cyan<T: Display>(t: T) -> String {
    format!("\x1b[36m{}\x1b[0m", t)
}

/// Get the text with white text
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::white("Hello");
/// println!("{}", text);
/// ```
pub fn white<T: Display>(t: T) -> String {
    format!("\x1b[37m{}\x1b[0m", t)
}

/// Get text with black background
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::bg_black("Hello");
/// println!("{}", text);
/// ```
pub fn bg_black<T: Display>(t: T) -> String {
    format!("\x1b[40m{}\x1b[0m", t)
}

/// Get text with red background
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::bg_red("Hello");
/// println!("{}", text);
/// ```
pub fn bg_red<T: Display>(t: T) -> String {
    format!("\x1b[41m{}\x1b[0m", t)
}

/// Get text with green background
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::bg_green("Hello");
/// println!("{}", text);
/// ```
pub fn bg_green<T: Display>(t: T) -> String {
    format!("\x1b[42m{}\x1b[0m", t)
}

/// Get text with yellow background
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::bg_yellow("Hello");
/// println!("{}", text);
/// ```
pub fn bg_yellow<T: Display>(t: T) -> String {
    format!("\x1b[43m{}\x1b[0m", t)
}

/// Get text with blue background
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::bg_blue("Hello");
/// println!("{}", text);
/// ```
pub fn bg_blue<T: Display>(t: T) -> String {
    format!("\x1b[44m{}\x1b[0m", t)
}

/// Get text with magenta background
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::bg_magenta("Hello");
/// println!("{}", text);
/// ```
pub fn bg_magenta<T: Display>(t: T) -> String {
    format!("\x1b[45m{}\x1b[0m", t)
}

/// Get text with cyan background
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::bg_cyan("Hello");
/// println!("{}", text);
/// ```
pub fn bg_cyan<T: Display>(t: T) -> String {
    format!("\x1b[46m{}\x1b[0m", t)
}

/// Get text with white background
///
/// Example
/// ```
/// use seahorse::color;
///
/// let text = color::bg_white("Hello");
/// println!("{}", text);
/// ```
pub fn bg_white<T: Display>(t: T) -> String {
    format!("\x1b[47m{}\x1b[0m", t)
}
