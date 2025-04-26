/// Category: algorithms
/// Level: Easy
/// Percent: 66.51998%

/// You are given an integer n.
///
/// Each number from 1 to n is grouped according to the sum of its digits.
///
/// Return the number of groups that have the largest size.
///
///
/// Example 1:
///
/// Input: n = 13
/// Output: 4
/// Explanation: There are 9 groups in total, they are grouped according sum of its digits of numbers from 1 to 13:
/// [1,10], [2,11], [3,12], [4,13], [5], [6], [7], [8], [9].
/// There are 4 groups with largest size.
///
///
/// Example 2:
///
/// Input: n = 2
/// Output: 2
/// Explanation: There are 2 groups [1], [2] of size 1.
///
///
///
/// Constraints:
///
///
/// 	1 <= n <= 10⁴
///
use std::collections::HashMap;

impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let mut count = HashMap::new();
        for mut x in 1..=n {
            let mut sum = 0;
            while x > 0 {
                sum += x % 10;
                x /= 10;
            }
            *count.entry(sum).or_insert(0) += 1;
        }
        let max = count.iter().max_by_key(|(_, c)| *c).unwrap().1;
        count.iter().filter(|(_, sum)| *sum == max).count() as i32
    }
}
