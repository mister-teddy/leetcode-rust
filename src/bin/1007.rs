use std::usize;

/// Category: algorithms
/// Level: Medium
/// Percent: 52.289997%

/// In a row of dominoes, tops[i] and bottoms[i] represent the top and bottom halves of the ith domino. (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)
///
/// We may rotate the ith domino, so that tops[i] and bottoms[i] swap values.
///
/// Return the minimum number of rotations so that all the values in tops are the same, or all the values in bottoms are the same.
///
/// If it cannot be done, return -1.
///
///
/// Example 1:
///
/// Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
/// Output: 2
/// Explanation:
/// The first figure represents the dominoes as given by tops and bottoms: before we do any rotations.
/// If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2, as indicated by the second figure.
///
///
/// Example 2:
///
/// Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
/// Output: -1
/// Explanation:
/// In this case, it is not possible to rotate the dominoes to make one row of values equal.
///
///
///
/// Constraints:
///
///
/// 	2 <= tops.length <= 2 * 10⁴
/// 	bottoms.length == tops.length
/// 	1 <= tops[i], bottoms[i] <= 6
///
impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        let n = tops.len();
        // We can solve this problem using dynamic programming
        // let dp[i][j][k] be the minimum number of rotations so that all the first i dominos have value k in the top row (if j = 0), or the bottom row (if j = 1)
        let mut dp = vec![[[-1i32; 6]; 2]; n + 1];

        // Base case
        dp[0][0].fill(0);
        dp[0][1].fill(0);

        for i in 1..=n {
            let top = tops[i - 1] as usize - 1;
            let bottom = bottoms[i - 1] as usize - 1;
            // For the top row, we can keep it or swap it
            let keep = dp[i - 1][0][top];
            dp[i][0][top] = keep;
            let swap = dp[i - 1][0][bottom] + 1;
            if swap != 0 {
                if top == bottom && keep != -1 {
                    dp[i][0][bottom] = swap.min(keep);
                } else {
                    dp[i][0][bottom] = swap;
                }
            }

            // The same for the bottom row
            let keep = dp[i - 1][1][bottom];
            dp[i][1][bottom] = keep;
            let swap = dp[i - 1][1][top] + 1;
            if swap != 0 {
                if top == bottom && keep != -1 {
                    dp[i][1][top] = swap.min(keep);
                } else {
                    dp[i][1][top] = swap;
                }
            }
        }

        // println!("🧮 {:#?}", dp);

        // The result is either one is smaller
        let min_top = dp.last().unwrap()[0].iter().filter(|it| **it != -1).min();
        let min_bottom = dp.last().unwrap()[1].iter().filter(|it| **it != -1).min();
        if min_top.is_some() && min_bottom.is_some() {
            return *min_top.unwrap().min(min_bottom.unwrap());
        }
        *min_top.max(min_bottom).unwrap_or(&-1)
    }
}
