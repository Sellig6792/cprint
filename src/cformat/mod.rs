/// Get a string of a message like Cargo does. The first argument is the title of the message, the second argument is the message itself and the third argument is the color of the title.
/// The end of the title is padded with spaces to make it 12 characters long.
/// # Examples
/// ```rust
/// use cprint::{cformat, Color, Coloration};
///
/// let string = cformat!("Compiling", "main.rs", Color::Green);
/// println!("{}", string);
/// ```
#[macro_export]
macro_rules! cformat {
    ($title:expr, $msg:expr, $color:expr) => {{
        use $crate::coloration::Coloration;

        let mut still_at_start = true;

        let white_spaces = $title
            .chars()
            .filter_map(|c| {
                if c.is_whitespace() && still_at_start {
                    Some(c)
                } else {
                    still_at_start = false;
                    None
                }
            })
            .collect::<String>();
        let title = $title.trim_start();

        format!(
            "{}{} {}",
            white_spaces,
            format!("{}{}", " ".repeat(12 - title.len()), title).apply_color($color),
            $msg
        )
    }};
}

#[cfg(test)]
mod tests {
    use crate::{Color, Coloration};

    #[test]
    fn test_cformat() {
        let string = cformat!("Compiling", "main.rs", Color::Green);
        let right = format!("{} {}", "   Compiling".apply_color(Color::Green), "main.rs");
        assert_eq!(string, right);
    }

    #[test]
    fn test_cformat_with_spaces() {
        let string = cformat!("Pre Build", "Parsing `main.rs`...", Color::Green);
        let right = format!(
            "{} {}",
            "   Pre Build".apply_color(Color::Green),
            "Parsing `main.rs`..."
        );
        assert_eq!(string, right);
    }

    #[test]
    fn test_cformat_with_carry_return_at_start() {
        let string = cformat!("\rPre Build", "Parsing `main.rs`...", Color::Green);
        let right = format!(
            "\r{} {}",
            "   Pre Build".apply_color(Color::Green),
            "Parsing `main.rs`..."
        );
        assert_eq!(string, right);
    }
}
