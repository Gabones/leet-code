use leet_code::daily_temperatures::Solution;

#[test]
fn case_1() {
    assert_eq!(
        Solution::daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]),
        vec![1, 1, 4, 2, 1, 1, 0, 0]
    )
}

#[test]
fn case_2() {
    assert_eq!(
        Solution::daily_temperatures(vec![30, 40, 50, 60]),
        vec![1, 1, 1, 0]
    )
}

#[test]
fn case_3() {
    assert_eq!(
        Solution::daily_temperatures(vec![30, 60, 90]),
        vec![1, 1, 0]
    )
}

#[test]
fn case_4() {
    assert_eq!(
        Solution::daily_temperatures(vec![89, 62, 70, 58, 47, 47, 46, 76, 100, 70]),
        vec![8, 1, 5, 4, 3, 2, 1, 1, 0, 0]
    )
}
