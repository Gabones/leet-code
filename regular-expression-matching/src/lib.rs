#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn is_match(text: String, pattern: String) -> bool {
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
