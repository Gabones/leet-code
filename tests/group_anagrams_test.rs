use leet_code::group_anagrams::Solution;

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[test]
fn case_1() {
    assert_eq!(
        Solution::group_anagrams(vec_of_strings!["eat", "tea", "tan", "ate", "nat", "bat"]),
        vec![vec!["bat"], vec!["tan", "nat"], vec!["eat", "tea", "ate"]]
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
