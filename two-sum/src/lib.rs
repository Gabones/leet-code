use std::collections::HashMap;

#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();
        for i in 0..nums.len() {
            hashmap.insert(nums[i].to_string(), i);
        }

        for i in 0..nums.len() {
            let complement = target - nums[i];
            if hashmap.contains_key(&complement.to_string())
                && hashmap[&complement.to_string()] != i
            {
                return Vec::from([i as i32, hashmap[&complement.to_string()] as i32]);
            }
        }

        return Vec::new();
    }
}
