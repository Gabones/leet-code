use std::cmp;

#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area_max = 0;
        for i in 0..height.len() {
            for j in (i + 1)..height.len() {
                let curr_area = cmp::min(height[i], height[j]) * (j as i32 - i as i32);
                area_max = cmp::max(curr_area, area_max);
            }
        }

        println!("{area_max}");

        area_max
    }
}
