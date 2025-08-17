use std::str::CharIndices;

/// Category: algorithms
/// Level: Medium
/// Percent: 44.78908%

/// Alice plays the following game, loosely based on the card game "21".
///
/// Alice starts with 0 points and draws numbers while she has less than k points. During each draw, she gains an integer number of points randomly from the range [1, maxPts], where maxPts is an integer. Each draw is independent and the outcomes have equal probabilities.
///
/// Alice stops drawing numbers when she gets k or more points.
///
/// Return the probability that Alice has n or fewer points.
///
/// Answers within 10-5 of the actual answer are considered accepted.
///
///
/// Example 1:
///
/// Input: n = 10, k = 1, maxPts = 10
/// Output: 1.00000
/// Explanation: Alice gets a single card, then stops.
///
///
/// Example 2:
///
/// Input: n = 6, k = 1, maxPts = 10
/// Output: 0.60000
/// Explanation: Alice gets a single card, then stops.
/// In 6 out of 10 possibilities, she is at or below 6 points.
///
///
/// Example 3:
///
/// Input: n = 21, k = 17, maxPts = 10
/// Output: 0.73278
///
///
///
/// Constraints:
///
///
/// 	0 <= k <= n <= 10⁴
/// 	1 <= maxPts <= 10⁴
///

impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        if k == 0 {
            return 1f64;
        }
        // We can solve this problem with dynamic programming
        // Let dp[i] be the possibility that Alice ends up with i points
        // The result is sum of dp[1..=n]
        // dp[i] = sum of dp[i-max_pts..=i-1]*1/max_pts
        // But Alice won't pick more cards if she already have k points
        let n = n as usize;
        let k = k as usize;
        let max_pts = max_pts as usize;
        let mut dp = vec![0f64; k + max_pts + 1]; // The max points we can get is k - 1 + max_pts
        for i in 1..=max_pts {
            dp[i] = 1f64 / max_pts as f64;
        }
        for i in 1..k {
            for j in i + 1..=i + max_pts {
                if j < dp.len() {
                    dp[j] += dp[i] / max_pts as f64;
                }
            }
        }
        dp.iter().skip(k).take(n - k + 1).sum()
    }
}
