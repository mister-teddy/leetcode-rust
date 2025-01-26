/// 188. Best Time to Buy and Sell Stock IV
/// You are given an integer array prices where prices[i] is the price of a given stock on the ith day, and an integer k.
///
/// Find the maximum profit you can achieve. You may complete at most k transactions: i.e. you may buy at most k times and sell at most k times.
///
/// Note: You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
///
///
/// Example 1:
///
/// Input: k = 2, prices = [2,4,1]
/// Output: 2
/// Explanation: Buy on day 1 (price = 2) and sell on day 2 (price = 4), profit = 4-2 = 2.
/// Example 2:
///
/// Input: k = 2, prices = [3,2,6,5,0,3]
/// Output: 7
/// Explanation: Buy on day 2 (price = 2) and sell on day 3 (price = 6), profit = 6-2 = 4. Then buy on day 5 (price = 0) and sell on day 6 (price = 3), profit = 3-0 = 3.
///
///
/// Constraints:
///
/// 1 <= k <= 100
/// 1 <= prices.length <= 1000
/// 0 <= prices[i] <= 1000
struct Solution;

fn main() {
    assert_eq!(Solution::max_profit(100, vec![2]), 0);
    assert_eq!(Solution::max_profit(2, vec![2, 4, 1]), 2);
    assert_eq!(Solution::max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
    assert_eq!(Solution::max_profit(1, vec![1, 2, 3, 4, 5]), 4);
    assert_eq!(Solution::max_profit(2, vec![7, 6, 4, 3, 1]), 0);
    assert_eq!(
        Solution::max_profit(3, vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]),
        15
    );
    assert_eq!(Solution::max_profit(2, vec![1, 2]), 1);
}

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        // This solution is just a more general version of 714
        let first_price = prices[0];
        prices
            .into_iter()
            .fold(vec![(-first_price, 0); k as usize], |mut profit, price| {
                for i in 0..k as usize {
                    profit[i] = (
                        profit[i]
                            .0
                            .max(if i == 0 { 0 } else { profit[i - 1].1 } - price),
                        profit[i].1.max(profit[i].0 + price),
                    )
                }
                profit
            })
            .last()
            .unwrap()
            .1
    }
}
