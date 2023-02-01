use std::fmt::Debug;

use colored::{ColoredString, Colorize};

/// Color enum for use with the [`Coloration`] trait or the [cprint!][crate::cprint] and [cprintln!][crate::cprintln] macros.
/// With this enum, you can easily choose a color for your text.
///
/// # Examples
/// ```rust
/// use cprint::Color;
///
/// let red = Color::Red;
/// ```
#[derive(Debug)]
pub enum Color {
    /// Used by Cargo to indicate an error.
    Red,
    /// Used by Cargo to indicate a success.
    Green,
    /// Used by Cargo to indicate a warning.
    Yellow,
    Blue,
    Magenta,
    /// Used by Cargo to indicate a progression.
    Cyan,
    White,
}

/// Convert a [`Color`] to a [`String`].
impl From<Color> for String {
    /// Converts a [`Color`] to a [`String`].
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

/// Trait for coloration of text. [`Coloration<ColorType>`] is the type of the color you want to use. It can be a [`Color`] or a [`String`].
pub trait Coloration<ColorType>
where
    ColorType: Into<Color>,
{
    /// Apply a color and boldness to a string.
    /// # Examples
    /// ```rust
    /// use cprint::{Color, Coloration};
    ///
    /// let string = "An amazing string in green and bold".apply_color(Color::Green);
    /// println!("{}", string);
    /// ```
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

pub fn colorize_string<FS, ColorType>(string: &FS, color: ColorType) -> ColoredString
    where
        FS: AsRef<str> + ?Sized,
        ColorType: Into<Color>,
{
    string.apply_color(color)
}

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use super::*;

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
