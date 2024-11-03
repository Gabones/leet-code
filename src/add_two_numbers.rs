use utils::listnode::{to_list, to_vector, ListNode};

pub struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let v1 = to_vector(&l1);
        let v2 = to_vector(&l2);

        let mut carry = 0;
        let mut vec_result: Vec<i32> = vec![];
        let mut i = 0;
        vec_result = loop {
            carry += v1.get(i).unwrap_or(&0) + v2.get(i).unwrap_or(&0);
            vec_result.push(carry % 10);
            carry /= 10;
            i += 1;
            if v1.get(i).is_none() && v2.get(i).is_none() && carry == 0 {
                break vec_result;
            }
        };

        to_list(vec_result)
    }
}
