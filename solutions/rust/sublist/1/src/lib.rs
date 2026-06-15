#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    let first_len = first_list.len();
    let second_len = second_list.len();
    let mut big_list = first_list;
    let mut small_list = second_list;
    let mut ret = Comparison::Superlist;
    if first_len < second_len {
        big_list = second_list;
        small_list = first_list;
        ret = Comparison::Sublist;
    }
    let big_len = big_list.len();
    let small_len = small_list.len();
    let diff = big_len - small_len;
    for i in 0..=diff {
        let end = i + small_len;
        if &big_list[i..end] == small_list {
            return ret;
        }
    }    
    return Comparison::Unequal;
}
