pub fn factors(n: u64) -> Vec<u64> {
    let mut num = n;
    let mut factor = Vec::new();

    while let Some(f) = find_first_factor(num) {
        factor.push(f);
        num /= f;
    }
    factor
}

fn find_first_factor(n: u64) -> Option<u64> {
    if n <= 1 {
        return None;
    }
    for i in 2..=(n as f64).sqrt() as u64 {
        if n.is_multiple_of(i) {
            return Some(i);
        }
    }
    // cannot find the factor, return prime number
    Some(n)
}