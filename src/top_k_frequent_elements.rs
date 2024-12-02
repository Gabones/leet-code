use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for num in nums {
            *counts.entry(num).or_insert(0) += 1;
        }

        let mut result: Vec<(i32, i32)> = counts.into_iter().collect();
        result.sort_by_key(|t| t.1);
        result.reverse();
        result[0..(k as usize)]
            .iter()
            .map(|t| t.0)
            .collect::<Vec<i32>>()
    }
}
