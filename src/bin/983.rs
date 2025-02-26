/// 983. Minimum Cost For Tickets
/// You have planned some train traveling one year in advance. The days of the year in which you will travel are given as an integer array days. Each day is an integer from 1 to 365.
///
/// Train tickets are sold in three different ways:
///
/// a 1-day pass is sold for costs[0] dollars,
/// a 7-day pass is sold for costs[1] dollars, and
/// a 30-day pass is sold for costs[2] dollars.
/// The passes allow that many days of consecutive travel.
///
/// For example, if we get a 7-day pass on day 2, then we can travel for 7 days: 2, 3, 4, 5, 6, 7, and 8.
/// Return the minimum number of dollars you need to travel every day in the given list of days.
///
/// Example 1:
///
/// Input: days = [1,4,6,7,8,20], costs = [2,7,15]
/// Output: 11
/// Explanation: For example, here is one way to buy passes that lets you travel your travel plan:
/// On day 1, you bought a 1-day pass for costs[0] = $2, which covered day 1.
/// On day 3, you bought a 7-day pass for costs[1] = $7, which covered days 3, 4, ..., 9.
/// On day 20, you bought a 1-day pass for costs[0] = $2, which covered day 20.
/// In total, you spent $11 and covered all the days of your travel.
/// Example 2:
///
/// Input: days = [1,2,3,4,5,6,7,8,9,10,30,31], costs = [2,7,15]
/// Output: 17
/// Explanation: For example, here is one way to buy passes that lets you travel your travel plan:
/// On day 1, you bought a 30-day pass for costs[2] = $15 which covered days 1, 2, ..., 30.
/// On day 31, you bought a 1-day pass for costs[0] = $2 which covered day 31.
/// In total, you spent $17 and covered all the days of your travel.
///
/// Constraints:
///
/// 1 <= days.length <= 365
/// 1 <= days[i] <= 365
/// days is in strictly increasing order.
/// costs.length == 3
/// 1 <= costs[i] <= 1000
struct Solution {}

impl Solution {
    /// This helper function should return the position in the dp array where a "new ticket" must be bought.
    /// That position is the day when the previously bought ticket does not include it.
    pub fn find_position(travel_days: &Vec<i32>, day_of_the_year: i32) -> usize {
        travel_days
            .iter()
            .enumerate()
            .find(|(_, d)| **d > day_of_the_year)
            .map_or(0, |(i, _)| i)
    }

    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        // We can solve this problem with dynamic programming.
        // Let dp[i] be the minimum number of dollars you need to travel every day in the first i days.
        let mut dp = vec![0; days.len() + 1];
        let day = costs[0];
        let week = costs[1];
        let month = costs[2];

        for i in 1..=days.len() {
            // Buy 1 day, and buy tickets for the days before it.
            let cost_d = day + dp[i - 1];
            // Buy 1 week, and buy tickets for the days before the last week.
            let cost_w = week + dp[Solution::find_position(&days, days[i - 1] - 7)];
            // Buy 1 month, and buy tickets for the days before the last month.
            let cost_m = month + dp[Solution::find_position(&days, days[i - 1] - 30)];
            // Pick the most efficient option.
            dp[i] = cost_d.min(cost_w).min(cost_m);
        }
        // The last item contains the solution to the problem.
        *dp.last().unwrap()
    }
}

fn main() {
    assert_eq!(Solution::find_position(&vec![1, 4, 6, 7, 8, 20], 5), 2);
    assert_eq!(Solution::find_position(&vec![1, 4, 5, 7, 8, 20], 5), 3);
    assert_eq!(Solution::find_position(&vec![1, 4, 6, 7, 8, 20], 7), 4);

    assert_eq!(
        Solution::mincost_tickets(vec![1, 4, 6, 7, 8, 20], vec![2, 7, 15]),
        11
    );
    assert_eq!(
        Solution::mincost_tickets(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 30, 31], vec![2, 7, 15]),
        17
    );
}
