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
