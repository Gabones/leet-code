use std::cmp::Ordering;
use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // Approach 1 - Brute Force
    pub fn two_sum_brute_force(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            for j in i + 1..numbers.len() {
                if numbers[i] + numbers[j] == target {
                    return vec![(i + 1) as i32, (j + 1) as i32];
                }
            }
        }
        Vec::new()
    }

    // Approach 2 - Binary Search
    pub fn two_sum_binary_search(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..numbers.len() {
            let (mut left, mut right) = (i + 1, numbers.len() - 1);
            let tmp = target - numbers[i];

            while left <= right {
                let middle = left + (right - left) / 2;
                match (numbers[middle]).cmp(&tmp) {
                    Ordering::Greater => right = middle - 1,
                    Ordering::Less => left = middle + 1,
                    Ordering::Equal => return vec![(i + 1) as i32, (middle + 1) as i32],
                }
            }
        }
        Vec::new()
    }

    // Approach 3 - Hashing
    pub fn two_sum_hash(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in 0..numbers.len() {
            let tmp = target - numbers[i];
            if map.contains_key(&tmp) {
                return vec![map[&tmp], (i + 1) as i32];
            }
            map.insert(numbers[i], (i + 1) as i32);
        }
        Vec::new()
    }

    // Approach 4 - Two Pointers
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0 as usize, numbers.len() - 1);

        while left < right {
            match (numbers[left] + numbers[right]).cmp(&target) {
                Ordering::Greater => right -= 1,
                Ordering::Less => left += 1,
                Ordering::Equal => return vec![(left + 1) as i32, (right + 1) as i32],
            }
        }

        vec![(left + 1) as i32, (right + 1) as i32]
    }
}
