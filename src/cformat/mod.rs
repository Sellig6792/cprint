/// Get a string of a message like Cargo does. If you pass only one string, the first word is the title and the rest is the message. If you pass two strings the first one is the title and the second one is the message.
/// You can specify the color of the title with a predefined color from the [`colored::Color`] enum or with RGB values `(r, g, b)`. To specify the color use `=>` after the strings.
/// The end of the title is padded with spaces to make it 12 characters long.
///
/// ```rust
/// # use cprint::{cformat, Color, Coloration};
/// // Specifying the color with a predefined color
/// // ✅: You can put spaces in the title, so it can be something like "Cleaning up"
/// let string = cformat!("Cleaning up", "the mess" => Green);
/// let string = cformat!("Using", "cprint crate!" => Green);
///
/// // Specifying the color with RGB values
/// let string = cformat!("Using",  "cprint crate!" => (255, 255, 0));
///
/// // Using the default color (Green)
/// let string = cformat!("Using", "cprint crate!");
///
/// // Using only one string. The first word is the title and the rest is the message.
/// // ⚠️: The title is the first word of the string, so it must not contain any spaces.
/// let string = cformat!("Compiling main.rs" => Green);
///
/// // Using only one string with RGB values
/// // ⚠️: The title is the first word of the string, so it must not contain any spaces.
/// let string = cformat!("Compiling main.rs" => (0, 255, 0));
///
/// // Using only one string with the default color (Green)
/// // ⚠️: The title is the first word of the string, so it must not contain any spaces.
/// let string = cformat!("Compiling main.rs");
///
/// ```
#[macro_export]
macro_rules! cformat {
    ($title:expr, $msg:expr => $color:ident) => {{
        use $crate::coloration::Coloration;

        let white_spaces = $crate::_get_white_spaces_at_start!($title);
        let title = $title.trim_start();

        format!(
            "{}{} {}",
            white_spaces,
            format!("{}{}", " ".repeat(12 - title.len()), title).as_colored_title($crate::Color::$color),
            $msg
        )
    }};

    ($title:expr, $msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        use $crate::coloration::Coloration;

        let white_spaces = $crate::_get_white_spaces_at_start!($title);
        let title = $title.trim_start();

        format!(
            "{}{} {}",
            white_spaces,
            format!("{}{}", " ".repeat(12 - title.len()), title).as_colored_title($crate::Color::TrueColor { r: $r, g: $g, b: $b }),
            $msg
        )
    }};

    ($title:expr, $msg:expr) => {{
        $crate::cformat!($title, $msg => Green)
    }};

    ($msg:expr => $color:ident) => {{
        let (title, msg) = $crate::_get_title_and_message!($msg);
        $crate::cformat!(title, msg => $color)
    }};

    ($msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        let (title, msg) = $crate::_get_title_and_message!($msg);
        $crate::cformat!(title, msg => ($r, $g, $b))
    }};

    ($msg:expr) => {{
        $crate::cformat!($msg => Green)
    }};
}

#[cfg(test)]
mod tests {
    use crate::{Color, Coloration};

    #[test]
    fn cformat_title_message_color() {
        let string = cformat!("Cleaning up", "the mess" => Green);
        let right = format!(
            "{} {}",
            format!("{}{}", " ".repeat(12 - "Cleaning up".len()), "Cleaning up")
                .as_colored_title(Color::Green),
            "the mess"
        );
        assert_eq!(string, right);
    }

    #[test]
    fn cformat_title_message_rgb() {
        let string = cformat!("Cleaning up", "the mess" => (255, 255, 0));
        let right = format!(
            "{} {}",
            format!("{}{}", " ".repeat(12 - "Cleaning up".len()), "Cleaning up").as_colored_title(
                Color::TrueColor {
                    r: 255,
                    g: 255,
                    b: 0,
                }
            ),
            "the mess"
        );
        assert_eq!(string, right);
    }

    #[test]
    fn cformat_title_message() {
        let string = cformat!("Cleaning up", "the mess");
        let right = format!(
            "{} {}",
            format!("{}{}", " ".repeat(12 - "Cleaning up".len()), "Cleaning up")
                .as_colored_title(Color::Green),
            "the mess"
        );
        assert_eq!(string, right);
    }

    #[test]
    fn cformat_message_color() {
        let string = cformat!("Testing cformat!" => Green);
        let right = format!(
            "{} {}",
            format!("{}{}", " ".repeat(12 - "Testing".len()), "Testing")
                .as_colored_title(Color::Green),
            "cformat!"
        );
        assert_eq!(string, right);
    }

    #[test]
    fn cformat_message_rgb() {
        let string = cformat!("Testing cformat!" => (255, 255, 0));
        let right = format!(
            "{} {}",
            format!("{}{}", " ".repeat(12 - "Testing".len()), "Testing").as_colored_title(
                Color::TrueColor {
                    r: 255,
                    g: 255,
                    b: 0,
                }
            ),
            "cformat!"
        );
        assert_eq!(string, right);
    }

    #[test]
    fn cformat_message() {
        let string = cformat!("Testing cformat!");
        let right = format!(
            "{} {}",
            format!("{}{}", " ".repeat(12 - "Testing".len()), "Testing")
                .as_colored_title(Color::Green),
            "cformat!"
        );
        assert_eq!(string, right);
    }
}
