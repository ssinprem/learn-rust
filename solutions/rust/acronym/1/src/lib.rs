pub fn abbreviate(phrase: &str) -> String {
    let mut string = String::new();
    for word in phrase.split(" ").flat_map(|x| x.split("-"))
    {
        let mut first_char = true;
        for i in 0..word.len() {
            let char = word.chars().nth(i).unwrap();
            if ! char.is_alphabetic() {
                continue;
            }
            if first_char {
                string.push(char.to_ascii_uppercase());
                first_char = false;
            } else if char.is_uppercase() && ! word.chars().all(|x| x.is_uppercase()) {
                string.push(char);
            }
        }
    }
    string
}
