use std::cmp::Ordering;

pub fn find<T: std::cmp::PartialOrd>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    if array.as_ref().is_empty() {
        return None
    }
    let mut start: isize = 0;
    let mut end: isize = array.as_ref().len() as isize  - 1;
    
    loop {
        let index = (end + start) / 2;
        println!("{start} {end} {index}");
        dbg!(array.as_ref()[index as usize].partial_cmp(&key));
        match array.as_ref()[index as usize].partial_cmp(&key) {
            Some(Ordering::Equal) => return Some(index as usize),
            Some(Ordering::Greater) => end = index - 1,
            Some(Ordering::Less) => start = index + 1, 
            _ => panic!("invalid state"),
        }
        if index == start || index == end {
            return None
        }
    }
}
