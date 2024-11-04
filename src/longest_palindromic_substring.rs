use std::fmt::Write;

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

            true
        }

        for length in (0..s.len() + 1).rev() {
            for start in 0..s.len() - length + 1 {
                if check(&s, start, start + length) {
                    return s[start..start + length].to_string();
                }
            }
        }

        String::from("")
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

    pub fn longest_palindrome_center_expand(s: String) -> String {
        pub fn expand(s: &String, i: i32, j: i32) -> usize {
            let [mut left, mut right] = [i, j];
            while left >= 0
                && right < s.len() as i32
                && s.as_bytes()[left as usize] == s.as_bytes()[right as usize]
            {
                right += 1;
                left -= 1;
            }

            (right - left - 1).try_into().unwrap()
        }

        let mut ans = [0, 0];
        for i in 0..s.len() {
            let odd_lenght = expand(&s, i as i32, i as i32);
            if odd_lenght > ans[1] - ans[0] + 1 {
                let dist = odd_lenght / 2;
                ans = [i - dist, i + dist];
            }

            let even_length = expand(&s, i as i32, i as i32 + 1);
            if even_length > ans[1] - ans[0] + 1 {
                let dist = (even_length / 2) - 1;
                ans = [i - dist, i + 1 + dist];
            }
        }

        s[ans[0]..ans[1] + 1].to_string()
    }

    pub fn longest_palindrome_manachers(s: String) -> String {
        use std::cmp;
        let old_s = s.chars().collect::<String>();
        let s: String = s.chars().fold(String::new(), |mut output, c: char| {
            let _ = write!(output, "#{}", c);
            output
        }) + "#";
        let n: usize = s.len();
        let mut vec_radi: Vec<i32> = vec![0; n];
        let [mut center, mut radius] = [0, 0];

        for i in 0..n as i32 {
            let mirror = 2 * center - i;

            if i < radius {
                vec_radi[i as usize] = cmp::min(radius - i, vec_radi[mirror as usize]);
            }

            while i + 1 + vec_radi[i as usize] < n.try_into().unwrap()
                && i - 1 - vec_radi[i as usize] >= 0
                && s.as_bytes()[(i + 1 + vec_radi[i as usize]) as usize]
                    == s.as_bytes()[(i - 1 - vec_radi[i as usize]) as usize]
            {
                vec_radi[i as usize] += 1
            }

            if i + vec_radi[i as usize] > radius {
                center = i;
                radius = i + vec_radi[i as usize]
            }
        }

        let max_length = vec_radi.iter().max().unwrap();
        let center_index = vec_radi.iter().position(|&r| r == *max_length).unwrap();
        let start_index = (center_index - *max_length as usize) / 2;

        old_s[start_index..start_index + *max_length as usize].to_string()
    }
}
