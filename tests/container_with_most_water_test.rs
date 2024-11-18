use leet_code::container_with_most_water::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(Solution::max_area(vec![1, 1]), 1);
}
