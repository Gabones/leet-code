use crate::Solution;

#[test]
fn case_1() {
    assert_eq!(
        Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]),
        vec![[-1, -1, 2], [-1, 0, 1]]
    )
}

#[test]
fn case_2() {
    let v: Vec<Vec<i32>> = Vec::new();
    assert_eq!(Solution::three_sum(vec![0, 1, 1]), v)
}

#[test]
fn case_3() {
    assert_eq!(Solution::three_sum(vec![0, 0, 0]), vec![[0, 0, 0]])
}

#[test]
fn case_61() {
    assert_eq!(
        Solution::three_sum(vec![3, 0, -2, -1, 1, 2]),
        vec![[-2, -1, 3], [-2, 0, 2], [-1, 0, 1]]
    )
}
