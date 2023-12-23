use crate::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert!(Solution::reverse(123) == 321);
}

#[test]
// #[ignore]
fn case_2() {
    assert!(Solution::reverse(-123) == -321);
}

#[test]
// #[ignore]
fn case_3() {
    assert!(Solution::reverse(120) == 21);
}
