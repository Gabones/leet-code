use leet_code::evaluate_reverse_polish_notation::Solution;
use utils::vec_of_strings;

#[test]
fn case_1() {
    assert_eq!(
        Solution::eval_rpn(vec_of_strings!["2", "1", "+", "3", "*"]),
        9
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::eval_rpn(vec_of_strings!["4", "13", "5", "/", "+"]),
        6
    )
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::eval_rpn(vec_of_strings![
            "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"
        ]),
        22
    )
}
