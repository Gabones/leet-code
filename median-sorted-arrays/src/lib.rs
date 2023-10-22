use std::mem::swap;
use std::cmp;

#[cfg(test)]
mod tests;

pub struct Solution;

pub fn solve(
    a: &Vec<i32>,
    b: &Vec<i32>,
    k: i32,
    a_start: i32,
    a_end: i32,
    b_start: i32,
    b_end: i32,
) -> f64 {
    if a_start > a_end {
        return b[k as usize - a_start as usize] as f64;
    }

    if b_start > b_end {
        return a[k as usize - b_start as usize] as f64;
    }

    let a_index = (a_start + a_end) / 2;
    let b_index = (b_start + b_end) / 2;

    let a_value = a[a_index as usize];
    let b_value = b[b_index as usize];

    if a_index + b_index < k {
        if a_value > b_value {
            return solve(a, b, k, a_start, a_end, b_index + 1, b_end);
        } else {
            return solve(a, b, k, a_index + 1, a_end, b_start, b_end);
        }
    } else {
        if a_value > b_value {
            return solve(a, b, k, a_start, a_index - 1, b_start, b_end);
        } else {
            return solve(a, b, k, a_start, a_end, b_start, b_index - 1);
        }
    }
}

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut p1 = 0;
        let mut p2 = 0;
        let mut cvec = Vec::new();

        while cvec.len() < nums1.len() + nums2.len() {
            if nums1.get(p1).unwrap_or(&i32::MAX) <= nums2.get(p2).unwrap_or(&i32::MAX) {
                cvec.push(nums1[p1]);
                p1 += 1;
            } else {
                cvec.push(nums2[p2]);
                p2 += 1;
            }
        }

        cvec.sort();
        let mid = cvec.len() / 2;
        if cvec.len() % 2 == 0 {
            return (cvec[mid - 1] + cvec[mid]) as f64 / 2.0 as f64;
        } else {
            return cvec[mid] as f64;
        }
    }

    pub fn find_median_sorted_arrays_binary_search(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let na: i32 = nums1.len().try_into().unwrap();
        let nb: i32 = nums2.len().try_into().unwrap();
        let n: i32 = na + nb;

        if n % 2 != 0 {
            return solve(&nums1, &nums2, n / 2, 0, na - 1, 0, nb - 1);
        } else {
            return 1.0
                * (solve(&nums1, &nums2, n / 2 - 1, 0, na - 1, 0, nb - 1)
                    + solve(&nums1, &nums2, n / 2, 0, na - 1, 0, nb - 1))
                / 2.0;
        }
    }

    pub fn find_median_sorted_arrays_better_binary_search(mut nums1: Vec<i32>,mut nums2: Vec<i32>) -> f64 {
        if nums1.len() > nums2.len() {
            swap(&mut nums1, &mut nums2);
        }
        let [m, n] = [nums1.len() as i32, nums2.len() as i32];
        let [mut left, mut right] = [0,m];

        while left <= right {
            let partition_a:i32 = ((left + right)/2).try_into().unwrap();
            let partition_b:i32 = (((m + n + 1) / 2) - partition_a).try_into().unwrap();

            let a_left = if partition_a - 1 < 0 { i32::MIN } else { nums1[(partition_a - 1) as usize] };
            let a_right = if partition_a == m { i32::MAX } else { nums1[partition_a as usize] };
            let b_left = if partition_b == 0 { i32::MIN } else { nums2[(partition_b - 1) as usize] };
            let b_right = if partition_b == n { i32::MAX } else { nums2[partition_b as usize] };

            if a_left <= b_right && b_left <= a_right {
                return if (m+n) % 2 == 0 {
                    (cmp::max(a_left, b_left) + cmp::min(a_right, b_right)) as f64 / 2.0
                } else {
                    cmp::max(a_left, b_left) as f64
                };
            } else if a_left > b_left {
                right = partition_a - 1;
            } else if b_left > a_left {
                left = partition_a + 1;
            }
        }

        0.0
    }
}
