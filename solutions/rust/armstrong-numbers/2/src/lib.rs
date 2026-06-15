pub fn is_armstrong_number(num: u32) -> bool {
    let string = format!("{}", num);
    let len = string.len();

    let mut sum = 0;
    for c in string.chars() {
        let n = c.to_digit(10).unwrap();
        sum += n.pow(len as u32);
    }
    sum == num
}
