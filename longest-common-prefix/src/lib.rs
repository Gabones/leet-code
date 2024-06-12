#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn longest_common_prefix_horizontal(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::from("");
        }

        let mut prefix = strs[0].to_string();
        for i in 0..strs.len() {
            while strs[i].find(&*prefix) != None {
                prefix = (&prefix[..prefix.len()-1]).to_string();
                if prefix.is_empty() {
                    return String::from("");
                }
            }
        }

        prefix.to_string()
    }


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
}