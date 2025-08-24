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
        let mut res = 901;
        let m = grid.len();
        let n = grid[0].len();
        for i in 0..m - 1 {
            for j in 0..n - 1 {
                // Case 1: 12
                //         33
                if let Some(area1) = calc_area(sub_grid(&grid, 0, i, 0, j)) {
                    if let Some(area2) = calc_area(sub_grid(&grid, 0, i, j + 1, n - 1)) {
                        if let Some(area3) = calc_area(sub_grid(&grid, i + 1, m - 1, 0, n - 1)) {
                            res = res.min(area1 + area2 + area3);
                        }
                    }
                }

                // Case 2: 33
                //         12
                if let Some(area1) = calc_area(sub_grid(&grid, i + 1, m - 1, 0, j)) {
                    if let Some(area2) = calc_area(sub_grid(&grid, i + 1, m - 1, j + 1, n - 1)) {
                        if let Some(area3) = calc_area(sub_grid(&grid, 0, i, 0, n - 1)) {
                            res = res.min(area1 + area2 + area3);
                        }
                    }
                }

                // Case 3: 13
                //         23
                if let Some(area1) = calc_area(sub_grid(&grid, 0, i, 0, j)) {
                    if let Some(area2) = calc_area(sub_grid(&grid, i + 1, m - 1, 0, j)) {
                        if let Some(area3) = calc_area(sub_grid(&grid, 0, m - 1, j + 1, n - 1)) {
                            res = res.min(area1 + area2 + area3);
                        }
                    }
                }

                // Case 4: 31
                //         32
                if let Some(area1) = calc_area(sub_grid(&grid, 0, i, j + 1, n - 1)) {
                    if let Some(area2) = calc_area(sub_grid(&grid, i + 1, m - 1, j + 1, n - 1)) {
                        if let Some(area3) = calc_area(sub_grid(&grid, 0, m - 1, 0, j)) {
                            res = res.min(area1 + area2 + area3);
                        }
                    }
                }
            }
        }

        for i in 0..m - 2 {
            for j in i + 1..m - 1 {
                // Case 5: 1
                //         2
                //         3
                if let Some(area1) = calc_area(sub_grid(&grid, 0, i, 0, n - 1)) {
                    if let Some(area2) = calc_area(sub_grid(&grid, i + 1, j, 0, n - 1)) {
                        if let Some(area3) = calc_area(sub_grid(&grid, j + 1, m - 1, 0, n - 1)) {
                            res = res.min(area1 + area2 + area3);
                        }
                    }
                }
            }
        }

        for i in 0..n - 2 {
            for j in i + 1..n - 1 {
                // Case 6: 123
                if let Some(area1) = calc_area(sub_grid(&grid, 0, m - 1, 0, i)) {
                    if let Some(area2) = calc_area(sub_grid(&grid, 0, m - 1, i + 1, j)) {
                        if let Some(area3) = calc_area(sub_grid(&grid, 0, m - 1, j + 1, n - 1)) {
                            res = res.min(area1 + area2 + area3);
                        }
                    }
                }
            }
        }

        res
    }
}

fn sub_grid(
    grid: &Vec<Vec<i32>>,
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
) -> Vec<Vec<i32>> {
    return grid
        .iter()
        .skip(top)
        .take(bottom - top + 1)
        .map(|row| {
            row.iter()
                .skip(left)
                .take(right - left + 1)
                .map(|it| *it)
                .collect()
        })
        .collect();
}

fn calc_area(grid: Vec<Vec<i32>>) -> Option<i32> {
    let n = grid[0].len();
    let left = grid
        .iter()
        .map(|row| row.iter().position(|cell| *cell == 1))
        .filter(|pos| pos.is_some())
        .map(|pos| pos.unwrap())
        .min()?;
    let right = grid
        .iter()
        .map(|row| row.iter().rposition(|cell| *cell == 1))
        .filter(|pos| pos.is_some())
        .map(|pos| pos.unwrap())
        .max()?;
    let cols: Vec<Vec<i32>> = (0..n)
        .map(|i| grid.iter().map(|row| row[i]).collect())
        .collect();
    let top = cols
        .iter()
        .map(|col| col.iter().position(|cell| *cell == 1))
        .filter(|pos| pos.is_some())
        .map(|pos| pos.unwrap())
        .min()?;
    let bottom = cols
        .iter()
        .map(|col| col.iter().rposition(|cell| *cell == 1))
        .filter(|pos| pos.is_some())
        .map(|pos| pos.unwrap())
        .max()?;
    Some(((right - left + 1) * (bottom - top + 1)) as i32)
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
