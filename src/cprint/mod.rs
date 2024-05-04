/// Print a message like Cargo does. If you pass only one string, the first word is the title and the rest is the message. If you pass two strings the first one is the title and the second one is the message.
/// You can specify the color of the title with a predefined color from the [`colored::Color`] enum or with RGB values `(r, g, b)`. To specify the color use `=>` after the strings.
/// The end of the title is padded with spaces to make it 12 characters long.
/// # Examples
/// ```rust
/// use cprint::{cprint, Color, Coloration};
///
/// cprint!("Compiling", "main.rs" => Green);
/// ```
#[macro_export]
macro_rules! cprint {
    ($title:expr, $msg:expr => $color:ident) => {{
        $crate::Color::$color;
        print!("{}", $crate::cformat!($title, $msg => $color))
    }};

    ($title:expr, $msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        $crate::Color::TrueColor { r: $r, g: $g, b: $b };
        print!("{}", $crate::cformat!($title, $msg => ($r, $g, $b)))
    }};

    ($title:expr, $msg:expr) => {{
        print!("{}", $crate::cformat!($title, $msg => Green))
    }};

    ($msg:expr => $color:ident) => {{
        $crate::Color::$color;
        print!("{}",$crate::cformat!($msg => $color))
    }};

    ($msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        $crate::Color::TrueColor { r: $r, g: $g, b: $b };
        print!("{}",$crate::cformat!($msg => ($r, $g, $b)))
    }};

    ($msg:expr) => {{
        print!("{}",$crate::cformat!($msg => Green))
    }};
}

/// Same as [`cprint!`] but with a newline at the end.
#[macro_export]
macro_rules! cprintln {
    ($title:expr, $msg:expr => $color:ident) => {{
        $crate::Color::$color;
        println!("{}", $crate::cformat!($title, $msg => $color))
    }};

    ($title:expr, $msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        $crate::Color::TrueColor { r: $r, g: $g, b: $b };
        println!("{}", $crate::cformat!($title, $msg => ($r, $g, $b)))
    }};

    ($title:expr, $msg:expr) => {{
        println!("{}", $crate::cformat!($title, $msg => Green))
    }};

    ($msg:expr => $color:ident) => {{
        $crate::Color::$color;
        println!("{}", $crate::cformat!($msg => $color))
    }};

    ($msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        $crate::Color::TrueColor { r: $r, g: $g, b: $b };
        println!("{}", $crate::cformat!($msg => ($r, $g, $b)))
    }};

    ($msg:expr) => {{
        println!("{}", $crate::cformat!($msg => Green))
    }};
}
