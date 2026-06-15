use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palindrome {
    value :u64,
    factors :HashSet<(u64,u64)>
}

impl Palindrome {
    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn into_factors(self) -> HashSet<(u64, u64)> {
        self.factors
    }
}

fn is_palindrome(num: u64) -> bool {
    num.to_string() == num.to_string().chars().rev().collect::<String>()
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut hm = HashMap::<u64,Palindrome>::new();
    for i in min..=max {
        for j in i..=max {
            let result = i * j;
            if is_palindrome(result) {
                hm.entry(result)
                    .and_modify(|p| _ = p.factors.insert((i,j)))
                    .or_insert(Palindrome { value: result, factors: HashSet::from([(i,j)]) });
            }
        }
    }
    let mut list = hm.keys().copied().collect::<Vec<u64>>();
    list.sort();
    if let (Some(first),Some(last)) = (list.first(),list.last()) {
        Some((
            hm.get(first).unwrap().clone(),
            hm.get(last).unwrap().clone()
        ))
    } else {
        None
    }
}
