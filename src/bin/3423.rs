use std::iter::once;

/// Category: algorithms
/// Level: Easy
/// Percent: 68.769165%

/// Given a circular array nums, find the maximum absolute difference between adjacent elements.
///
/// Note: In a circular array, the first and last elements are adjacent.
///
///
/// Example 1:
///
///
/// Input: nums = [1,2,4]
///
/// Output: 3
///
/// Explanation:
///
/// Because nums is circular, nums[0] and nums[2] are adjacent. They have the maximum absolute difference of |4 - 1| = 3.
///
///
/// Example 2:
///
///
/// Input: nums = [-5,-10,-5]
///
/// Output: 5
///
/// Explanation:
///
/// The adjacent elements nums[0] and nums[1] have the maximum absolute difference of |-5 - (-10)| = 5.
///
///
///
/// Constraints:
///
///
/// 	2 <= nums.length <= 100
/// 	-100 <= nums[i] <= 100
///

impl Solution {
    pub fn max_adjacent_distance(nums: Vec<i32>) -> i32 {
        nums.iter()
            .zip(nums.iter().skip(1).chain(once(&nums[0])))
            .map(|(a, b)| (a - b).abs())
            .max()
            .unwrap()
    }
}
