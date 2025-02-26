pub fn new_count_distinct(input_str: &str) -> usize {
    let words: Vec<&str> = input_str.split(',').collect();
    let mut distinct_words: Vec<&str> = Vec::new();
    for word in words {
        if !distinct_words.contains(&word) {
            distinct_words.push(word);
        }
    }
    distinct_words.len()
}
