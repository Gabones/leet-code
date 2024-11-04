pub struct Solution;

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut trimmed = s.trim().chars();
        let mut start = String::from(trimmed.next().unwrap_or('0'));
        let value = trimmed.take_while(|c| c.is_numeric()).collect::<String>();
        start.push_str(&value);
        match start.parse() {
            Ok(result) => result,
            Err(ref e) if *e.kind() == std::num::IntErrorKind::NegOverflow => i32::MIN,
            Err(ref e) if *e.kind() == std::num::IntErrorKind::PosOverflow => i32::MAX,
            _ => 0,
        }
    }
}
