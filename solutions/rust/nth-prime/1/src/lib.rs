pub fn nth(n: u32) -> u32 {
    let mut prime = 2;

    dbg!(prime);
    for _i in 0..n {
        prime = find_next_prime(prime+1);
        dbg!(prime);
    }
    prime
}

fn find_next_prime(start: u32) -> u32 {
    let mut num = start;

    loop {
        let mut count =0;
        for i in 1..=num {
            if num.is_multiple_of(i) {
                count += 1;
            }
        }
        if count == 2 {
            return num;
        }
        num += 1;
    }
}
