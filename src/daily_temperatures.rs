pub struct Solution;

impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = Vec::new();
        let mut result: Vec<i32> = Vec::new();

        stack.push((*temperatures.last().unwrap(), temperatures.len() - 1));
        result.push(0);

        for index in (0..temperatures.len() - 1).rev() {
            if temperatures[index] < stack.last().unwrap().0 {
                result.push((stack.last().unwrap().1 - index) as i32);
                stack.push((temperatures[index], index));
            } else {
                while stack.len() > 0 && temperatures[index] >= stack.last().unwrap().0 {
                    stack.pop();
                }
                result.push((stack.last().unwrap_or(&(temperatures[index], index)).1 - index) as i32);
                stack.push((temperatures[index], index));
            }
        }

        result.into_iter().rev().collect()
    }
}
