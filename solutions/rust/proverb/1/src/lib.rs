pub fn build_proverb(list: &[&str]) -> String {
    let mut lyrics = String::new();
    let len = list.len();
    if len < 1 {
        return lyrics;
    }
    for i in 0..len {
        if i < len-1 {
            lyrics.push_str(&format!("For want of a {} the {} was lost.\n", list[i], list[i+1]));
        }
    }
    lyrics.push_str(&format!("And all for the want of a {}.", list[0]));
    lyrics
}
