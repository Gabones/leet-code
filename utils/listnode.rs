#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    pub fn append(&mut self, elem: i32) {
        match self.next {
            Option::Some(ref mut next_address) => {
                next_address.append(elem);
            }
            Option::None => {
                let node = ListNode {
                    val: elem,
                    next: Option::None,
                };
                self.next = Option::Some(Box::new(node))
            }
        }
    }
}

pub fn to_list(vector: Vec<i32>) -> Option<Box<ListNode>> {
    let mut curr = None;
    for &value in vector.iter().rev() {
        let mut new_node = ListNode::new(value);
        new_node.next = curr;
        curr = Some(Box::new(new_node));
    }
    curr
}

pub fn to_vector(list: &Option<Box<ListNode>>) -> Vec<i32> {
    let mut curr = list.as_ref().unwrap().as_ref();
    let mut vector: Vec<i32> = vec![];
    vector = loop {
        vector.push(curr.val);
        let option_next = curr.next.as_ref();
        if option_next != None {
            curr = option_next.unwrap().as_ref();
        } else {
            break vector;
        }
    };

    vector
}
