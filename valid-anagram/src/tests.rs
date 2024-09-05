use crate::Solution;

#[test]
fn case_1() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    )
}

#[test]
fn case_44() {
    assert_eq!(
        Solution::is_anagram("aa".to_string(), "bb".to_string()),
        false
    )
}
