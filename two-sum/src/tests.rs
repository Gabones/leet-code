use crate::Solution;

#[test]
//#[ignore]
fn case_1() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    assert_eq!(Solution::two_sum(nums, target), [0, 1]);
}
#[test]
//#[ignore]
fn case_2() {
    let nums = vec![3, 2, 4];
    let target = 6;

    assert_eq!(Solution::two_sum(nums, target), [1, 2]);
}
#[test]
//#[ignore]
fn case_3() {
    let nums = vec![3, 3];
    let target = 6;

    assert_eq!(Solution::two_sum(nums, target), [0, 1]);
}
