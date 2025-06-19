/// Category: algorithms
/// Level: Hard
/// Percent: 31.177198%

/// You are given three integers n, m, k. A good array arr of size n is defined as follows:
///
///
/// 	Each element in arr is in the inclusive range [1, m].
/// 	Exactly k indices i (where 1 <= i < n) satisfy the condition arr[i - 1] == arr[i].
///
///
/// Return the number of good arrays that can be formed.
///
/// Since the answer may be very large, return it modulo 109 + 7.
///
///
/// Example 1:
///
///
/// Input: n = 3, m = 2, k = 1
///
/// Output: 4
///
/// Explanation:
///
///
/// 	There are 4 good arrays. They are [1, 1, 2], [1, 2, 2], [2, 1, 1] and [2, 2, 1].
/// 	Hence, the answer is 4.
///
///
///
/// Example 2:
///
///
/// Input: n = 4, m = 2, k = 2
///
/// Output: 6
///
/// Explanation:
///
///
/// 	The good arrays are [1, 1, 1, 2], [1, 1, 2, 2], [1, 2, 2, 2], [2, 1, 1, 1], [2, 2, 1, 1] and [2, 2, 2, 1].
/// 	Hence, the answer is 6.
///
///
///
/// Example 3:
///
///
/// Input: n = 5, m = 2, k = 0
///

/// Output: 2
///
/// Explanation:
///
///
/// 	The good arrays are [1, 2, 1, 2, 1] and [2, 1, 2, 1, 2]. Hence, the answer is 2.
///
///
///
///
/// Constraints:
///
///
/// 	1 <= n <= 10⁵
/// 	1 <= m <= 10⁵
/// 	0 <= k <= n - 1
///

impl Solution {
    pub fn count_good_arrays(n: i32, m: i32, k: i32) -> i32 {
        // We can solve this problem using dynamic programming
        // Let dp[i][j] = the number of good arrays of size i and j (instead of k) indices satisfied the condition
        // dp[0][0] = 1 // Emtpy array
        // dp[0][*] = 0 // Impossible
        // dp[i][j] = dp[i-1][j-1] + (m-1)*dp[i-1][j]
        // Explan:
        // - There is only one way to append a similar character to the end of the previous array, and there are (m-1) ways to append a different character
        // - If you append a similar character, then you have to append (j-1) other similar character
        // - If you append a different chracter, then you left the j similar character for the other positions
        let n = n as usize;
        let k = k as usize;
        let mut dp = vec![vec![0; k + 1]; n + 1];
        dp[0][0] = 1;
        dp[1][0] = m;
        for i in 1..=n {
            if i > 1 {
                dp[i][0] = (m - 1) * dp[i - 1][0];
            }
            for j in 1..=k {
                dp[i][j] = dp[i - 1][j - 1] + (m - 1) * dp[i - 1][j];
            }
        }
        println!("{:?}", dp);
        dp[n][k]
    }
}
