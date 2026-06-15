pub fn translate(input: &str) -> String {
    let mut words: Vec<String> = Vec::new();
    for mut word in input.split(" ").map(|w| w.to_string()) {
        words.push(
            if word.starts_with(|c| "aeiou".contains(c)) ||
                word.starts_with("xr") || word.starts_with("yt") {
                word 
            } else if word.contains("qu") {
                word = rotate_string_left(&word, 1);
                while ! word.starts_with(|c| "aeio".contains(c)) {
                    word = rotate_string_left(&word, 1);
                }
                word
            } else {
                word = rotate_string_left(&word, 1);
                while ! word.starts_with(|c| "aeiouy".contains(c)) {
                    word = rotate_string_left(&word, 1);
                }
                word
            }
            + "ay"
        )
    }
    words.join(" ")
}

fn rotate_string_left(s: &str, n: usize) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    chars.rotate_left(n);
    chars.into_iter().collect()
}