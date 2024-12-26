use std::cmp;

pub struct Solution;

impl Solution {
    // Approach 1 - Brute Force
    pub fn max_area_brute_force(height: Vec<i32>) -> i32 {
        let mut area_max = 0;

        for i in 0..height.len() {
            for j in (i + 1)..height.len() {
                let curr_area = cmp::min(height[i], height[j]) * (j as i32 - i as i32);
                area_max = cmp::max(curr_area, area_max);
            }
        }

        area_max
    }

    // Approach 2 - Two Pointers
    pub fn max_area(height: Vec<i32>) -> i32 {
        let (mut area_max, mut left, mut right) = (0, 0, height.len() - 1);

        while left < right {
            let min = cmp::min(height[left], height[right]);
            let curr_area = min * (right - left) as i32;
            area_max = cmp::max(curr_area, area_max);

            if min == height[left] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        area_max
    }
}
