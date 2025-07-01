use std::iter::once;

/// Category: algorithms
/// Level: Easy
/// Percent: 56.81553%

/// We define a harmonious array as an array where the difference between its maximum value and its minimum value is exactly 1.
///
/// Given an integer array nums, return the length of its longest harmonious subsequence among all its possible subsequences.
///
///
/// Example 1:
///
///
/// Input: nums = [1,3,2,2,5,2,3,7]
///
/// Output: 5
///
/// Explanation:
///
/// The longest harmonious subsequence is [3,2,2,2,3].
///
///
/// Example 2:
///
///
/// Input: nums = [1,2,3,4]
///
/// Output: 2
///
/// Explanation:
///
/// The longest harmonious subsequences are [1,2], [2,3], and [3,4], all of which have a length of 2.
///
///
/// Example 3:
///
///
/// Input: nums = [1,1,1,1]
///
/// Output: 0
///
/// Explanation:
///
/// No harmonic subsequence exists.
///
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 2 * 10⁴
/// 	-10⁹ <= nums[i] <= 10⁹
///

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut sorted = nums;
        sorted.sort_unstable();
        let mut res = 0;
        let mut prev = sorted[0] - 2;
        let mut count_prev = 0;
        let mut curr = sorted[0] - 2;
        let mut count_curr = 0;
        for &num in sorted.iter().chain(once(&(sorted[sorted.len() - 1] + 2))) {
            if num == curr {
                count_curr += 1;
            } else {
                if prev + 1 == curr {
                    res = res.max(count_curr + count_prev);
                }
                prev = curr;
                count_prev = count_curr;
                curr = num;
                count_curr = 1;
            }
        }
        res
    }
}
