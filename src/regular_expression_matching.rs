use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn is_match(text: String, pattern: String) -> bool {
        fn dfs(
            i: usize,
            j: usize,
            t: &[u8],
            p: &[u8],
            cache: &mut HashMap<(usize, usize), bool>,
        ) -> bool {
            if let Some(&v) = cache.get(&(i, j)) {
                return v;
            }

            if p.is_empty() {
                return t.is_empty();
            }

            if i >= t.len() && j >= p.len() {
                return true;
            }

            if j >= p.len() {
                return false;
            }

            let first_match = i < t.len() && (p[j] == b'.' || p[j] == t[i]);

            if (j + 1) < p.len() && p[j + 1] == b'*' {
                let result =
                    (dfs(i, j + 2, t, p, cache)) || (first_match && dfs(i + 1, j, t, p, cache));
                let _ = &cache.insert((i, j), result);
                result
            } else {
                let result = first_match && dfs(i + 1, j + 1, t, p, cache);
                let _ = &cache.insert((i, j), result);
                result
            }
        }

        let mut cache: HashMap<(usize, usize), bool> = HashMap::new();

        return dfs(0, 0, text.as_bytes(), pattern.as_bytes(), &mut cache);
    }

    pub fn is_match_brute(text: String, pattern: String) -> bool {
        if pattern.is_empty() {
            return text.is_empty();
        }

        let first_match = !text.is_empty()
            && (pattern.as_bytes()[0] == text.as_bytes()[0]
                || pattern.as_bytes()[0] as char == '.');

        if pattern.len() >= 2 && pattern.as_bytes()[1] as char == '*' {
            return (Self::is_match(
                text.clone(),
                match pattern.get(2..) {
                    None => String::from(""),
                    Some(n) => String::from(n),
                },
            )) || (first_match
                && Self::is_match(
                    match text.get(1..) {
                        None => String::from(""),
                        Some(n) => String::from(n),
                    },
                    pattern,
                ));
        } else {
            return first_match
                && Self::is_match(
                    match text.get(1..) {
                        None => String::from(""),
                        Some(n) => String::from(n),
                    },
                    match pattern.get(1..) {
                        None => String::from(""),
                        Some(n) => String::from(n),
                    },
                );
        }
    }
}
