use std::collections::HashSet;
use std::collections::BTreeSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();
    for i in 0..possible_anagrams.len() {
        if anagrams_check(word, possible_anagrams[i]) {
            set.insert(possible_anagrams[i]);
        }
    }
    set
}

pub fn anagrams_check(word1: &str, word2: &str) -> bool {

    let mut set1: Vec<char> = word1.to_lowercase().chars().collect();
    let mut set2: Vec<char> = word2.to_lowercase().chars().collect();

    if set1 == set2 {
        return false;
    }

    set1.sort();
    set2.sort();

    dbg!(&set1.clone().into_iter().collect::<String>());
    dbg!(&set2.clone().into_iter().collect::<String>());

    &set1.clone().into_iter().collect::<String>() == &set2.clone().into_iter().collect::<String>()
}
