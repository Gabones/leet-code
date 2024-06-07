#[cfg(test)]
mod tests;

pub struct Solution;

impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut number = num;
        let num = vec![
            (1, "I"),
            (4, "IV"),
            (5, "V"),
            (9, "IX"),
            (10, "X"),
            (40, "XL"),
            (50, "L"),
            (90, "XC"),
            (100, "C"),
            (400, "CD"),
            (500, "D"),
            (900, "CM"),
            (1000, "M"),
        ];
        let mut i = num.len() - 1;
        let mut result = String::new();
        while number > 0 {
            let mut div: i32 = number / num[i].0;
            number = number % num[i].0;
            while div > 0 {
                result.push_str(num[i as usize].1);
                div -= 1;
            }
            if i > 0 {
                i -= 1;
            }
        }

        result
    }

    pub fn int_to_roman_not_fast(num: i32) -> String {
        let mut number = num;
        let num: Vec<i32> = vec![1, 4, 5, 9, 10, 40, 50, 90, 100, 400, 500, 900, 1000];
        let sym: Vec<&str> = vec![
            "I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M",
        ];
        let mut i = num.len() - 1;
        let mut result: Vec<String> = Vec::new();
        while number > 0 {
            let mut div: i32 = number / num[i];
            number = number % num[i];
            while div > 0 {
                result.push(sym[i as usize].to_string());
                div -= 1;
            }
            if i > 0 {
                i -= 1;
            }
        }

        result.join("")
    }
}
