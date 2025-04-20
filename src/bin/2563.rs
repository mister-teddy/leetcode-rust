/// Category: algorithms
/// Level: Medium
/// Percent: 48.81495%

/// Given a 0-indexed integer array nums of size n and two integers lower and upper, return the number of fair pairs.
///
/// A pair (i, j) is fair if:
///
///
/// 	0 <= i < j < n, and
/// 	lower <= nums[i] + nums[j] <= upper
///
///
///
/// Example 1:
///
/// Input: nums = [0,1,7,4,4,5], lower = 3, upper = 6
/// Output: 6
/// Explanation: There are 6 fair pairs: (0,3), (0,4), (0,5), (1,3), (1,4), and (1,5).
///
///
/// Example 2:
///
/// Input: nums = [1,7,9,2,5], lower = 11, upper = 11
/// Output: 1
/// Explanation: There is a single fair pair: (2,3).
///
///
///
/// Constraints:
///
///
/// 	1 <= nums.length <= 10⁵
/// 	nums.length == n
/// 	-10⁹ <= nums[i] <= 10⁹
/// 	-10⁹ <= lower <= upper <= 10⁹
///
use std::cmp::Ordering;

impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut sorted = nums.clone();
        sorted.sort_unstable();
        let mut res = 0 as i64;
        for i in 1..nums.len() {
            let min_need = lower - sorted[i];
            let max_need = upper - sorted[i];
            let min_j = sorted[0..i].binary_search_by(|x| {
                if *x >= min_need {
                    Ordering::Greater // search left
                } else {
                    Ordering::Less
                }
            });
            let max_j = sorted[0..i].binary_search_by(|x| {
                if *x <= max_need {
                    Ordering::Less // search right
                } else {
                    Ordering::Greater
                }
            });
            if let Err(min_j) = min_j {
                if let Err(max_j) = max_j {
                    if min_j < i && (max_j > 0 || (max_j == 0 && sorted[max_j] <= max_need)) {
                        res += (max_j - min_j) as i64;
                    }
                }
            }
        }
        res
    }
}
