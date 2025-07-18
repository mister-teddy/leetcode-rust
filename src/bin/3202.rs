/// Category: algorithms
/// Level: Medium
/// Percent: 39.715733%

/// You are given an integer array nums and a positive integer k.
/// A subsequence sub of nums with length x is called valid if it satisfies:
///
///
/// 	(sub[0] + sub[1]) % k == (sub[1] + sub[2]) % k == ... == (sub[x - 2] + sub[x - 1]) % k.
///
/// Return the length of the longest valid subsequence of nums.
///
/// Example 1:
///
///
/// Input: nums = [1,2,3,4,5], k = 2
///
/// Output: 5
///
/// Explanation:
///
/// The longest valid subsequence is [1, 2, 3, 4, 5].
///
///
/// Example 2:
///
///
/// Input: nums = [1,4,2,3,1,4], k = 3
///
/// Output: 4
///
/// Explanation:
///
/// The longest valid subsequence is [1, 4, 1, 4].
///
///
///
/// Constraints:
///
///
/// 	2 <= nums.length <= 10³
/// 	1 <= nums[i] <= 10⁷
/// 	1 <= k <= 10³
///

impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        // We can solve this problem with dynamic programming
        // Denote dp[i][j] be the longest length of the subsequence
        // where the latest num mod k = j and the next to latest
        // num mod k = i
        let k = k as usize;
        let mut dp = vec![vec![0; k]; k];
        let mut res = 0;
        for num in nums {
            for i in 0..k {
                let next = num as usize % k;
                dp[i][next] = dp[next][i] + 1;
                res = res.max(dp[i][next]);
            }
        }
        res
    }
}
