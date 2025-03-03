/*
    Longest Substring Without Repeating Characters
    Given a string, find the length of the longest substring without repeating characters. 
    The substring must not contain any duplicate characters, and its length should be maximized.

    You need to implement the function `longest_substring_without_repeating_chars(s: String) -> i32`.
    The function should return the length of the longest substring without repeating characters.
    
    Hint: Consider using the sliding window technique to efficiently solve this problem in O(n) time complexity.
*/

use std::{collections::HashMap, fmt::{self, Display, Formatter}};

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    // TODO: Implement the logic to find the longest substring without repeating characters
    //0 // Placeholder return value
    let s = s.as_bytes();
    let mut left_idx = 0;
    let mut right_idx = 0;
    let mut max_length = 0;
    let mut char_map:  HashMap<u8, usize> = std::collections::HashMap::new();
    while right_idx < s.len() {
        if let Some(&idx) = char_map.get(&s[right_idx]) {
            left_idx = left_idx.max(idx + 1);
        }
        char_map.insert(s[right_idx], right_idx);
        max_length = max_length.max(right_idx - left_idx + 1);
        right_idx += 1;
    }
    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1);  // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3);  // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0);  // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5);  // "abcde"
    }
}
