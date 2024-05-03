/// Print a message like Cargo does. The first argument is the title of the message, the second argument is the message itself and the third argument is the color of the title.
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
        print!("{}", $crate::cformat!($title, $msg => $color))
    }};

    ($title:expr, $msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        print!("{}", $crate::cformat!($title, $msg => ($r, $g, $b)))
    }};

    ($title:expr, $msg:expr) => {{
        print!("{}", $crate::cformat!($title, $msg => Green))
    }};

    ($msg:expr => $color:ident) => {{
        print!("{}",$crate::cformat!($msg => $color))
    }};

    ($msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
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
        println!("{}", $crate::cformat!($title, $msg => $color))
    }};

    ($title:expr, $msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        println!("{}", $crate::cformat!($title, $msg => ($r, $g, $b)))
    }};

    ($title:expr, $msg:expr) => {{
        println!("{}", $crate::cformat!($title, $msg => Green))
    }};

    ($msg:expr => $color:ident) => {{
        println!("{}", $crate::cformat!($msg => $color))
    }};

    ($msg:expr => ($r:expr, $g:expr, $b:expr)) => {{
        println!("{}", $crate::cformat!($msg => ($r, $g, $b)))
    }};

    ($msg:expr) => {{
        println!("{}", $crate::cformat!($msg => Green))
    }};
}
