use leet_code::two_sum::Solution;

#[test]
//#[ignore]
fn case_1() {
    assert_eq!(Solution::two_sum_brute_force(vec![2, 7, 11, 15], 9), [0, 1]);
    assert_eq!(
        Solution::two_sum_two_pass_hashmap(vec![2, 7, 11, 15], 9),
        [0, 1]
    );
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
}

#[test]
//#[ignore]
fn case_2() {
    assert_eq!(Solution::two_sum_brute_force(vec![3, 2, 4], 6), [1, 2]);
    assert_eq!(Solution::two_sum_two_pass_hashmap(vec![3, 2, 4], 6), [1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
}

#[test]
//#[ignore]
fn case_3() {
    assert_eq!(Solution::two_sum_brute_force(vec![3, 3], 6), [0, 1]);
    assert_eq!(Solution::two_sum_two_pass_hashmap(vec![3, 3], 6), [0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);
}
