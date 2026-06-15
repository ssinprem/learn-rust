use std::collections::HashMap;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let hs = sentence.to_ascii_lowercase().chars()
        .filter(|c| c.is_alphabetic())
        .fold(HashMap::<char, u32>::new(), 
            |mut hs, c| {
                hs.entry(c)
                    .and_modify(|e| *e += 1)
                    .or_insert(1);
                hs
            }
        );
    ('a'..='z').all(|c| *hs.get(&c).unwrap_or(&0) > 0)
}
