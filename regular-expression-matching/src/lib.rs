#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        if p.len() == 0 {
            return false;
        }

        let p0 = p.chars().nth(0).unwrap_or('\0');
        let t0 = p.chars().nth(0).unwrap_or('\0');
        let first_match = s.len() > 0 && (p0 == t0 || p0 == '*');

        if p.len() >= 2 && p.chars().nth(1).unwrap_or('\0') == '*' {
            return (Self::is_match(s.clone(), String::from(&p[2..p.len()])))
                || first_match && Self::is_match(String::from(&s[1..s.len()]), p);
        } else {
            return first_match
                && Self::is_match(String::from(&s[1..s.len()]), String::from(&p[1..p.len()]));
        }
    }
}
