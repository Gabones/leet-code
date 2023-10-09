#![allow(dead_code)]

mod listnode;
use listnode::ListNode;
#[cfg(test)]
mod tests;

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut first_str = String::new();
    let mut second_str = String::new();
    let mut current = l1.as_ref().unwrap().as_ref();

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

    let mut result = ListNode::new(0);
    for (i, word) in str_result.chars().rev().enumerate() {
        if i == 0 {
            result.val = word.to_digit(10).unwrap() as i32;
        } else {
            result.append(word.to_digit(10).unwrap() as i32);
        }
    }

    return Some(Box::new(result));
}
