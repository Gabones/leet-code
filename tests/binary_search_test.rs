
use leet_code::binary_search::Solution;

#[test]
fn case_1() {
    assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 9), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::search(vec![-1,0,3,5,9,12], 2), -1);
}

#[test]
fn case_3() {
    assert_eq!(Solution::search(vec![5], -5), -1);
}

#[test]
fn case_4() {
    assert_eq!(Solution::search(vec![5], 5), 0);
}

#[test]
fn case_5() {
    assert_eq!(Solution::search(vec![2,5], 5), 1);
}

#[test]
fn case_6() {
    assert_eq!(Solution::search(vec![2,5], 0), -1);
}