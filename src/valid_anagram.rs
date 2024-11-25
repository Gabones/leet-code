use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() == t.len() {
            let mut counts: HashMap<char, i32> = HashMap::new();

            for (a, b) in s.chars().zip(t.chars()) {
                *counts.entry(a).or_insert(0) += 1;
                *counts.entry(b).or_insert(0) -= 1;
            }

            return !counts.values().any(|&value| value != 0);
        }

        false
    }
}
