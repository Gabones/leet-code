use std::collections::HashSet;
use std::cmp::max;

pub struct Solution;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
       let nums_set:HashSet<&i32> = HashSet::from_iter(nums.iter());
       let mut longest = 0;

       for n in &nums_set {
            if !nums_set.contains(&(*n-1)) {
                let mut len = 1;
                while nums_set.contains(&(*n + len)){
                    len += 1;
                }
                longest = max(len, longest);
            }
       }

       longest
    }
}
