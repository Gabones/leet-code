use leet_code::contains_duplicate::Solution;

#[test]
fn case_1() {
    assert!(Solution::contains_duplicate([1, 2, 3, 1].to_vec()))
}

#[test]
fn case_2() {
    assert!(!Solution::contains_duplicate([1, 2, 3, 4].to_vec()))
}

#[test]
fn case_3() {
    assert!(Solution::contains_duplicate(
        [1, 1, 1, 3, 3, 4, 3, 2, 4, 2].to_vec()
    ),)
}
