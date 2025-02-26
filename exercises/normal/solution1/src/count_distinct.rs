pub fn new_count_distinct(input_str: &str) -> usize {
    let hash = input_str.chars().fold(0, |acc, c| acc | 1 << (c as u8 - b'a'));
    hash.count_ones() as usize
}
