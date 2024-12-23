use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    // Approach 1 - Brute Force
    pub fn three_sum_brute_force(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut set_result = HashSet::new();

        for i in 0..nums.len() {
            for j in i+1..nums.len() {
                for k in j+1..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        set_result.insert(vec![nums[i], nums[j], nums[k]]);
                    }
                }
            }
        }
        
        let mut result:Vec<Vec<i32>> = set_result.into_iter().collect();
        result.sort();
        result
    }

    // Approach 2 - HashMap
    pub fn three_sum_hashmap(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut count: HashMap<i32, i32> = HashMap::new();

        for num in nums.iter() {
           *count.entry(*num).or_insert(0) += 1;
        }

        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..nums.len() {
            *count.entry(nums[i]).or_insert(0) -= 1;
            if i > 0 && nums[i] == nums[i-1] {
                continue;
            }

            for j in i+1..nums.len() {
                *count.entry(nums[j]).or_insert(0) -= 1;
                if j > i + 1 && nums[j] == nums[j-1] {
                    continue;
                }
                
                let target = -(nums[i] + nums[j]);
                if count.get(&target).is_some() && *count.get(&target).unwrap() > 0 {
                    result.push(vec![nums[i], nums[j], target]);
                }
            }

            for j in i+1..nums.len() {
                *count.entry(nums[j]).or_insert(0) += 1;
            }
        }
        result
    }

    // Approach 3 - Two Pointers
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        nums.sort();
        let mut result: Vec<Vec<i32>> = Vec::new();

        for i in 0..nums.len() {
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }

            let (mut j, mut k) = (i + 1, nums.len() - 1);

            while j < k {
                let three_sum = nums[i] + nums[k] + nums[j];
                match three_sum.cmp(&0) {
                    std::cmp::Ordering::Greater => k -= 1,
                    std::cmp::Ordering::Less => j += 1,
                    _ => {
                        result.push(vec![nums[i], nums[j], nums[k]]);
                        j += 1;
                        while nums[j] == nums[j - 1] && j < k {
                            j += 1;
                        }
                    }
                }
            }
        }

        result
    }
}
