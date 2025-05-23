use std::collections::HashMap;

/// Category: algorithms
/// Level: Hard
/// Percent: 67.68362%

/// You are given an integer array nums and two integers minK and maxK.
///
/// A fixed-bound subarray of nums is a subarray that satisfies the following conditions:
///
///
/// 	The minimum value in the subarray is equal to minK.
/// 	The maximum value in the subarray is equal to maxK.
///
///
/// Return the number of fixed-bound subarrays.
///
/// A subarray is a contiguous part of an array.
///
///
/// Example 1:
///
/// Input: nums = [1,3,5,2,7,5], minK = 1, maxK = 5
/// Output: 2
/// Explanation: The fixed-bound subarrays are [1,3,5] and [1,3,5,2].
///
///
/// Example 2:
///
/// Input: nums = [1,1,1,1], minK = 1, maxK = 1
/// Output: 10
/// Explanation: Every subarray of nums is a fixed-bound subarray. There are 10 possible subarrays.
///
///
///
/// Constraints:
///
///
/// 	2 <= nums.length <= 10⁵
/// 	1 <= nums[i], minK, maxK <= 10⁶
impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, min_k: i32, max_k: i32) -> i64 {
        // We can solve this problem using sliding window
        let mut left_bad = -1i64;
        let mut left_min = -1i64;
        let mut left_max = -1i64;
        let mut res = 0;
        for i in 0..nums.len() {
            let x = nums[i];
            let i = i as i64;
            if x < min_k || x > max_k {
                left_bad = i;
            }
            if x == min_k {
                left_min = i;
            }
            if x == max_k {
                left_max = i;
            }
            res += 0.max(left_min.min(left_max) - left_bad)
        }
        res
    }
}
