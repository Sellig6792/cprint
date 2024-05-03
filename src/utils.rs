#[macro_export]
macro_rules! _get_white_spaces_at_start {
    ($text:expr) => {{
        let mut still_at_start = true;

        $text
            .chars()
            .filter_map(|c| {
                if c.is_whitespace() && still_at_start {
                    Some(c)
                } else {
                    still_at_start = false;
                    None
                }
            })
            .collect::<String>()
    }};
}

#[macro_export]
macro_rules! _get_title_and_message {
    ($text:expr) => {{
        let white_spaces = $crate::_get_white_spaces_at_start!($text);
        let title = white_spaces.clone()
            + &$text
                .trim_start_matches(&white_spaces)
                .chars()
                .take_while(|c| !c.is_whitespace())
                .collect::<String>();
        let message = &$text.trim_start_matches(&title)[1..];

        (title, message)
    }};
}
