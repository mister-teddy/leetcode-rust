/// Category: algorithms
/// Level: Medium
/// Percent: 58.46045%

/// You are given an integer array nums and a positive integer k.
///
/// Return the number of subarrays where the maximum element of nums appears at least k times in that subarray.
///
/// A subarray is a contiguous sequence of elements within an array.
///
///
/// Example 1:
///
/// Input: nums = [1,3,2,3,3], k = 2
/// Output: 6
/// Explanation: The subarrays that contain the element 3 at least 2 times are: [1,3,2,3], [1,3,2,3,3], [3,2,3], [3,2,3,3], [2,3,3] and [3,3].
///
///
/// Example 2:
///
/// Input: nums = [1,4,2,1], k = 3
/// Output: 0
/// Explanation: No subarray contains the element 4 at least 3 times.
///
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 10⁵
/// 	1 <= nums[i] <= 10⁶
/// 	1 <= k <= 10⁵
///

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let max = *nums.iter().max().unwrap();
        let k = k as usize;
        let mut left = vec![];
        let mut res = 0i64;
        for right in 0..nums.len() {
            if nums[right] == max {
                left.push(right);
                if left.len() > k {
                    left.remove(0);
                }
            }
            if left.len() == k {
                res += left[0] as i64 + 1;
            }
        }
        res
    }
}
