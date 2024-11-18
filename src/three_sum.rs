pub struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut j, mut k) = (i + 1, nums.len() - 1);

            while j < k {
                let three_sum = nums[i] + nums[k] + nums[j];
                match three_sum.cmp(&0) {
                    std::cmp::Ordering::Greater => k -= 1,
                    std::cmp::Ordering::Less => j += 1,
                    _ => {
                        result.push(vec![nums[i], nums[j], nums[k]]);
                        j += 1;
                        while nums[j] == nums[j - 1] && j < k {
                            j += 1;
                        }
                    }
                }
            }
        }

        result
    }
}
