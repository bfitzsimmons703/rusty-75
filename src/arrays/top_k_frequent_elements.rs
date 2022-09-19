/*
Top K Frequent Elements

Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

Time: O(n + (m * n * log(n)) - need to iterate through array, get the counts, and sort them by the count value
Space: O(n) - we're holding every value in a hashmap so we can count their frequency
*/

use std::{cmp::Reverse, collections::HashMap};

pub fn top_k(arr: &[i32], k: usize) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }

    let mut counts = HashMap::<&i32, i32>::new();
    for i in arr {
        counts.entry(i).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut counts = Vec::from_iter(counts);
    counts.sort_unstable_by_key(|(_, count)| Reverse(*count));

    let counts: Vec<i32> = counts.into_iter().map(|(val, _)| *val).collect();

    counts[..k].to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input_arr = [1, 1, 1, 2, 2, 3];
        let k = 2;
        let output = top_k(&input_arr, k);

        assert_eq!(output, [1, 2]);
    }

    #[test]
    fn test_2() {
        let input_arr = [1];
        let k = 1;
        let output = top_k(&input_arr, k);

        assert_eq!(output, [1]);
    }

    #[test]
    fn test_3() {
        let input_arr = [1, 1, 4, 3, 6, 22, 4, 4, 4, 22, 1];
        let k = 3;
        let output = top_k(&input_arr, k);

        assert_eq!(output, [4, 1, 22]);
    }

    #[test]
    fn test_4() {
        let input_arr = [];
        let k = 1000;
        let output = top_k(&input_arr, k);

        assert_eq!(output, []);
    }
}
