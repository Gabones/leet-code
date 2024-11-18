use leet_code::longest_common_prefix::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(
        Solution::longest_common_prefix_binary_search(vec![
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
        Solution::longest_common_prefix_binary_search(vec![
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
    assert_eq!(
        Solution::longest_common_prefix_binary_search(vec![String::from(""),]),
        ""
    );
}

#[test]
// #[ignore]
fn case_4() {
    assert_eq!(
        Solution::longest_common_prefix_binary_search(vec![String::from("a"),]),
        "a"
    );
}

#[test]
// #[ignore]
fn case_120() {
    assert_eq!(
        Solution::longest_common_prefix_binary_search(vec![
            String::from("abb"),
            String::from("abc")
        ]),
        String::from("ab")
    );
}
#[test]
// #[ignore]
fn case_125() {
    assert_eq!(
        Solution::longest_common_prefix_binary_search(vec![
            String::from("abca"),
            String::from("abc")
        ]),
        String::from("abc")
    )
}
