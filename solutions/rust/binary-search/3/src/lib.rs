use std::cmp::Ordering;

pub fn find<T: std::cmp::PartialOrd>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    if array.as_ref().is_empty() {
        return None
    }
    let mut start = 0;
    let mut end = array.as_ref().len() ;
    
    while start < end {
        let index = start + (end - start) / 2;
        match array.as_ref()[index].partial_cmp(&key) {
            Some(Ordering::Equal) => return Some(index),
            Some(Ordering::Less) => start = index + 1, 
            Some(Ordering::Greater) => end = index,
            _ => panic!("invalid state"),
        }
    }
    None
}
