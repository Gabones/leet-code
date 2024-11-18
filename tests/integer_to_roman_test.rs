use leet_code::integer_to_roman::Solution;

#[test]
// #[ignore]
fn case_1() {
    assert_eq!(Solution::int_to_roman(3749), "MMMDCCXLIX");
}

#[test]
// #[ignore]
fn case_2() {
    assert_eq!(Solution::int_to_roman(58), "LVIII");
}

#[test]
// #[ignore]
fn case_3() {
    assert_eq!(Solution::int_to_roman(1994), "MCMXCIV");
}
