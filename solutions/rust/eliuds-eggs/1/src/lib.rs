pub fn egg_count(display_value: u32) -> usize {
    display_value.count_ones() as usize
}
