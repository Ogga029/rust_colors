pub struct Color;

impl Color {
    pub const RESET: &'static str = "\x1b[0m";
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";
    pub const BRIGHT_BLACK: &'static str = "\x1b[90m";
    pub const BRIGHT_RED: &'static str = "\x1b[91m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";
    pub const BRIGHT_WHITE: &'static str = "\x1b[97m";
    pub const BLACK: &'static str = "\x1b[30m";

    pub fn print(text: &str, color: &str) -> String {
        format!("{}{}{}", color, text, Self::RESET)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_paint_red() {
        let colored = Color::print("hello", Color::RED);
        assert_eq!(colored, "\x1b[31mhello\x1b[0m");
    }
}
