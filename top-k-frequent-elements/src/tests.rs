use crate::Solution;

#[test]
fn case_1() {
    assert_eq!(
        Solution::top_k_frequent(vec![1,1,1,2,2,3], 2),
        vec![1,2]
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::top_k_frequent(vec![1], 1),
        vec![1]
    )
}

