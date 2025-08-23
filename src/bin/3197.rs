/// Category: algorithms
/// Level: Hard
/// Percent: 28.409773%

/// You are given a 2D binary array grid. You need to find 3 non-overlapping rectangles having non-zero areas with horizontal and vertical sides such that all the 1's in grid lie inside these rectangles.
///
/// Return the minimum possible sum of the area of these rectangles.
///
/// Note that the rectangles are allowed to touch.
///
///
/// Example 1:
///
///
/// Input: grid = [[1,0,1],[1,1,1]]
///
/// Output: 5
///
/// Explanation:
///
///
///
///
/// 	The 1's at (0, 0) and (1, 0) are covered by a rectangle of area 2.
/// 	The 1's at (0, 2) and (1, 2) are covered by a rectangle of area 2.
/// 	The 1 at (1, 1) is covered by a rectangle of area 1.
///
///
///
/// Example 2:
///
///
/// Input: grid = [[1,0,1,0],[0,1,0,1]]
///
/// Output: 5
///
/// Explanation:
///
///
///
///
/// 	The 1's at (0, 0) and (0, 2) are covered by a rectangle of area 3.
/// 	The 1 at (1, 1) is covered by a rectangle of area 1.
/// 	The 1 at (1, 3) is covered by a rectangle of area 1.
///
///
///
///
/// Constraints:
///
///
/// 	1 <= grid.length, grid[i].length <= 30
/// 	grid[i][j] is either 0 or 1.
/// 	The input is generated such that there are at least three 1's in grid.
///

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();

        // We can solve this problem with dynamic programming
        // Let dp[i][j][k] be the minimum total area of i rectangle(s) that cover all 1's in grid[0..j][0..k]
        // The result is dp[3][grid.length - 1][grid[0].length - 1]
        let mut dp = vec![vec![vec![0; n + 1]; m + 1]; 4];

        // dp[0], is when we don't draw any rectangle, then the area will be 0 if all cells are 0, otherwise INFINITY
        // The max size of the grid is 30*30, so max possible total area would be 600
        const INFINITY: i32 = 601;

        // We also precalculate the min and max position of the cells with value 1 in each row and each column
        let row_min: Vec<usize> = grid
            .iter()
            .map(|row| row.iter().position(|cell| *cell == 1).unwrap_or(n))
            .collect();
        let cols: Vec<Vec<i32>> = (0..n)
            .map(|i| grid.iter().map(|row| row[i]).collect())
            .collect();
        let col_min: Vec<usize> = cols
            .iter()
            .map(|col| col.iter().position(|cell| *cell == 1).unwrap_or(m))
            .collect();

        for i in 1..=m {
            for j in 1..=n {
                dp[0][i][j] = if row_min[i - 1] < i || col_min[j - 1] < j {
                    INFINITY
                } else {
                    0
                }
            }
        }

        // Now we calculate the minimum total area of rectangles by using dynamic programming
        for i in 1..=3 {
            for j in 1..=m {
                for k in 1..=n {
                    // We consider drawing in each direction
                    let mut new_dp = INFINITY;
                    // Horizontally
                    let mut kk = k;
                    let mut top = n;
                    let mut bottom = 0;
                    loop {
                        if col_min[kk - 1] < j {
                            // This column has 1
                            top = top.min(col_min[kk - 1]);
                            bottom = bottom.max(
                                cols[kk - 1]
                                    .iter()
                                    .take(j)
                                    .rposition(|cell| *cell == 1)
                                    .expect("There must be at least 1 valid cell in this column"),
                            );
                            let height = bottom - top + 1;
                            let width = k - kk + 1;
                            let area = height * width;
                            new_dp = new_dp.min(area as i32 + dp[i - 1][j][kk - 1]);
                        } else if kk == k {
                            new_dp = new_dp.min(dp[i][j][kk - 1]);
                        }
                        if kk == 1 {
                            // Try drawing the rectangle with different values of width, and height is the neccessary height that cover all 1's
                            break;
                        }
                        kk -= 1;
                    }

                    // Vertically
                    // This row has 1
                    let mut left = m;
                    let mut right = 0;
                    let mut jj = j;
                    loop {
                        if row_min[jj - 1] < k {
                            left = left.min(row_min[jj - 1]);
                            right = right.max(
                                grid[jj - 1]
                                    .iter()
                                    .take(k)
                                    .rposition(|cell| *cell == 1)
                                    .expect("There must be at least 1 valid cell in this row"),
                            );
                            let width = right - left + 1;
                            let height = j - jj + 1;
                            let area = width * height;
                            new_dp = new_dp.min(area as i32 + dp[i - 1][jj - 1][k]);
                        } else if jj == j {
                            new_dp = new_dp.min(dp[i][jj - 1][k]);
                        }
                        if jj == 1 {
                            // Try drawing the rectangle with different values of width, and height is the neccessary height that cover all 1's
                            break;
                        }
                        jj -= 1;
                    }
                    dp[i][j][k] = new_dp;
                }
            }
        }

        // The result
        dp[3][m][n]
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::minimum_sum(vec![vec![1, 0, 1], vec![1, 1, 1]]), 5);
    assert_eq!(
        Solution::minimum_sum(vec![vec![1, 0, 1, 0], vec![0, 1, 0, 1]]),
        5
    );
    assert_eq!(Solution::minimum_sum(vec![vec![1], vec![1], vec![1]]), 3);
    assert_eq!(Solution::minimum_sum(vec![vec![1, 1, 1]]), 3);
    assert_eq!(
        Solution::minimum_sum(vec![
            vec![0, 0, 0],
            vec![0, 0, 0],
            vec![1, 1, 0],
            vec![0, 1, 0]
        ]),
        3
    );
    assert_eq!(
        Solution::minimum_sum(vec![
            vec![0, 0, 0],
            vec![0, 0, 1],
            vec![0, 0, 0],
            vec![1, 0, 1]
        ]),
        3
    );
}
