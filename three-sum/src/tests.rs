use crate::Solution;

#[test]
fn case_1() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![[-1, -1, 2], [-1, 0, 1]]
    )
}
