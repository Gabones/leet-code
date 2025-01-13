use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut low, mut high) = (0 as i32, (nums.len() - 1) as i32);

        while low <= high {
            let middle = low + (high - low) / 2;

            match nums[middle as usize].cmp(&target) {
                Ordering::Less => low = middle + 1,
                Ordering::Equal => return middle as i32,
                Ordering::Greater => high = middle - 1,
            }
        }

        -1
    }
}
