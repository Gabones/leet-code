use std::cmp;
use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        if s.is_empty() {
            return 0;
        }

        let mut biggest: i32 = 0;
        let vec_char: Vec<char> = s.chars().collect();
        let mut curr_hash = HashSet::new();
        let mut i = 0;
        
        while i < s.len() {
            match curr_hash.get(&vec_char[i]) {
                None => {
                    curr_hash.insert(vec_char[i]);
                }
                _ => {
                    let index = vec_char[0..i]
                        .iter()
                        .rposition(|&r| r == vec_char[i])
                        .unwrap_or(i - 1);

                    i = index + 1;
                    biggest = cmp::max(biggest, curr_hash.len() as i32);
                    curr_hash = HashSet::new();
                    curr_hash.insert(vec_char[i]);
                }
            }

            i += 1;
        }

        cmp::max(biggest, curr_hash.len() as i32)
    }
}
