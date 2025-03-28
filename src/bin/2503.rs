/// 2503. Maximum Number of Points From Grid Queries
/// Hard
///
/// Topics
/// Companies
///
/// Hint
/// You are given an m x n integer matrix grid and an array queries of size k.
///
/// Find an array answer of size k such that for each integer queries[i] you start in the top left cell of the matrix and repeat the following process:
///
/// If queries[i] is strictly greater than the value of the current cell that you are in, then you get one point if it is your first time visiting this cell, and you can move to any adjacent cell in all 4 directions: up, down, left, and right.
/// Otherwise, you do not get any points, and you end this process.
/// After the process, answer[i] is the maximum number of points you can get. Note that for each query you are allowed to visit the same cell multiple times.
///
/// Return the resulting array answer.
///
/// # Example 1:
///
/// Input: grid = [[1,2,3],[2,5,7],[3,5,1]], queries = [5,6,2]
/// Output: [5,8,1]
/// Explanation: The diagrams above show which cells we visit to get points for each query.
///
/// # Example 2:
///
/// Input: grid = [[5,2,1],[1,1,2]], queries = [3]
/// Output: [0]
/// Explanation: We can not get any points because the value of the top left cell is already greater than or equal to 3.
///
/// # Constraints:
///
/// - m == grid.length
/// - n == grid[i].length
/// - 2 <= m, n <= 1000
/// - 4 <= m * n <= 10^5
/// - k == queries.length
/// - 1 <= k <= 10^4
/// - 1 <= grid[i][j], queries[i] <= 10^6
struct Solution {}

use std::collections::{BinaryHeap, HashMap};

impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        // We can reply to the queries in ascending order
        let mut sorted_queries = queries.clone();
        sorted_queries.sort();

        // That way, we can traverse the adjacent cells using BFS "incrementally"
        // So we need a priority queue, which will process the smaller cells first
        // Each element in a queue is a tuple of 3 number: (v, x, y), with v = -grid[x][y]
        // (so that we pop the smaller cells first)
        let mut queue = BinaryHeap::new();
        queue.push((-grid[0][0], 0, 0));

        // Then we process the BFS
        let mut score = 0;
        let mut scores = HashMap::new();
        let mut checked = vec![vec![false; grid[0].len()]; grid.len()];
        sorted_queries.iter().for_each(|query| {
            while let Some((v, _, _)) = queue.peek() {
                if -v < *query {
                    // If the current query can reach this cell, we continue BFS-ing
                    let (_, x, y) = queue.pop().unwrap();
                    if !checked[x][y] {
                        checked[x][y] = true;
                        score += 1;
                        if x > 0 {
                            queue.push((-grid[x - 1][y], x - 1, y));
                        }
                        if x < grid.len() - 1 {
                            queue.push((-grid[x + 1][y], x + 1, y));
                        }
                        if y > 0 {
                            queue.push((-grid[x][y - 1], x, y - 1));
                        }
                        if y < grid[0].len() - 1 {
                            queue.push((-grid[x][y + 1], x, y + 1));
                        }
                    }
                } else {
                    // If the current query cannot reach this cell, we have found the answer for this query
                    break;
                }
            }
            scores.insert(query, score);
        });

        queries.iter().map(|query| scores[query]).collect()
    }
}

fn main() {
    // Example 1
    let grid1 = vec![vec![1, 2, 3], vec![2, 5, 7], vec![3, 5, 1]];
    let queries1 = vec![5, 6, 2];
    let expected1 = vec![5, 8, 1];
    assert_eq!(Solution::max_points(grid1, queries1), expected1);

    // Example 2
    let grid2 = vec![vec![5, 2, 1], vec![1, 1, 2]];
    let queries2 = vec![3];
    let expected2 = vec![0];
    assert_eq!(Solution::max_points(grid2, queries2), expected2);
}
