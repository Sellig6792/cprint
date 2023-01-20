


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

#[macro_export]
macro_rules! cprintln {
    ($title:expr, $msg:expr, $color:expr) => {
        cprint!($title, $msg, $color);
        println!();
    };
}
