use crate::Solution;

#[test]
fn case_1() {
    assert_eq!(
        Solution::product_except_self(vec![1,2,3,4]),
        vec![24,12,8,6]
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::product_except_self(vec![-1,1,0,-3,3]),
        vec![0,0,9,0,0]
    )
}
