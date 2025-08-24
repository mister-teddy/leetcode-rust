/// Category: algorithms
/// Level: Medium
/// Percent: 68.83305%

/// You are given a 2D binary array grid. Find a rectangle with horizontal and vertical sides with the smallest area, such that all the 1's in grid lie inside this rectangle.
///
/// Return the minimum possible area of the rectangle.
///
///
/// Example 1:
///
///
/// Input: grid = [[0,1,0],[1,0,1]]
///
/// Output: 6
///
/// Explanation:
///
///
///
/// The smallest rectangle has a height of 2 and a width of 3, so it has an area of 2 * 3 = 6.
///
///
/// Example 2:
///
///
/// Input: grid = [[1,0],[0,0]]
///
/// Output: 1
///
/// Explanation:
///
///
///
/// The smallest rectangle has both height and width 1, so its area is 1 * 1 = 1.
///
///
///
/// Constraints:
///
///
/// 	1 <= grid.length, grid[i].length <= 1000
/// 	grid[i][j] is either 0 or 1.
/// 	The input is generated such that there is at least one 1 in grid.
///

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let left = grid
            .iter()
            .map(|row| row.iter().position(|cell| *cell == 1).unwrap_or(n))
            .min()
            .unwrap();
        let right = grid
            .iter()
            .map(|row| row.iter().rposition(|cell| *cell == 1).unwrap_or(0))
            .max()
            .unwrap();
        let cols: Vec<Vec<i32>> = (0..n)
            .map(|i| grid.iter().map(|row| row[i]).collect())
            .collect();
        let top = cols
            .iter()
            .map(|col| col.iter().position(|cell| *cell == 1).unwrap_or(m))
            .min()
            .unwrap();
        let bottom = cols
            .iter()
            .map(|col| col.iter().rposition(|cell| *cell == 1).unwrap_or(0))
            .max()
            .unwrap();
        ((right - left + 1) * (bottom - top + 1)) as i32
    }
}
