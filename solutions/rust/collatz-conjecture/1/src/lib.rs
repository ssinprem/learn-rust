pub fn collatz(n: u64) -> Option<u64> {
    let mut step: u64 = 0;
    let mut num = n;
    // zero is not positive interger
    if n == 0 {
        return None;
    }
    while num != 1 {
        match num.is_multiple_of(2) {
            true  => num /= 2,
            false => num = num *3 +1
        }
        step += 1;
    }
    Some(step)
}
