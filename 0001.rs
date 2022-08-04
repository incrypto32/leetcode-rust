use std::{collections::HashMap, ops::Sub, vec::Vec};
struct Solution {}

impl Solution {
    pub fn solution(nums_c: Vec<i32>, _target: i32) -> Vec<i32> {
        let mut nums = nums_c;
        let mut num_map: HashMap<i32, i32> = HashMap::new();

        for (i, num) in nums.iter().enumerate() {
            num_map.insert(*num, i as i32);
        }

        nums.sort();
        let left: usize = 0;
        let right: usize = nums.len().sub(1);
        let result = Self::find(nums, _target, left, right);
        let left_res = result[0];
        let right_res = result[1];

        return vec![num_map[left_res], num_map[right_res]];
    }

    fn find(nums: Vec<i32>, _target: i32, left: usize, right: usize) -> Vec<i32> {
        let sum = nums[left as usize] + nums[right as usize];

        if sum == _target {
            return vec![nums[left as usize], nums[right as usize]];
        } else if sum < _target {
            return Self::find(nums, _target, left + 1, right);
        }

        return Self::find(nums, _target, left, right - 1);
    }
}

fn main() {
    let nums = vec![3, 3];
    let target = 6;
    let result = Solution::solution(nums, target);
    println!("{:?}", result);
}
