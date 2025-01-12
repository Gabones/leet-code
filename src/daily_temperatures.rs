pub struct Solution;

impl Solution {
    pub fn daily_temperatures_brute_force(temperatures: Vec<i32>) -> Vec<i32> {
        let n = temperatures.len();
        let mut result: Vec<i32> = vec![0; n];

        for i in 0..result.len() {
            for j in i..result.len() {
                if temperatures[j] > temperatures[i] {
                    result[i] = (j - i) as i32;
                    break;
                }
            }
        }

        result
    }

    // Approach 2.1 - Reverse Monotonic Stack
    pub fn daily_temperatures_reverse_stack(temperatures: Vec<i32>) -> Vec<i32> {
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
                result
                    .push((stack.last().unwrap_or(&(temperatures[index], index)).1 - index) as i32);
                stack.push((temperatures[index], index));
            }
        }

        result.into_iter().rev().collect()
    }

    // Approach 2.2 - Monotonic Stack
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = Vec::new();
        let mut result: Vec<i32> = vec![0; temperatures.len()];

        for (index, &temp) in temperatures.iter().enumerate() {
            while stack.len() > 0 && temp > stack.last().unwrap().0 {
                let (_s_temp, s_index) = stack.pop().unwrap();
                result[s_index] = (index - s_index) as i32;
            }
            stack.push((temp, index));
        }

        result
    }
}
