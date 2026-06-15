pub fn nth(n: u32) -> u32 {
    (2..)
        .filter(|x| is_prime(*x))
        .nth(n as usize).unwrap()
}

fn is_prime(num: u32) -> bool {
    for i in 2..=(num as f64).sqrt() as u32 {
        if num.is_multiple_of(i) {
            return false;
        }
    }
    true
}
