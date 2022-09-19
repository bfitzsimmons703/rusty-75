/*
Two Sum

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order.

Time: O(n) - need to iterate through array
Space: O(n) - worst case, the hashmap will contain nearly every element in the array
*/

use std::collections::HashMap;

pub fn two_sum(arr: &[i32], target: i32) -> Vec<usize> {
    let mut map = HashMap::<i32, usize>::new();
    for (index, val) in arr.iter().enumerate() {
        let diff = target - *val;
        if let Some(other_index) = map.get(&diff) {
            return vec![*other_index, index];
        }

        map.insert(*val, index);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input_arr = [2, 7, 11, 15];
        let target = 9;
        let output = two_sum(&input_arr, target);

        assert_eq!(output, [0, 1]);
    }

    #[test]
    fn test_2() {
        let input_arr = [3, 2, 4];
        let target = 6;
        let output = two_sum(&input_arr, target);

        assert_eq!(output, [1, 2]);
    }

    #[test]
    fn test_3() {
        let input_arr = [3, 3];
        let target = 6;
        let output = two_sum(&input_arr, target);

        assert_eq!(output, [0, 1]);
    }
}
