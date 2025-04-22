use std::f64::MAX_10_EXP;

/// Category: algorithms
/// Level: Hard
/// Percent: 27.423534%

/// You are given two integers n and maxValue, which are used to describe an ideal array.
///
/// A 0-indexed integer array arr of length n is considered ideal if the following conditions hold:
///
///
/// 	Every arr[i] is a value from 1 to maxValue, for 0 <= i < n.
/// 	Every arr[i] is divisible by arr[i - 1], for 0 < i < n.
///
///
/// Return the number of distinct ideal arrays of length n. Since the answer may be very large, return it modulo 10⁹ + 7.
///
///
/// Example 1:
///
/// Input: n = 2, maxValue = 5
/// Output: 10
/// Explanation: The following are the possible ideal arrays:
/// - Arrays starting with the value 1 (5 arrays): [1,1], [1,2], [1,3], [1,4], [1,5]
/// - Arrays starting with the value 2 (2 arrays): [2,2], [2,4]
/// - Arrays starting with the value 3 (1 array): [3,3]
/// - Arrays starting with the value 4 (1 array): [4,4]
/// - Arrays starting with the value 5 (1 array): [5,5]
/// There are a total of 5 + 2 + 1 + 1 + 1 = 10 distinct ideal arrays.
///
///
/// Example 2:
///
/// Input: n = 5, maxValue = 3
/// Output: 11
/// Explanation: The following are the possible ideal arrays:
/// - Arrays starting with the value 1 (9 arrays):
///    - With no other distinct values (1 array): [1,1,1,1,1]
///    - With 2nd distinct value 2 (4 arrays): [1,1,1,1,2], [1,1,1,2,2], [1,1,2,2,2], [1,2,2,2,2]
///    - With 2nd distinct value 3 (4 arrays): [1,1,1,1,3], [1,1,1,3,3], [1,1,3,3,3], [1,3,3,3,3]
/// - Arrays starting with the value 2 (1 array): [2,2,2,2,2]
/// - Arrays starting with the value 3 (1 array): [3,3,3,3,3]
/// There are a total of 9 + 1 + 1 = 11 distinct ideal arrays.
///
///
///
/// Constraints:
///
///
/// 	2 <= n <= 10⁴
/// 	1 <= maxValue <= 10⁴

impl Solution {
    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        // We can solve this problem with dynamic programming
        // Let `dp[i][j]` be the number of distinct ideal arrays of length i, end with j
        // - `dp[1][*] = 1`
        // - `dp[i][j] = dp[i-1][k]` with all `j%k == 0`
        const MODULO: i32 = 1_000_000_007;
        let n = n as usize;
        let max_value = max_value as usize;
        let mut dp = vec![vec![0 as i32; max_value + 1]; n + 1];
        dp[1].fill(1);
        for i in 1..n {
            for j in 1..=max_value {
                for k in (j..=max_value).step_by(j) {
                    dp[i + 1][k] += dp[i][j];
                    dp[i + 1][k] %= MODULO;
                }
            }
        }
        dp[n].iter().fold(0, |sum, x| (sum + x) % MODULO)
    }
}
