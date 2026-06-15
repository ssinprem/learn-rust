pub fn is_armstrong_number(num: u32) -> bool {
    let string = format!("{}", num);
    let len = string.len();

    let mut sum = 0;
    for c in string.chars() {
        dbg!(c);
        let n = c.to_digit(10).unwrap();
        let mut pow = 1;
        for _i in 0..len {
            pow *= n;
        }
        dbg!(pow);
        sum += pow;
    }
    sum == num
}
