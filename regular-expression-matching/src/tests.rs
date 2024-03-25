use crate::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert!(!Solution::is_match(String::from("aa"), String::from("a")));
}

#[test]
// #[ignore]
fn case_2() {
    assert!(Solution::is_match(String::from("aa"), String::from("a*")));
}

#[test]
// #[ignore]
fn case_3() {
    assert!(Solution::is_match(String::from("aa"), String::from(".*")));
}
