/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut cnt = 0;
    let mut sum = 0;
    let string = isbn.replace("-", "");
    
    for (i,char) in string.chars().enumerate() {
        match (i,char) {
            (_,x) if x.is_numeric() => {
                cnt += 1;
                sum += x.to_digit(10).unwrap() * ( 10-i as u32 );
            }
            (9,'X') => {
                cnt += 1;
                sum += 10 * ( 10-i as u32 );
            }
            _ => return false
        }
    }
    sum % 11 == 0 && cnt == 10
}
