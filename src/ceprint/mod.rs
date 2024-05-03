/// Print an error like Cargo does. The argument is the message of the error.
/// The end of the title is padded with spaces to make it 12 characters long.
/// # Examples
/// ```rust
/// use cprint::{ceprint, Color, Coloration};
///
/// ceprint!("Failed to compile");
/// ```
#[macro_export]
macro_rules! ceprint {
    ($msg:expr) => {{
        use $crate::coloration::{colorize_string, Color::Red};

        let white_spaces = $crate::_get_white_spaces_at_start!($msg);
        let msg = $msg.trim_start();

        print!("{}{}", white_spaces, $crate::cformat!("Error", msg, Red));
    }};
}

/// Same as [`ceprint!`] but with a newline at the end.
#[macro_export]
macro_rules! ceprintln {
    ($msg:expr) => {
        $crate::ceprint!($msg);
        println!();
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn ceprint_macro() {
        ceprintln!("Failed to compile");
    }
}
