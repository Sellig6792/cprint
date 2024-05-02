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

        let mut still_at_start = true;

        let white_spaces = $msg
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
        let msg = $msg.trim_start();

        print!(
            "{}{} {}",
            white_spaces,
            format!(
                "{}{}",
                " ".repeat(12 - "Error".len()),
                colorize_string("Error", Red)
            ),
            msg
        );
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
