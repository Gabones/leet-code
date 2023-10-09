use crate::add_two_numbers;
use crate::listnode::to_list;

#[test]
fn case_1() {
    let l1 = to_list(vec![2, 4, 3]);
    let l2 = to_list(vec![5, 6, 4]);
    assert_eq!(add_two_numbers(l1, l2), to_list(vec![7, 0, 8]));
}

#[test]
fn case_2() {
    let l1 = to_list(vec![0]);
    let l2 = to_list(vec![0]);
    assert_eq!(add_two_numbers(l1, l2), to_list(vec![0]));
}

#[test]
fn case_3() {
    let l1 = to_list(vec![9, 9, 9, 9, 9, 9, 9]);
    let l2 = to_list(vec![9, 9, 9, 9]);
    assert_eq!(
        add_two_numbers(l1, l2),
        to_list(vec![8, 9, 9, 9, 0, 0, 0, 1])
    );
}
