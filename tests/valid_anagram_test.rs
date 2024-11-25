use leet_code::valid_anagram::Solution;

#[test]
fn case_1() {
    assert!(Solution::is_anagram(
        "anagram".to_string(),
        "nagaram".to_string()
    ))
}

#[test]
fn case_2() {
    assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()))
}

#[test]
fn case_44() {
    assert!(!Solution::is_anagram("aa".to_string(), "bb".to_string()))
}
