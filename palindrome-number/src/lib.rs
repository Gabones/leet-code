#[cfg(test)]
mod tests;

pub struct Solution;


impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        
        if x >= 0 && x <= 9 {
            return true;
        }

        let radix = 10;
        let mut n = x;
        let mut reversed = 0;

        while n != 0 {
            reversed = reversed * radix + n % radix;
            n /= radix;
        }

        reversed == x
    }
}
