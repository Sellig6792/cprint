/// Print an error message like Cargo does. If you pass only one string, the first word is the title and the rest is the message. If you pass two strings the first one is the title and the second one is the message.
/// You can specify the color of the title with a predefined color from the [`colored::Color`] enum or with RGB values `(r, g, b)`. To specify the color use `=>` after the strings.
/// The end of the title is padded with spaces to make it 12 characters long.
/// # Examples
/// ```rust
/// use cprint::{ceprint, Color, Coloration};
///
/// ceprint!("Failed to compile");
/// ```
#[macro_export]
macro_rules! ceprint {
    ($title:expr, $msg:expr => $color:ident) => {{
        $crate::Color::$color;
        eprint!("{}", $crate::cformat!($title, $msg => $color))
    }};

    ($title:expr, $msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        $crate::Color::TrueColor { r: $r, g: $g, b: $b };
        eprint!("{}", $crate::cformat!($title, $msg => ($r, $g, $b)))
    }};

    ($title:expr, $msg:expr) => {{
        eprint!("{}", $crate::cformat!($title, $msg => Red))
    }};

    ($msg:expr => $color:ident) => {{
        $crate::Color::$color;
        eprint!("{}",$crate::cformat!($msg => $color))
    }};

    ($msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        $crate::Color::TrueColor { r: $r, g: $g, b: $b };
        eprint!("{}",$crate::cformat!($msg => ($r, $g, $b)))
    }};

    ($msg:expr) => {{
        eprint!("{}",$crate::cformat!($msg => Red))
    }};
}

/// Same as [`ceprint!`] but with a newline at the end.
#[macro_export]
macro_rules! ceprintln {
    ($title:expr, $msg:expr => $color:ident) => {{
        $crate::Color::$color;
        eprintln!("{}", $crate::cformat!($title, $msg => $color))
    }};

    ($title:expr, $msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        $crate::Color::TrueColor { r: $r, g: $g, b: $b };
        eprintln!("{}", $crate::cformat!($title, $msg => ($r, $g, $b)))
    }};

    ($title:expr, $msg:expr) => {{
        eprintln!("{}", $crate::cformat!($title, $msg => Red))
    }};

    ($msg:expr => $color:ident) => {{
        $crate::Color::$color;
        eprintln!("{}", $crate::cformat!($msg => $color))
    }};

    ($msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        $crate::Color::TrueColor { r: $r, g: $g, b: $b };
        eprintln!("{}", $crate::cformat!($msg => ($r, $g, $b)))
    }};

    ($msg:expr) => {{
        eprintln!("{}", $crate::cformat!($msg => Red))
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn ceprint_title_message_color() {
        ceprint!("Failed", "to compile" => Red);
    }

    #[test]
    fn ceprint_title_message_rgb() {
        ceprint!("Failed", "to compile main.rs" => (255, 0, 0));
    }

    #[test]
    fn ceprint_title_message() {
        ceprint!("Failed", "to compile main.rs");
    }

    #[test]
    fn ceprint_message_color() {
        ceprint!("Failed to compile" => Red);
    }

    #[test]
    fn ceprint_message_rgb() {
        ceprint!("Failed to compile" => (255, 0, 0));
    }

    #[test]
    fn ceprint_message() {
        ceprint!("Failed to compile");
    }
}
