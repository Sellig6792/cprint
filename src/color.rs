use std::fmt::Debug;

use colored::{ColoredString, Colorize};

#[derive(Debug)]
pub enum Color {
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

impl From<Color> for String {
    fn from(color: Color) -> Self {
        match color {
            Color::Red => "red".to_string(),
            Color::Green => "green".to_string(),
            Color::Yellow => "yellow".to_string(),
            Color::Blue => "blue".to_string(),
            Color::Magenta => "magenta".to_string(),
            Color::Cyan => "cyan".to_string(),
            Color::White => "white".to_string(),
        }
    }
}

pub trait Coloration<ColorType>
where
    ColorType: Into<Color>,
{
    fn apply_color(&self, color: ColorType) -> ColoredString;
}

impl<FS, ColorType> Coloration<ColorType> for FS
where
    FS: AsRef<str> + ?Sized,
    ColorType: Into<Color>,
{
    fn apply_color(&self, color: ColorType) -> ColoredString {
        match color.into() {
            Color::Red => self.as_ref().red().bold(),
            Color::Green => self.as_ref().green().bold(),
            Color::Yellow => self.as_ref().yellow().bold(),
            Color::Blue => self.as_ref().blue().bold(),
            Color::Magenta => self.as_ref().magenta().bold(),
            Color::Cyan => self.as_ref().cyan().bold(),
            Color::White => self.as_ref().white().bold(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;

    #[test]
    fn coloration_trait_with_str() {
        let a = "Hello".apply_color(Color::Red);
        assert_eq!(a, "Hello".red().bold());
    }

    #[test]
    fn coloration_trait_with_string() {
        let a = String::from("Hello").apply_color(Color::Red);
        assert_eq!(a, "Hello".red().bold());
    }

    #[test]
    fn coloration_trait_with_colored_string() {
        let a = ColoredString::from("Hello").apply_color(Color::Red);
        assert_eq!(a, "Hello".red().bold());
    }
}
