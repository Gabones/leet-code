use crate::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("flower"),
            String::from("flow"),
            String::from("flight")
        ]),
        "fl"
    );
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            String::from("dog"),
            String::from("racecar"),
            String::from("car")
        ]),
        ""
    );
}

#[test]
// #[ignore]
fn case_3() {
    assert_eq!(Solution::longest_common_prefix(vec![String::from(""),]), "");
}

#[test]
// #[ignore]
fn case_4() {
    assert_eq!(
        Solution::longest_common_prefix(vec![String::from("a"),]),
        "a"
    );
}
