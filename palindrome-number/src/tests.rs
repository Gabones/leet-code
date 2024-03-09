use crate::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert!(Solution::is_palindrome(121));
}


#[test]
// #[ignore]
fn case_2() {
    assert!(!Solution::is_palindrome(-121));
}

#[test]
// #[ignore]
fn case_3() {
    assert!(!Solution::is_palindrome(10));
}