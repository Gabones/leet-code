use leet_code::generate_parentheses::Solution;
use utils::vec_of_strings;

#[test]
fn case_1() {
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec_of_strings!["((()))", "(()())", "(())()", "()(())", "()()()"]
    )
}

#[test]
fn case_2() {
    assert_eq!(Solution::generate_parenthesis(1), vec_of_strings!["()"])
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::generate_parenthesis(2),
        vec_of_strings!["(())", "()()"]
    )
}
