use std::fmt::Display;

pub fn black<T: Display>(t: T) -> String {
    format!("\x1b[30m{}\x1b[0m", t)
}

pub fn red<T: Display>(t: T) -> String {
    format!("\x1b[31m{}\x1b[0m", t)
}

pub fn green<T: Display>(t: T) -> String {
    format!("\x1b[32m{}\x1b[0m", t)
}

pub fn yellow<T: Display>(t: T) -> String {
    format!("\x1b[33m{}\x1b[0m", t)
}

pub fn blue<T: Display>(t: T) -> String {
    format!("\x1b[34m{}\x1b[0m", t)
}

pub fn magenta<T: Display>(t: T) -> String {
    format!("\x1b[35m{}\x1b[0m", t)
}

pub fn cyan<T: Display>(t: T) -> String {
    format!("\x1b[36m{}\x1b[0m", t)
}

pub fn white<T: Display>(t: T) -> String {
    format!("\x1b[37m{}\x1b[0m", t)
}

pub fn bg_black<T: Display>(t: T) -> String {
    format!("\x1b[40m{}\x1b[0m", t)
}

pub fn bg_red<T: Display>(t: T) -> String {
    format!("\x1b[41m{}\x1b[0m", t)
}

pub fn bg_green<T: Display>(t: T) -> String {
    format!("\x1b[42m{}\x1b[0m", t)
}

pub fn bg_yellow<T: Display>(t: T) -> String {
    format!("\x1b[43m{}\x1b[0m", t)
}

pub fn bg_blue<T: Display>(t: T) -> String {
    format!("\x1b[44m{}\x1b[0m", t)
}

pub fn bg_magenta<T: Display>(t: T) -> String {
    format!("\x1b[45m{}\x1b[0m", t)
}

pub fn bg_cyan<T: Display>(t: T) -> String {
    format!("\x1b[46m{}\x1b[0m", t)
}

pub fn bg_white<T: Display>(t: T) -> String {
    format!("\x1b[47m{}\x1b[0m", t)
}
