use crate::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert!(Solution::my_atoi("42".to_owned()) == 42);
}

#[test]
// #[ignore]
fn case_2() {
    assert!(Solution::my_atoi("   -42".to_owned()) == -42);
}

#[test]
// #[ignore]
fn case_3() {
    assert!(Solution::my_atoi("4193 with words".to_owned()) == 4193);
}