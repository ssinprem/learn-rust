pub fn raindrops(n: u32) -> String {
    let  mut text = String::new();
    if n.is_multiple_of(3) {
        text.push_str("Pling");
    } 
    if n.is_multiple_of(5) {
        text.push_str("Plang");
    } 
    if n.is_multiple_of(7) {
        text.push_str("Plong");
    }

    if ! text.is_empty() {
        text
    } else {
        format!("{}", n)
    }
}
