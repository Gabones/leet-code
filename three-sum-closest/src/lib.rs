use std::i32;

#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut closest = 0;

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut j, mut k) = (i + 1, nums.len() - 1);

            while j < k {
                let three_sum = nums[i] + nums[k] + nums[j];
                if target - three_sum == 0 {
                    return three_sum;
                } else if (target - three_sum).abs() < (target - closest).abs() {
                    closest = three_sum;
                    j += 1;
                    while nums[j] == nums[j - 1] && j < k {
                        j += 1;
                    }
                } else if target - three_sum < 0 {
                    j += 1;
                } else {
                    k -= 1;
                }
            }
        }

        return closest;
    }
}
