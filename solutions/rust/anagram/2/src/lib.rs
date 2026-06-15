use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    let mut word_set: Vec<char> = word.to_lowercase().chars().collect();
    word_set.sort();
    
    for anagram in possible_anagrams {
        if word.to_lowercase() == anagram.to_lowercase() {
            continue;
        }
        let mut anagram_set: Vec<char> = anagram.to_lowercase().chars().collect();
        anagram_set.sort();
        if word_set == anagram_set {
            set.insert(*anagram);
        }
    }
    set
}
