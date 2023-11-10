#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn longest_palindrome_check_all(s: String) -> String {
        pub fn check(s: &String, i: usize, j: usize) -> bool {
            let [mut left, mut right] = [i, j - 1];
            while left < right {
                if s.as_bytes()[left] != s.as_bytes()[right] {
                    return false;
                }
                left += 1;
                right -= 1;
            }
            return true;
        }

        for length in (0..s.len()).rev() {
            for start in 0..s.len() - length + 1 {
                if check(&s, start, start + length) {
                    return s[start..start + length].to_string();
                }
            }
        }
        return String::from("");
    }

    pub fn longest_palindrome_dynamic(s: String) -> String {
        let n = s.len();
        let mut dp = vec![vec![false; n]; n];
        let mut ans = [0, 0];

        for i in 0..n {
            dp[i][i] = true;
        }

        for i in 0..(n - 1) {
            if s.as_bytes()[i] == s.as_bytes()[i + 1] {
                dp[i][i + 1] = true;
                ans = [i, i + 1]
            }
        }

        for diff in 2..n {
            for i in 0..(n - diff) {
                let j = i + diff;
                if s.as_bytes()[i] == s.as_bytes()[j] && dp[i + 1][j - 1] {
                    dp[i][j] = true;
                    ans = [i, j]
                }
            }
        }

        s[ans[0]..(ans[1] + 1)].to_string()
    }
}
