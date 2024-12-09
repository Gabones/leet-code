use utils::string_utils::remove_non_alphanumeric;

pub struct Solution;

impl Solution {
    // Approach 1
    pub fn is_palindrome_str_clone(s: String) -> bool {
        let cleaned = remove_non_alphanumeric(&s.to_lowercase());
        cleaned.chars().rev().collect::<String>() == cleaned
    }

    // Approach 2 - Two Pointers
    pub fn is_palindrome(s: String) -> bool {
        let (mut left, mut right) = (0, s.len() - 1);
        let s_vec: Vec<char> = s.chars().collect();
        while left < right {
            while left < right && !s_vec[left].is_alphanumeric() {
                left += 1;
            }

            while left < right && !s_vec[right].is_alphanumeric() {
                right = right.saturating_sub(1);
            }

            if s_vec[left].to_ascii_lowercase() != s_vec[right].to_ascii_lowercase() {
                return false;
            }
            left += 1;
            right = right.saturating_sub(1);
        }

        true
    }
}
