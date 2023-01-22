/// Print a message like Cargo does. The first argument is the title of the message, the second argument is the message itself and the third argument is the color of the title.
/// The end of the title is padded with spaces to make it 12 characters long.
/// # Examples
/// ```rust
/// use cprint::{cprint, Color, Coloration};
///
/// cprint!("Compiling", "main.rs", Color::Green);
/// ```
#[macro_export]
macro_rules! cprint {
    ($title:expr, $msg:expr, $color:expr) => {
        print!(
            "{} {}",
            format!("{}{}", " ".repeat(12 - $title.len()), $title).apply_color($color),
            $msg
        );
    };
}

/// Same as [`cprint!`] but with a newline at the end.
#[macro_export]
macro_rules! cprintln {
    ($title:expr, $msg:expr, $color:expr) => {
        cprint!($title, $msg, $color);
        println!();
    };
}
