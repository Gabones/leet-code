#![allow(dead_code)]

mod listnode;
use listnode::{to_list, to_vector, ListNode};
#[cfg(test)]
mod tests;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut first_str = String::new();
    let mut second_str = String::new();
    let mut current = l1.as_ref().unwrap().as_ref();

    // println!("{:?}", to_vector(&l1));
    // println!("{:?}", to_vector(&l2));

    first_str = loop {
        first_str.push_str(&current.val.to_string());
        let option_next = current.next.as_ref();
        if option_next != None {
            current = option_next.unwrap().as_ref();
        } else {
            break first_str;
        }
    };

    let mut current = l2.as_ref().unwrap().as_ref();
    second_str = loop {
        second_str.push_str(&current.val.to_string());
        let option_next = current.next.as_ref();
        if option_next != None {
            current = option_next.unwrap().as_ref();
        } else {
            break second_str;
        }
    };

    let first = first_str
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u128>()
        .unwrap_or(0);
    let second = second_str
        .chars()
        .rev()
        .collect::<String>()
        .parse::<u128>()
        .unwrap_or(0);

    let str_result = (first + second).to_string();

    let vec_result: Vec<i32> = str_result
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap_or(0) as i32)
        .collect();

    return to_list(vec_result);
}
