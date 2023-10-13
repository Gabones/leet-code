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
}
