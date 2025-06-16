/// Category: algorithms
/// Level: Easy
/// Percent: 58.982414%

/// Given a 0-indexed integer array nums of size n, find the maximum difference between nums[i] and nums[j] (i.e., nums[j] - nums[i]), such that 0 <= i < j < n and nums[i] < nums[j].
///
/// Return the maximum difference. If no such i and j exists, return -1.
///
///
/// Example 1:
///
/// Input: nums = [7,1,5,4]
/// Output: 4
/// Explanation:
/// The maximum difference occurs with i = 1 and j = 2, nums[j] - nums[i] = 5 - 1 = 4.
/// Note that with i = 1 and j = 0, the difference nums[j] - nums[i] = 7 - 1 = 6, but i > j, so it is not valid.
///
///
/// Example 2:
///
/// Input: nums = [9,4,3,2]
/// Output: -1
/// Explanation:
/// There is no i and j such that i < j and nums[i] < nums[j].
///
///
/// Example 3:
///
/// Input: nums = [1,5,2,10]
/// Output: 9
/// Explanation:
/// The maximum difference occurs with i = 0 and j = 3, nums[j] - nums[i] = 10 - 1 = 9.
///
///
///
/// Constraints:
///
///
/// 	n == nums.length
/// 	2 <= n <= 1000
/// 	1 <= nums[i] <= 10⁹
///

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut res = -1;
        let mut min = nums[0];
        for &num in nums.iter().skip(1) {
            if num > min {
                res = res.max(num - min);
            }
            min = min.min(num);
        }
        res
    }
}
