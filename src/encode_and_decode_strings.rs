pub struct Solution;

impl Solution {
    pub fn encode(strs: Vec<String>) -> String {
        let mut result = String::new();
        for str in strs {
            let len = str.len().to_string();
            result.push_str(&format!("{len}#{str}"));
        }
        result
    }

    pub fn decode(str: String) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let mut index: i32 = 0;
        while index < str.len() as i32 {
            let mut j: i32 = index;
            while str.chars().nth(j as usize).unwrap() != '#' {
                j += 1;
            }
            let length: i32 = str[index as usize..j as usize].parse::<i32>().unwrap();
            result.push(str[(j + 1) as usize..(j + 1 + length) as usize].to_string());
            index = j + 1 + length;
        }

        result
    }
}
