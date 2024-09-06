use crate::Solution;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[test]
fn case_1() {
    assert_eq!(
        Solution::group_anagrams(vec_of_strings!["eat", "tea", "tan", "ate", "nat", "bat"]),
        vec![
            vec_of_strings!["bat"],
            vec_of_strings!["nat", "tan"],
            vec_of_strings!["ate", "eat", "tea"]
        ]
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::group_anagrams(vec_of_strings![""]),
        vec![vec_of_strings![""]]
    )
}

#[test]
fn case_44() {
    assert_eq!(
        Solution::group_anagrams(vec_of_strings!["a"]),
        vec![vec_of_strings!["a"]]
    )
}
