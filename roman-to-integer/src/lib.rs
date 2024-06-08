#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        fn matcher(r: Option<char>) -> i32 {
            match r {
                Some('I') => 1,
                Some('V') => 5,
                Some('X') => 10,
                Some('L') => 50,
                Some('C') => 100,
                Some('D') => 500,
                Some('M') => 1000,
                _ => -1,
            }
        }

        let word = s;
        let mut counter = 0;
        let mut jump = false;

        for i in 0..word.len() {
            if jump {
                jump = false;
                continue;
            }

            let s1 = matcher(word.chars().nth(i));

            if i + 1 < word.len() {
                let s2 = matcher(word.chars().nth(i as usize + 1));
                if s1 >= s2 {
                    counter += s1;
                } else {
                    counter += s2 - s1;
                    jump = true;
                }
            } else {
                counter += s1;
            }
        }

        counter
    }
}
