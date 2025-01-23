/// 714. Best Time to Buy and Sell Stock with Transaction Fee
/// You are given an array prices where prices[i] is the price of a given stock on the ith day, and an integer fee representing a transaction fee.
/// Find the maximum profit you can achieve. You may complete as many transactions as you like, but you need to pay the transaction fee for each transaction.
/// Note:
/// You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
/// The transaction fee is only charged once for each stock purchase and sale.
///
/// Example 1:
/// Input: prices = [1,3,2,8,4,9], fee = 2
/// Output: 8
/// Explanation: The maximum profit can be achieved by:
/// - Buying at prices[0] = 1
/// - Selling at prices[3] = 8
/// - Buying at prices[4] = 4
/// - Selling at prices[5] = 9
/// The total profit is ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
///
/// Example 2:
/// Input: prices = [1,3,7,5,10,3], fee = 3
/// Output: 6
///
/// Constraints:
/// 1 <= prices.length <= 5 * 104
/// 1 <= prices[i] < 5 * 104
/// 0 <= fee < 5 * 104
struct Solution {}

fn main() {
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2), 8);
    assert_eq!(Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3), 6);
    assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5], 1), 3);
    assert_eq!(Solution::max_profit(vec![5, 4, 3, 2, 1], 1), 0);
    assert_eq!(Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 0), 13);
}

#[derive(Default, Debug)]
struct Profit {
    hold: i32,
    free: i32,
}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        // Just follow the approach in this great solution: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/solutions/2520407/rust-dp-solution-evolution-with-comments
        let hold = -prices[0];
        prices
            .into_iter()
            .fold(Profit { hold, free: 0 }, |profit, price| Profit {
                free: profit.free.max(profit.hold + price - fee),
                hold: profit.hold.max(profit.free - price),
            })
            .free
    }
}
