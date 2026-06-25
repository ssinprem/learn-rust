pub fn is_prime(num: u64) -> bool {
    ! (2..=(num as f32).sqrt() as u64)
        .any(|n| num.is_multiple_of(n))
}

pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    (2..=upper_bound).filter(|&num | {
        is_prime(num)
    }).collect()
}
