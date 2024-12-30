use leet_code::min_stack::MinStack;

#[test]
fn case_1() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    assert_eq!(obj.get_min(), -3);
    obj.pop();
    assert_eq!(obj.top(), 0);
    assert_eq!(obj.get_min(), -2);
}
