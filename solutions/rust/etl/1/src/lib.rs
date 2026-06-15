use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {

    let mut b = BTreeMap::<char, i32>::new();

    for (score, char_set) in h {
        for char in char_set {
            b.insert(char.to_ascii_lowercase(), *score);
        }
    }
    b
}
