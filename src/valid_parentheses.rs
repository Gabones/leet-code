use std::{char, collections::HashMap};

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        let map: HashMap<char, char> = HashMap::from([
            (')', '('),
            (']', '['),
            ('}', '{')
        ]);

        for ch in s.chars().into_iter() {
            if map.contains_key(&ch) {
               if stack.len() > 0 && stack.last().unwrap() == map.get(&ch).unwrap() {
                   stack.pop();
               } else {
                   return false;
               }
            } else {
                stack.push(ch);
            }
        }

        stack.len() == 0
    }
}
