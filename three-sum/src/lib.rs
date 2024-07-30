#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut p_1 = 0;
        let mut p_2 = nums.len() - 1;
        let mut result: Vec<Vec<i32>> = Vec::new();
        let mut last_up = true;

        while p_1 < p_2 {
            for (idx, _) in nums.iter().enumerate() {
                if idx != p_1 && idx != p_2 {
                    println!(
                        "p_1: {:?} idx: {:?} p_2: {:?}",
                        nums[p_1], nums[idx], nums[p_2]
                    );
                    if nums[p_1] + nums[p_2] + nums[idx] == 0 {
                        let mut new_vec = vec![nums[p_1], nums[p_2], nums[idx]];
                        new_vec.sort();
                        result.push(new_vec);
                    }
                }
            }

            if last_up {
                p_1 += 1;
                last_up = false;
            } else {
                p_2 -= 1;
                last_up = true;
            }
        }

        return result;
    }
}
