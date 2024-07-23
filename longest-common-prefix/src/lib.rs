#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    // Approach 0: My Own Solution
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        fn common_prefix_length(s1: &str, s2: &str) -> i32 {
            let mut common_length = 0;

            for (c1, c2) in s1.chars().zip(s2.chars()) {
                if c1 != c2 {
                    break;
                }

                common_length += 1;
            }

            common_length
        }

        if strs.len() == 1 {
            return String::from(&strs[0]);
        }

        let mut min = i32::MAX;

        for i in 0..strs.len() - 1 {
            let common = common_prefix_length(&strs[i], &strs[i + 1]);
            min = std::cmp::min(min, common);
        }

        let min: usize = min.try_into().unwrap();

        String::from(&(strs[0])[..min])
    }

    // Approach 1: Horizontal scanning
    pub fn longest_common_prefix_horizontal(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }

        let mut prefix = strs[0].to_string();
        for i in 0..strs.len() {
            while strs[i].find(&*prefix) != None {
                prefix = (&prefix[..prefix.len() - 1]).to_string();
                if prefix.is_empty() {
                    return String::from("");
                }
            }
        }

        prefix.to_string()
    }

    // Approach 2: Vertical scanning
    pub fn longest_common_prefix_vertical(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }

        for i in 0..=strs[0].len() {
            let c = strs[0].chars().nth(i);
            for j in 0..strs.len() {
                if i == strs[j].len() || strs[j].chars().nth(i) != c {
                    return strs[0][0..i].to_string();
                }
            }
        }

        strs[0].to_string()
    }

    // Approach 3: Divide and Conquer
    pub fn longest_common_prefix_divide_and_conquer(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }

        let size: i32 = (strs.len() - 1) as i32;

        Self::longest_common_prefix_divide(strs, 0, size)
    }

    // Approach 3: Divide and Conquer
    fn longest_common_prefix_divide(strs: Vec<String>, l: i32, r: i32) -> String {
        if l == r {
            return strs[l as usize].clone();
        } else {
            let mid = (l + r) / 2;
            let lcp_left = Self::longest_common_prefix_divide(strs.clone(), l, mid);
            let lcp_right = Self::longest_common_prefix_divide(strs, mid + 1, r);
            return Self::common_prefix(lcp_left, lcp_right);
        }
    }

    // Approach 3: Divide and Conquer
    fn common_prefix(left: String, right: String) -> String {
        let min = std::cmp::min(left.len(), right.len());
        for i in 0..min {
            if left.chars().nth(i) != right.chars().nth(i) {
                return left[..i].to_string();
            }
        }
        left[..min].to_string()
    }

    // Approach 4: Binary Search
    pub fn longest_common_prefix_binary_search(strs: Vec<String>) -> String {
        fn is_common_prefix(strs: Vec<String>, length: usize) -> bool {
            let str1 = strs[0][0..length].to_string();
            for str in strs {
                if str.find(&str1).is_some() {
                    return false;
                }
            }
            true
        }

        if strs.len() == 0 {
            return String::from("");
        }
        let min_len: usize = strs.iter().min_by_key(|s| s.len()).unwrap().len();
        let mut low: usize = 1;
        let mut high: usize = min_len;
        while low <= high {
            let middle: usize = (low + high) / 2;
            if is_common_prefix(strs.clone(), middle) {
                low = middle + 1;
            } else {
                high = middle - 1;
            }
        }

        strs[0][0..((low + high) / 2)].to_string()
    }
}
