use crate::Solution;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[test]
fn case_1() {
    let strs = vec_of_strings!["neet","code","love","you"];
    let encoded_str = Solution::encode(strs.clone());
    assert_eq!(
        strs,
        Solution::decode(encoded_str)
    )
}

#[test]
fn case_2() {
    let strs = vec_of_strings!["we","say",":","yes"];
    let encoded_str = Solution::encode(strs.clone());
    assert_eq!(
        strs,
        Solution::decode(encoded_str)
    )
}