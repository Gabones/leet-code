pub struct Solution;

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let (mut left, mut right) = (0 as usize, numbers.len() - 1);

        while left < right {
            if numbers[left] + numbers[right] > target {
                right -= 1;
            } else if numbers[left] + numbers[right] < target {
                left += 1;
            } else {
                return vec![(left + 1) as i32, (right + 1) as i32];
            }
        }

        vec![(left + 1) as i32, (right + 1) as i32]
    }
}
