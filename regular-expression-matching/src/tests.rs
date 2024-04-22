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

#[test]
// #[ignore]
fn case_4() {
    assert!(!Solution::is_match(
        String::from("mississippi"),
        String::from("mis*is*p*.")
    ));
}

#[test]
// #[ignore]
fn case_5() {
    assert!(Solution::is_match(
        String::from("aab"),
        String::from("c*a*b")
    ));
}

#[test]
// #[ignore]
fn case_6() {
    assert!(!Solution::is_match(
        String::from("aaaaaaaaaaaaaaaaaaa"),
        String::from("a*a*a*a*a*a*a*a*a*b")
    ));
}
