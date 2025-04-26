struct Solution;

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length = 0;
        let mut counting = false; 
        
        for c in s.chars().rev() {
            if c != ' ' {
                counting = true;
                length += 1;
            } else if counting {
                break;
            }
        }
        length
    }
}

fn main() {
    let s = "Hello World".to_string();
    let result = Solution::length_of_last_word(s);
    println!("The length of the last word is: {}", result);

    let s = "   fly me   to   the moon  ".to_string();
    let result = Solution::length_of_last_word(s);
    println!("The length of the last word is: {}", result);
}