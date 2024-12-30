pub struct Solution;

impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut numbers: Vec<i32> = Vec::new();
        for s in tokens {
            match s.as_str() {
                "+" => {
                    let op_1 = numbers.pop().unwrap();
                    let op_2 = numbers.pop().unwrap();
                    numbers.push(op_2 + op_1);
                }
                "-" => {
                    let op_1 = numbers.pop().unwrap();
                    let op_2 = numbers.pop().unwrap();
                    numbers.push(op_2 - op_1);
                }
                "*" => {
                    let op_1 = numbers.pop().unwrap();
                    let op_2 = numbers.pop().unwrap();
                    numbers.push(op_2 * op_1);
                }
                "/" => {
                    let op_1 = numbers.pop().unwrap();
                    let op_2 = numbers.pop().unwrap();
                    numbers.push(op_2 / op_1);
                }
                _ => {
                    numbers.push(s.parse::<i32>().unwrap());
                }
            }
        }
        numbers.pop().unwrap()
    }
}
