use std::collections::BinaryHeap;

/// Category: algorithms
/// Level: Medium
/// Percent: 35.78506%

/// There is a dungeon with n x m rooms arranged as a grid.
///
/// You are given a 2D array moveTime of size n x m, where moveTime[i][j] represents the minimum time in seconds when you can start moving to that room. You start from the room (0, 0) at time t = 0 and can move to an adjacent room. Moving between adjacent rooms takes exactly one second.
///
/// Return the minimum time to reach the room (n - 1, m - 1).
///
/// Two rooms are adjacent if they share a common wall, either horizontally or vertically.
///
///
/// Example 1:
///
///
/// Input: moveTime = [[0,4],[4,4]]
///
/// Output: 6
///
/// Explanation:
///
/// The minimum time required is 6 seconds.
///
///
/// 	At time t == 4, move from room (0, 0) to room (1, 0) in one second.
/// 	At time t == 5, move from room (1, 0) to room (1, 1) in one second.
///
///
///
/// Example 2:
///
///
/// Input: moveTime = [[0,0,0],[0,0,0]]
///
/// Output: 3
///
/// Explanation:
///
/// The minimum time required is 3 seconds.
///
///
/// 	At time t == 0, move from room (0, 0) to room (1, 0) in one second.
/// 	At time t == 1, move from room (1, 0) to room (1, 1) in one second.
/// 	At time t == 2, move from room (1, 1) to room (1, 2) in one second.
///
///
///
/// Example 3:
///
///
/// Input: moveTime = [[0,1],[1,2]]
///
/// Output: 3
///
///
///
/// Constraints:
///
///
/// 	2 <= n == moveTime.length <= 50
/// 	2 <= m == moveTime[i].length <= 50
/// 	0 <= moveTime[i][j] <= 10⁹
///

impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        // We can solve this problem using Dijkstra
        let n = move_time.len();
        let m = move_time[0].len();
        let mut res = vec![vec![i32::MAX; m]; n];
        let mut queue = BinaryHeap::new();
        // We need a Priority Queue to store the next cell with the minimal total travel time.
        // Since it is a Max Heap, the first value of the tuple (will be used for comparing) will be the negative value of the minimal total travel time.
        // Other extradata include: vertical index, horizontal index, and whether the next move would take 1 or 2 seconds
        queue.push((0, (0, 0)));
        loop {
            if let Some((time, (i, j))) = queue.pop() {
                if -time < res[i][j] {
                    res[i][j] = -time;
                    if i > 0 {
                        queue.push((time.min(-move_time[i - 1][j]) - 1, (i - 1, j)));
                    }
                    if j > 0 {
                        queue.push((time.min(-move_time[i][j - 1]) - 1, (i, j - 1)));
                    }
                    if i < n - 1 {
                        queue.push((time.min(-move_time[i + 1][j]) - 1, (i + 1, j)));
                    }
                    if j < m - 1 {
                        queue.push((time.min(-move_time[i][j + 1]) - 1, (i, j + 1)));
                    }
                }
            } else {
                break;
            }
        }
        res[n - 1][m - 1]
    }
}
