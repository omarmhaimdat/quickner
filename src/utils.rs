#[warn(dead_code)]
#[derive(Copy)]
/// convert String to colored String with ANSI escape codes
pub(crate) enum TermColor {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
}

impl Clone for TermColor {
    fn clone(&self) -> Self {
        *self
    }
}

/// Convert String to colored String with ANSI escape codes
/// # Examples
/// ```
/// use utils::colorize;
/// let colored_string = colorize("Hello World", TermColor::Red);
/// ```
pub(crate) fn colorize(text: &str, color: TermColor) -> String {
    let color_code = match color {
        TermColor::Red => 31,
        TermColor::Green => 32,
        TermColor::Yellow => 33,
        TermColor::Blue => 34,
        TermColor::Magenta => 35,
        TermColor::Cyan => 36,
    };
    format!("\x1b[{color_code}m{text}\x1b[0m")
}
