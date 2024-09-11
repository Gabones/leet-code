#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        
        let mut prefix = 1;
        for x in 0..nums.len() {
            result[x] = prefix;
            prefix *= nums[x];
        }

        let mut postfix = 1;
        for y in (0..nums.len()).rev() {
            result[y] *= postfix;
            postfix *= nums[y];
        }

        result
    }

    pub fn product_except_self_on(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        let mut left_result = vec![0; nums.len()];
        for x in 1..nums.len() {
            if x == 1 {
                left_result[x] = nums[x - 1];
            } else {
                left_result[x] = nums[x - 1] * left_result[x - 1];
            }
        }

        let mut right_result = vec![0; nums.len()];
        for y in (0..nums.len() - 1).rev() {
            if y == nums.len() - 2 {
                right_result[y] = nums[y + 1];
            } else {
                right_result[y] = nums[y + 1] * right_result[y + 1];
            }
        }

        for z in 0..nums.len() {
            if z == 0 {
                result[0] = right_result[0];
            } else if z == nums.len() - 1 {
                result[z] = left_result[z];
            } else {
                result[z] = right_result[z] * left_result[z];
            }
        }

        result
    }

    pub fn product_except_self_on2(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![0; nums.len()];
        for x in 0..nums.len() {
            let mut product = 0;
            let mut init = false;

            for y in 0..nums.len() {
                if x != y && !init {
                    init = true;
                    product = nums[y];
                } else if x != y && init {
                    product = product * nums[y];
                }
            }

            result[x] = product;
        }

        result
    }
}
