use leet_code::koko_eating_bananas::Solution;

#[test]
fn case_1() {
    assert_eq!(Solution::min_eating_speed(vec![3, 6, 7, 11], 8), 4);
}

#[test]
fn case_2() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 5), 30);
}

#[test]
fn case_3() {
    assert_eq!(Solution::min_eating_speed(vec![30, 11, 23, 4, 20], 6), 23);
}

#[test]
fn case_125() {
    assert_eq!(
        Solution::min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000),
        3
    );
}
