/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() {
        return None;
    }

    Some(
        (0..s1.len()).map(|index| {
            if s1.chars().nth(index) != s2.chars().nth(index) {
                1
            } else {
                0
            }
        }).sum()
    )
}
