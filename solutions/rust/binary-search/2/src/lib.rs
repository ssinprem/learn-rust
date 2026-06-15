use std::cmp::Ordering;

pub fn find<T: std::cmp::PartialOrd>(array: impl AsRef<[T]>, key: T) -> Option<usize> {
    if array.as_ref().is_empty() {
        return None
    }
    let mut start: isize = 0;
    let mut end: isize = array.as_ref().len() as isize;
    
    while start < end {
        let index = start + (end - start) / 2;
        match array.as_ref()[index as usize].partial_cmp(&key) {
            Some(Ordering::Equal) => return Some(index as usize),
            Some(Ordering::Less) => start = index + 1, 
            Some(Ordering::Greater) => end = index,
            _ => panic!("invalid state"),
        }
    }
    None
}
