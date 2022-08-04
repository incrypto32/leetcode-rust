// Leetcode #1: https://leetcode.com/problems/two-sum/
//
use std::{collections::HashMap, vec::Vec};
struct Solution {}

impl Solution {
    pub fn solution(nums: Vec<i32>, _target: i32) -> Vec<i32> {
        let mut num_map: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            num_map.insert(*num, i as i32);
        }

        for (i, num) in nums.iter().enumerate() {
            let comp = _target - num;
            if let Some(&index) = num_map.get(&comp) {
                return vec![i as i32, index];
            }
        }

        vec![]
    }
}

fn main() {
    let nums = vec![3, 3];
    let target = 6;
    let result = Solution::solution(nums, target);
    println!("{:?}", result);
}
