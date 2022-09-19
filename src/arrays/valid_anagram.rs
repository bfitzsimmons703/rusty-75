/*
Valid Anagram

Given two strings s and t, return true if t is an anagram of s, and false otherwise.
An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.
*/

use std::collections::HashMap;

/*
Time: O((n + nlogn) + (m + mlogm)) - sorting the two strings then comparing them
Space: O(n + m) - we have to turn the string into vecs to sort them
*/
pub fn valid_anagram_sort(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut s1_chars: Vec<char> = s1.chars().collect();
    let mut s2_chars: Vec<char> = s2.chars().collect();

    s1_chars.sort_unstable();
    s2_chars.sort_unstable();

    let s1: String = s1_chars.into_iter().collect();
    let s2: String = s2_chars.into_iter().collect();

    s1 == s2
}

/*
Time: O(n + m) - need to iterate over each string to get the character counts
Space: O(n + m) - we have to store the counts of characters in each string
*/
pub fn valid_anagram_hashmap(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let s1_char_counts = build_char_count_map(s1);
    let s2_char_counts = build_char_count_map(s2);

    for (char, count) in s1_char_counts.into_iter() {
        if let Some(s2_count) = s2_char_counts.get(&char) {
            if *s2_count != count {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}

fn build_char_count_map(s: &str) -> HashMap<char, i32> {
    let mut char_counts = HashMap::new();
    for char in s.chars() {
        char_counts
            .entry(char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    char_counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s1 = "anagram";
        let s2 = "nagaram";

        assert!(valid_anagram_sort(s1, s2));
        assert!(valid_anagram_hashmap(s1, s2));
    }

    #[test]
    fn test_2() {
        let s1 = "rat";
        let s2 = "car";

        assert!(!valid_anagram_sort(s1, s2));
        assert!(!valid_anagram_hashmap(s1, s2));
    }

    #[test]
    fn test_3() {
        let s1 = "aaa";
        let s2 = "aaaa";

        assert!(!valid_anagram_sort(s1, s2));
        assert!(!valid_anagram_hashmap(s1, s2));
    }

    #[test]
    fn test_4() {
        let s1 = "";
        let s2 = "";

        assert!(valid_anagram_sort(s1, s2));
        assert!(valid_anagram_hashmap(s1, s2));
    }
}
