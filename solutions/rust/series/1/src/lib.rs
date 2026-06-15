pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        Vec::<String>::new()
    } else {
        (0..=digits.len()-len).map(|i| digits[i..i+len].to_string()).collect()
    }
}
