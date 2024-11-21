use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: HashMap<Vec<i32>, Vec<String>> = HashMap::new();

        for word in strs {
            let mut counts = vec![0; 26];
            for c in word.chars() {
                counts[c as usize - 'a' as usize] += 1;
            }

            result.entry(counts).or_default().push(word);
        }

        result.into_values().collect()
    }
}
