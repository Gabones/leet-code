use leet_code::car_fleet::Solution;

#[test]
fn case_1() {
    assert_eq!(
        Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]),
        3
    );
}
