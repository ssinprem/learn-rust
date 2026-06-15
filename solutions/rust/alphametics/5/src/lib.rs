use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn solve(input: &str) -> Option<HashMap<char, u32>> {
    let mut char_set = HashSet::<char>::new();
    for c in input.chars() {
        if c.is_alphabetic() {
            char_set.insert(c);
        }
    }

    println!("{:?}", char_set);

    let mut string = input.split(" == ");
    let q_list : Vec<&str> = string.next().unwrap().split(" + ").collect();
    let r_str = string.next().unwrap().trim();
    let mut l_list : HashSet<char> = q_list.iter().map(|str| str.chars().next().unwrap()).collect();
    l_list.insert(r_str.chars().next().unwrap());
    for perm in (0..=9).permutations(char_set.len()) {
        let mut cd = HashMap::<char, u32>::new();
        for ( i , char ) in char_set.clone().into_iter().enumerate() {
            let n = perm.clone().into_iter().nth(i).unwrap();
            cd.insert(char, n);
        }

        // first first digit each num is 0, skip this solution
        if l_list.iter().any(|char| *cd.get(char).unwrap() == 0 ) {
            continue;
        }

        let sum = r_str.chars()
            .fold(
                0_u64, 
                |acc, c| 
                    acc*10 + *cd.get(&c).unwrap_or(&0_u32) as u64
            );
        let cal : u64 = q_list.iter()
            .map(
                |str|
                    (*str).chars()
                        .fold(
                            0_u64, 
                            |acc, c|
                                acc*10 + *cd.get(&c).unwrap() as u64
                        )
            ).sum();

        // println!("{cd:?} cal = {cal}, sum = {sum}");

        if cal == sum {
            println!("{cd:?}");
            return Some(cd);
        }
    }
    None
}