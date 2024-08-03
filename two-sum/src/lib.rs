use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    // Approach 1: Brute Force
    pub fn two_sum_brute_force(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for x in 0..nums.len() {
            for y in x + 1..nums.len() {
                if nums[x] + nums[y] == target {
                    return Vec::from([x as i32, y as i32]);
                }
            }
        }

        return Vec::from([0, 0]);
    }

    // Approach 2: Two-pass Hash Table
    pub fn two_sum_two_pass_hashmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();
        for (index, value) in nums.iter().enumerate() {
            hashmap.insert(value, index);
        }

        for (index, value) in nums.iter().enumerate() {
            let complement = target - value;
            if hashmap.contains_key(&complement) && hashmap[&complement] != index {
                return Vec::from([index as i32, hashmap[&complement] as i32]);
            }
        }

        return Vec::new();
    }

    // Approach 3: One-pass Hash Table
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();

        for (index, value) in nums.iter().enumerate() {
            let complement = target - value;
            if hashmap.contains_key(&complement) {
                return Vec::from([hashmap[&complement] as i32, index as i32]);
            } else {
                hashmap.insert(value, index);
            }
        }

        return Vec::new();
    }
}
