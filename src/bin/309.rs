/// Best Time to Buy and Sell Stock with Cooldown
/// You are given an array prices where prices[i] is the price of a given stock on the ith day.
/// Find the maximum profit you can achieve. You may complete as many transactions as you like (i.e., buy one and sell one share of the stock multiple times) with the following restrictions:
/// After you sell your stock, you cannot buy stock on the next day (i.e., cooldown one day). Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
///
/// Example 1:
/// Input: prices = [1,2,3,0,2]
/// Output: 3
/// Explanation: transactions = [buy, sell, cooldown, buy, sell]
///
/// Example 2:
/// Input: prices = [1]
/// Output: 0
///
/// Constraints:
/// 1 <= prices.length <= 5000 0 <= prices[i] <= 1000
struct Solution {}

fn main() {
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 0, 2]), 3);
    assert_eq!(Solution::max_profit(vec![1]), 0);
    assert_eq!(Solution::max_profit(vec![6, 1, 3, 2, 4, 7]), 6);
    assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 11);
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // We can solve this problem with dynamic programming
        // Let dp[i] be the maximum profit you can achieve in the first i days
        let mut dp = vec![0; prices.len()];

        // dp[0] = dp[1] = 0
        // dp[i] = max(dp[i-1], prices[i] - prices[j] + dp[j-2])
        for i in 1..prices.len() {
            dp[i] = dp[i - 1];
            for j in 0..i {
                dp[i] = dp[i].max(if j >= 2 { dp[j - 2] } else { 0 } + prices[i] - prices[j]);
            }
        }

        *dp.last().unwrap()
    }
}
