use crate::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(Solution::roman_to_int("III".to_string()), 3);
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(Solution::roman_to_int("LVIII".to_string()), 58);
}

#[test]
// #[ignore]
fn case_3() {
    assert_eq!(Solution::roman_to_int("MCMXCIV".to_string()), 1994);
}
