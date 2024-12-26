use leet_code::valid_palindrome::Solution;

#[test]
fn case_1() {
    assert!(Solution::is_palindrome(
        "A man, a plan, a canal: Panama".to_string()
    ));
}

#[test]
fn case_2() {
    assert!(!Solution::is_palindrome("race a car".to_string()));
}

#[test]
fn case_3() {
    assert!(Solution::is_palindrome(" ".to_string()));
}

#[test]
fn case_5() {
    assert!(Solution::is_palindrome("a.".to_string()));
}
