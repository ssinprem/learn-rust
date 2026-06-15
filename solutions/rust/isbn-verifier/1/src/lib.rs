/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut sum = 0;
    if isbn.chars().any(|c| !c.is_numeric() && c != '-' && c != 'X') {
        return false;
    }
    let filter_char = isbn.chars().filter(|c| c.is_numeric() || *c == 'X');
    let str_num = filter_char
        .map(|c| c.to_digit(10).unwrap_or(10));

    let mut size = 0;
    for (index, num) in str_num.enumerate() {
        if num == 10 && index != 9 {
            return false
        }
        size += 1;
        sum += num * (10 - index as u32);
    }
    if size != 10 {
        return false
    }
    sum % 11 == 0
}
