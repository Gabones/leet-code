use std::cmp::Ordering;

pub struct Solution;

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let flatten_matrix = matrix.into_iter().flatten().collect::<Vec<i32>>();

        let (mut low, mut high) = (0 as i32, (flatten_matrix.len() - 1) as i32);

        while low <= high {
            let middle = low + (high - low) / 2;

            match flatten_matrix[middle as usize].cmp(&target) {
                Ordering::Less => low = middle + 1,
                Ordering::Equal => return true,
                Ordering::Greater => high = middle - 1,
            }
        }

        false
    }
}
