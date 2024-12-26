use leet_code::two_sum_ii_input_array_is_sorted::Solution;

#[test]
fn case_1() {
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![1, 2])
}

#[test]
fn case_2() {
    assert_eq!(Solution::two_sum(vec![2, 3, 4], 6), vec![1, 3])
}

#[test]
fn case_3() {
    assert_eq!(Solution::two_sum(vec![-1, 0], -1), vec![1, 2])
}
