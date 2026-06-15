use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut hash = HashSet::new();
    for c in candidate.to_ascii_lowercase().chars() {
        if c.is_alphanumeric() {
            _ = hash.insert(c);
        }
    }
    hash.len() == candidate.chars().filter(|c| c.is_alphanumeric()).collect::<String>().len()
}
