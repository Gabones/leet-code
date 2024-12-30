pub struct Solution;

impl Solution {
    pub fn backtrack(open_n: i32, close_n: i32, n: i32, res: &mut Vec<String>, stack: &mut String) {
        if open_n == close_n && open_n == n {
            res.push((*stack.clone()).to_string());
            return;
        }

        if open_n < n {
            *stack += "(";
            Self::backtrack(open_n + 1, close_n, n, res, stack);
            stack.pop();
        }

        if close_n < open_n {
            *stack += ")";
            Self::backtrack(open_n, close_n + 1, n, res, stack);
            stack.pop();
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut res: Vec<String> = Vec::new();
        let mut stack = String::new();
        Self::backtrack(0, 0, n, &mut res, &mut stack);
        println!("{:?}", res);
        return res;
    }
}
