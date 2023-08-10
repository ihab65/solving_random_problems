// Given an array of integers `nums` and an integer `target`, return indices of the two numbers such that they add up to target.
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
// You can return the answer in any order.

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut nums: Vec<(usize, i32)> =
        nums.into_iter()
            .enumerate()
            .collect();

    nums.sort_unstable_by_key(|&(_, n)| n);

    for (k, (i, ni)) in nums.iter().enumerate() {
        match nums[k+1..].binary_search_by_key(&(target - *ni), |&(_, nj)| nj) {
            Ok(jj) => return vec![*i as i32, nums[jj+k+1].0 as i32],
            Err(_) => {}
        }
    }
    unreachable!()
}

fn main() {
    let nums: Vec<i32> = vec![1, 3, 3, 1, 0];
    let target: i32 = 4;

    println!("{:?}", two_sum(nums, target))
}
