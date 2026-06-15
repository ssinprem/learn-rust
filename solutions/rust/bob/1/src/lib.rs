pub fn reply(message: &str) -> &str {
    match (
        message.trim().is_empty(),
        !message.chars().any(|c| c.is_ascii_lowercase()) &&
        message.chars().any(|c| c.is_ascii_uppercase()),
        message.trim().ends_with('?')
    ) {
        (true, _, _ )   => "Fine. Be that way!",
        (_, true, true) => "Calm down, I know what I'm doing!",
        (_, _, true)    => "Sure.",
        (_, true, _)    => "Whoa, chill out!",
        _               => "Whatever."
    }
}
