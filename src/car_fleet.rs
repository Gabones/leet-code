pub struct Solution;

impl Solution {
    pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut stack: Vec<f32> = Vec::new();
        let mut pairs: Vec<(i32, i32, f32)> = position
            .iter()
            .zip(speed.iter())
            .map(|(&p, &s)| (p, s, ((target - p) as f32 / s as f32)))
            .collect();
        pairs.sort_by_key(|&(p, _s, _t)| p);

        for &(_p, _s, t) in pairs.iter().rev() {
            stack.push(t);
            if stack.len() >= 2
                && stack.last().unwrap() <= stack.get(stack.len().wrapping_sub(2)).unwrap()
            {
                stack.pop();
            }
        }

        stack.len() as i32
    }
}
