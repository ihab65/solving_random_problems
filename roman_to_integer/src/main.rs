struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev_value = 0;

        for c in s.chars() {
            let value = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if value > prev_value {
                result += value - 2 * prev_value; // Subtract twice the previous value
            } else {
                result += value;
            }
            prev_value = value;
        }
        result
    }
}

fn main() {
    let s = "MCMXCIV".to_string();
    let result = Solution::roman_to_int(s);
    println!("The integer value of the Roman numeral is: {}", result);
}
