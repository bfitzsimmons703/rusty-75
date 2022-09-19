/*
Top K Frequent Elements

Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.

Time: O(n) - need to iterate through array
Space: O(n) - worst case, the hashmap will contain nearly every element in the array
*/

use std::collections::HashMap;

pub fn top_k(arr: &[i32], k: usize) -> Vec<i32> {
    let mut counts = HashMap::<&i32, i32>::new();
    for i in arr {
        counts.entry(i).and_modify(|count| *count += 1).or_insert(1);
    }

    let mut counts = Vec::from_iter(counts);
    counts.sort_unstable_by_key(|(_i, count)| *count);

    let counts: Vec<i32> = counts.into_iter().map(|(_i, count)| count).collect();

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
}
