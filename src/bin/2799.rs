/// Category: algorithms
/// Level: Medium
/// Percent: 64.61512%

/// You are given an array nums consisting of positive integers.
///
/// We call a subarray of an array complete if the following condition is satisfied:
///
///
/// 	The number of distinct elements in the subarray is equal to the number of distinct elements in the whole array.
///
///
/// Return the number of complete subarrays.
///
/// A subarray is a contiguous non-empty part of an array.
///
///
/// Example 1:
///
/// Input: nums = [1,3,1,2,2]
/// Output: 4
/// Explanation: The complete subarrays are the following: [1,3,1,2], [1,3,1,2,2], [3,1,2] and [3,1,2,2].
///
///
/// Example 2:
///
/// Input: nums = [5,5,5,5]
/// Output: 10
/// Explanation: The array consists only of the integer 5, so any subarray is complete. The number of subarrays that we can choose is 10.
///
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 1000
/// 	1 <= nums[i] <= 2000
///
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn count_complete_subarrays(nums: Vec<i32>) -> i32 {
        let mut elements = HashSet::new();
        for x in nums.iter() {
            elements.insert(x);
        }
        let elements = elements.len();
        let mut count = HashMap::new();
        let mut start = 0;
        let mut end = 0;
        let mut res = 0;
        while end <= nums.len() {
            while count.len() == elements {
                res += nums.len() - end + 1;
                count.entry(nums[start]).and_modify(|x: &mut usize| *x -= 1);
                if count.get(&nums[start]).unwrap() == &0 {
                    count.remove(&nums[start]);
                }
                start += 1;
            }
            if end < nums.len() {
                *count.entry(nums[end]).or_insert(0) += 1;
            }
            end += 1;
        }
        res as i32
    }
}
