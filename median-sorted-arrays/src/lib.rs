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
}
