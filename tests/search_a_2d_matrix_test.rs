use leet_code::search_a_2d_matrix::Solution;

#[test]
fn case_1() {
    assert!(Solution::search_matrix(
        vec![
            [1, 3, 5, 7].to_vec(),
            [10, 11, 16, 20].to_vec(),
            [23, 30, 34, 60].to_vec()
        ],
        3
    ))
}

#[test]
fn case_2() {
    assert!(!Solution::search_matrix(
        vec![
            [1, 3, 5, 7].to_vec(),
            [10, 11, 16, 20].to_vec(),
            [23, 30, 34, 60].to_vec()
        ],
        13
    ))
}
