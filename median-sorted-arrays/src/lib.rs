#[cfg(test)]
mod tests;

pub struct Solution;

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
        let na = nums1.len();
        let nb = nums2.len();
        let n = na + nb;

        let [mut a_start, mut a_end] = [0, na];
        let [mut b_start, mut b_end] = [0, nb];

        let k = n / 2;

        let result = loop {
            if a_start > a_end {
                let i = k - a_start;
                break nums2[i] as f64;
            }

            if b_start > b_end {
                let i = k - b_start;
                break nums1[i] as f64;
            }

            let a_index = (a_start + a_end) / 2;
            let b_index = (b_start + b_end) / 2;

            let a_value = nums1[a_index];
            let b_value = nums2[b_index];

            if a_index + b_index < k {
                if a_value > b_value {
                    b_start = b_index + 1;
                } else {
                    a_start = a_index + 1;
                }
            } else {
                if a_value > b_value {
                    a_end = a_index - 1;
                } else {
                    b_end = b_index - 1;
                }
            }
        };

        result
    }
}
