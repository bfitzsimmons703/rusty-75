/*
Group Anagrams

Given an array of strings strs, group the anagrams together. You can return the answer in any order.
An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase, typically using all the original letters exactly once.

Time: O(m*n) - m is average length of string, n is number of words
Space: O(m*n) - need to store every word in hashmap
*/

use std::collections::HashMap;

pub fn group_anagrams<'a>(words: &'a [&str]) -> Vec<Vec<&'a str>> {
    let mut result = HashMap::new(); //mapping of char_counts to list of anagrams

    for word in words {
        let mut char_counts = [0_u8; 26]; //array of byte values for word, which we'll use as the result key

        for &byte in word.as_bytes() {
            char_counts[(byte - b'a') as usize] += 1;
        }

        result
            .entry(char_counts)
            .or_insert_with(Vec::new)
            .push(*word);
    }

    result.into_iter().map(|(_k, v)| v).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = ["eat", "tea", "tan", "ate", "nat", "bat"];
        let mut output: Vec<Vec<&str>> = group_anagrams(&input);
        output.sort_by_key(|a| a.len());

        let expected = vec![vec!["bat"], vec!["tan", "nat"], vec!["eat", "tea", "ate"]];

        assert_eq!(output, expected);
    }

    #[test]
    fn test_2() {
        let input = [""];
        let output: Vec<Vec<&str>> = group_anagrams(&input);
        assert_eq!(output, vec![vec![""]]);
    }

    #[test]
    fn test_3() {
        let input = ["a"];
        let output: Vec<Vec<&str>> = group_anagrams(&input);
        assert_eq!(output, vec![vec!["a"]]);
    }
}
