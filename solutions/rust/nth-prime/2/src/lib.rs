pub fn nth(n: u32) -> u32 {
    let mut prime = 2;

    for _i in 0..n {
        prime = find_next_prime(prime);
        dbg!(prime);
    }
    prime
}

fn find_next_prime(start: u32) -> u32 {
    let mut num = start;

    'outloop: loop {
        num += 1;
        for i in 2..num {
            if num.is_multiple_of(i) {
                continue 'outloop;
            }
        }
        return num;
    }
}
