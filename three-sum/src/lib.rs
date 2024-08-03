#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut i = 0;
        let mut k = nums.len() - 1;
        let mut last_up = false;
        let mut result: Vec<Vec<i32>> = Vec::new();

        while i < nums.len() && k > 0 {
            for j in 0..(nums.len() - 1) {
                if i == 2 {
                    println!("{:?}", vec![nums[i], nums[j], nums[k]]);
                }
                if j != i && j != k && k != i {
                    let mut t = vec![nums[i], nums[j], nums[k]];
                    t.sort();
                    if nums[i] + nums[k] + nums[j] == 0 && !result.contains(&t) {
                        result.push(t);
                    }
                }
            }

            if last_up {
                i += 1;
                last_up = false;
            } else {
                k -= 1;
                last_up = true;
            }
        }

        result.sort();

        return result;
    }
}
