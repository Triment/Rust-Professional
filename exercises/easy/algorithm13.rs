/*
    Anagram Check
    Given two strings, check if they are anagrams of each other. 
    Anagrams are words or phrases formed by rearranging the letters of another, 
    using all the original letters exactly once. 
    The strings may contain spaces or punctuation, but you need to ignore them while checking.

    You need to implement the function `are_anagrams(s1: String, s2: String) -> bool`.
    The function should return `true` if the two strings are anagrams, and `false` otherwise.

    Hint: Consider normalizing the strings by removing non-alphabetical characters and converting to lowercase before checking.
*/

use std::{collections::HashMap, fmt::{self, Display, Formatter}};

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // TODO: Implement the logic to check if two strings are anagrams
    //false // Placeholder return value
    let mut s1_hash: HashMap<char, u32> = std::collections::HashMap::new();
    let s1 = s1.to_lowercase();
    for c in s1.chars() {
        if c.is_alphabetic() {
            *s1_hash.entry(c).or_insert(1) += 1;
        }
    }
    let s2 = s2.to_lowercase();
    for c in s2.chars() {
        if c.is_alphabetic() {
            if let Some(val) = s1_hash.get_mut(&c) {
                if *val == 1 {
                    s1_hash.remove(&c);
                } else {
                    *val -= 1;
                    if val == &0 {
                        s1_hash.remove(&c);
                    }
                }
            } else {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("Are anagrams: {}", result);
        assert_eq!(result, true);
    }
}
