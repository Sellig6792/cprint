/// Get a string of a message like Cargo does. The first argument is the title of the message, the second argument is the message itself and the third argument is the color of the title.
/// The end of the title is padded with spaces to make it 12 characters long.
/// # Examples
/// ```rust
/// use cprint::{cformat, Color, Coloration};
///
/// let string = cformat!("Compiling", "main.rs" => Green); // "=> Green" is optional, it's the default color
/// println!("{}", string);
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
    fn test_cformat() {
        let string = cformat!("Compiling", "main.rs" => Green);
        let right = format!(
            "{} {}",
            "   Compiling".as_colored_title(Color::Green),
            "main.rs"
        );
        assert_eq!(string, right);
    }

    #[test]
    fn test_cformat_with_spaces() {
        let string = cformat!("Pre Build", "Parsing `main.rs`..." => Green);
        let right = format!(
            "{} {}",
            "   Pre Build".as_colored_title(Color::Green),
            "Parsing `main.rs`..."
        );
        assert_eq!(string, right);
    }

    #[test]
    fn test_cformat_with_carry_return_at_start() {
        let string = cformat!("\rPre-Build parsing `main.rs`..." => (0, 255, 0));
        let right = format!(
            "\r{} {}",
            "   Pre-Build".as_colored_title(Color::TrueColor { r: 0, g: 255, b: 0 }),
            "parsing `main.rs`..."
        );
        assert_eq!(string, right);
    }
}
