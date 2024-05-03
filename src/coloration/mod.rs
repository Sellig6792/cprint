pub use colored::Color;
use colored::{ColoredString, Colorize};

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
    /// let string = "An amazing string in green and bold".as_colored_title(Color::Green);
    /// println!("{}", string);
    /// ```
    fn as_colored_title(&self, color: ColorType) -> ColoredString;
}

impl<FS, ColorType> Coloration<ColorType> for FS
where
    FS: AsRef<str> + ?Sized,
    ColorType: Into<Color>,
{
    fn as_colored_title(&self, color: ColorType) -> ColoredString {
        self.as_ref().color(color.into()).bold()
    }
}

pub fn colorize_string<FS, ColorType>(string: &FS, color: ColorType) -> ColoredString
where
    FS: AsRef<str> + ?Sized,
    ColorType: Into<Color>,
{
    string.as_colored_title(color)
}

#[cfg(test)]
mod tests {
    use colored::Colorize;

    use super::*;

    #[test]
    fn coloration_trait_with_str() {
        let a = "Hello".as_colored_title(Color::Red);
        assert_eq!(a, "Hello".red().bold());
    }

    #[test]
    fn coloration_trait_with_string() {
        let a = String::from("Hello").as_colored_title(Color::Red);
        assert_eq!(a, "Hello".red().bold());
    }

    #[test]
    fn coloration_trait_with_colored_string() {
        let a = ColoredString::from("Hello").as_colored_title(Color::Red);
        assert_eq!(a, "Hello".red().bold());
    }
}
