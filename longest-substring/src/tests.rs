use crate::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("abcabcbb")),
        3
    );
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("bbbbb")),
        1
    );
}

#[test]
// #[ignore]
fn case_3() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("pwwkew")),
        3
    );
}

#[test]
// #[ignore]
fn case_340() {
    assert_eq!(Solution::length_of_longest_substring(String::from(" ")), 1);
}

#[test]
// #[ignore]
fn case_407() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("dvdf")),
        3
    );
}

#[test]
// #[ignore]
fn case_477() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("anviaj")),
        5
    );
}

#[test]
// #[ignore]
fn case_702() {
    assert_eq!(
        Solution::length_of_longest_substring(String::from("busvutpwmu")),
        7
    );
}
