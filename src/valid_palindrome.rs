use utils::string_utils::remove_non_alphanumeric;

pub struct Solution;

impl Solution {
    // Approach 1
    pub fn is_palindrome_str_clone(s: String) -> bool {
        let cleaned = remove_non_alphanumeric(&s.to_lowercase());
        cleaned.chars().rev().collect::<String>() == cleaned
    }

    pub fn is_palindrome(s: String) -> bool {
        let (mut left, mut right) = (0, s.len() - 1);
        while left < right {
            if s.chars().nth(left).expect("left unwrap fail").is_alphanumeric() && s.chars().nth(right).expect("right unwrap fail").is_alphanumeric() {
                if s.chars().nth(left) == s.chars().nth(right) {
                    left += 1;
                    right -= 1;
                } else {
                   return false;
                }
            }

            if !s.chars().nth(left).expect("left unwrap fail").is_alphanumeric() {
                left += 1;
            }

            if !s.chars().nth(right).expect("right unwrap fail").is_alphanumeric() {
                right -= 1;
            }
        }

        true
    }
}
