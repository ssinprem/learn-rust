use std::{cmp::Ordering::{Greater, Less, Equal}, collections::HashSet};

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    let hs: HashSet::<u64> = (0..num).filter(|n| *n==0 || num.is_multiple_of(*n)).collect();
    if hs.is_empty() {
        None
    } else {
        Some(
            match hs.iter().sum::<u64>().cmp(&num) {
                Equal => Classification::Perfect,
                Less => Classification::Deficient,
                Greater => Classification::Abundant
            }
        )
    }
}
