/*
Contains Duplicate

Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
*/

use std::collections::HashMap;

/*
Time: O(n) - need to iterate through array
Space: O(n) - worst case, the hashmap will contain every element in the array if they're all unique
*/
pub fn contains_dupes_hashmap(arr: &[i32]) -> bool {
    if arr.len() <= 1 {
        return false;
    }

    let mut counts = HashMap::new();
    for i in arr.iter() {
        if counts.contains_key(i) {
            return true;
        }

        counts.insert(i, true);
    }

    false
}

/*
Time: O(nlogn + n)
Space: O(1) - don't need to hold a hasmap since we're sorting now
*/
pub fn contains_dupes_sort(arr: &mut [i32]) -> bool {
    if arr.len() <= 1 {
        return false;
    }

    arr.sort_unstable();

    let mut prev = match arr.first() {
        Some(n) => *n,
        None => {
            return false;
        }
    };

    for curr in &arr[1..] {
        if *curr == prev {
            return true;
        }

        prev = *curr;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let mut input_arr = [2, 7, 11, 15];
        let output = contains_dupes_hashmap(&input_arr);
        assert!(!output);

        let output = contains_dupes_sort(&mut input_arr);
        assert!(!output);
    }

    #[test]
    fn test_2() {
        let mut input_arr = [3, 2, 4, 5, 3, 5];
        let output = contains_dupes_hashmap(&input_arr);
        assert!(output);

        let output = contains_dupes_sort(&mut input_arr);
        assert!(output);
    }

    #[test]
    fn test_3() {
        let mut input_arr = [3, 3];
        let output = contains_dupes_hashmap(&input_arr);
        assert!(output);

        let output = contains_dupes_sort(&mut input_arr);
        assert!(output);
    }

    #[test]
    fn test_4() {
        let mut input_arr = [];
        let output = contains_dupes_hashmap(&input_arr);
        assert!(!output);

        let output = contains_dupes_sort(&mut input_arr);
        assert!(!output);
    }

    #[test]
    fn test_5() {
        let mut input_arr = [1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        let output = contains_dupes_hashmap(&input_arr);
        assert!(output);

        let output = contains_dupes_sort(&mut input_arr);
        assert!(output);
    }
}
