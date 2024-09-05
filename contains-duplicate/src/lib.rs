use std::collections::HashSet;

#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut counts: HashSet<i32> = HashSet::new();

        for item in nums {
            if counts.contains(&item) {
                return true;
            } else {
                counts.insert(item);
            }
        }

        return false;
    }
}
