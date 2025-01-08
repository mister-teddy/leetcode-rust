use std::collections::HashMap;

/// # 1027. Longest Arithmetic Subsequence
/// Given an array nums of integers, return the length of the longest arithmetic subsequence in nums.
///
/// Note that:
/// - A subsequence is an array that can be derived from another array by deleting some or no elements without changing the order of the remaining elements.
/// - A sequence seq is arithmetic if seq[i + 1] - seq[i] are all the same value (for 0 <= i < seq.length - 1).
///
/// Example 1:
/// Input: nums = [3,6,9,12]
/// Output: 4
/// Explanation:  The whole array is an arithmetic sequence with steps of length = 3.
///
/// Example 2:
/// Input: nums = [9,4,7,2,10]
/// Output: 3
/// Explanation:  The longest arithmetic subsequence is [4,7,10].
///
/// Example 3:
/// Input: nums = [20,1,15,3,10,5,8]
/// Output: 4
/// Explanation:  The longest arithmetic subsequence is [20,15,10,5].
///
/// Constraints:
/// 2 <= nums.length <= 1000
/// 0 <= nums[i] <= 500
struct Solution;

fn main() {
    assert_eq!(Solution::longest_arith_seq_length(vec![3, 6]), 2);
    assert_eq!(Solution::longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
    assert_eq!(Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
    assert_eq!(
        Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]),
        4
    );
}

impl Solution {
    /// Let dp[i][k] be the longest arithmetic sequence length with nums[i] as the last element of the sequence and k as the step between elements.
    /// it is clear that:
    /// - dp[0][*] = 0
    /// - dp[1][*] = 1
    /// - dp[i][k] =
    ///     + at least 1 (the element itself) and
    ///     + max dp[j][k] + 1 where j is the previous item (j < i) and k is the difference between nums[i] and nums[j]
    /// - The result of the solution is max of dp[*][*]
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut dp = vec![HashMap::new(); nums.len()];
        let mut res = 2;
        for i in 1..nums.len() {
            for j in 0..i {
                let diff = nums[i] - nums[j];
                let val = dp[j].get(&diff).unwrap_or(&1) + 1;
                dp[i].insert(diff, val);
                res = res.max(val);
            }
        }
        res
    }
}
