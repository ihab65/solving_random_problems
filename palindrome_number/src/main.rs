pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        let mut reversed = 0;
        let mut original = x;
        while original > 0 {
            let digit = original % 10;
            reversed = reversed * 10 + digit;
            original /= 10;
        }
        x == reversed
    }
}

fn main() {
    let x = 123;
    let result = Solution::is_palindrome(x);
    println!("Is {} a palindrome? {}", x, result);
}