use crate::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert!(Solution::convert("PAYPALISHIRING".to_owned(), 3) == "PAHNAPLSIIGYIR");
}

#[test]
// #[ignore]
fn case_2() {
    assert!(Solution::convert("PAYPALISHIRING".to_owned(), 4) == "PINALSIGYAHRPI");
}

#[test]
// #[ignore]
fn case_3() {
    assert!(Solution::convert("A".to_owned(), 1) == "A");
}

#[test]
// #[ignore]
fn case_4() {
    assert!(Solution::convert("ABAB".to_owned(), 2) == "AABB");
}
