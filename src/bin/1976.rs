use std::collections::BinaryHeap;

/// 1976. Number of Ways to Arrive at Destination
/// Medium
///
/// Topics
/// Companies
///
/// Hint
/// You are in a city that consists of n intersections numbered from 0 to n - 1 with bi-directional roads between some intersections. The inputs are generated such that you can reach any intersection from any other intersection and that there is at most one road between any two intersections.
///
/// You are given an integer n and a 2D integer array roads where roads[i] = [ui, vi, timei] means that there is a road between intersections ui and vi that takes timei minutes to travel. You want to know in how many ways you can travel from intersection 0 to intersection n - 1 in the shortest amount of time.
///
/// Return the number of ways you can arrive at your destination in the shortest amount of time. Since the answer may be large, return it modulo 10^9 + 7.
///
/// # Example 1:
///
/// Input: n = 7, roads = [[0,6,7],[0,1,2],[1,2,3],[1,3,3],[6,3,3],[3,5,1],[6,5,1],[2,5,1],[0,4,5],[4,6,2]]
/// Output: 4
/// Explanation: The shortest amount of time it takes to go from intersection 0 to intersection 6 is 7 minutes.
/// The four ways to get there in 7 minutes are:
/// - 0 ➝ 6
/// - 0 ➝ 4 ➝ 6
/// - 0 ➝ 1 ➝ 2 ➝ 5 ➝ 6
/// - 0 ➝ 1 ➝ 3 ➝ 5 ➝ 6
///
/// # Example 2:
///
/// Input: n = 2, roads = [[1,0,10]]
/// Output: 1
/// Explanation: There is only one way to go from intersection 0 to intersection 1, and it takes 10 minutes.
///
/// # Constraints:
///
/// - 1 <= n <= 200
/// - n - 1 <= roads.length <= n * (n - 1) / 2
/// - roads[i].length == 3
/// - 0 <= ui, vi <= n - 1
/// - 1 <= timei <= 10^9
/// - ui != vi
/// - There is at most one road connecting any two intersections.
/// - You can reach any intersection from any other intersection.
struct Solution {}

impl Solution {
    pub fn count_paths(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        // For faster indexing, we build an adjacency matrix based on roads
        let n = n as usize;
        let mut matrix = vec![vec![0; n]; n];
        for road in roads {
            let a = road[0] as usize;
            let b = road[1] as usize;
            matrix[a][b] = road[2];
            matrix[b][a] = road[2];
        }

        // Let dp[i] be the number of ways you can travel from 0 -> i in the shortest amount of time
        let mut dp = vec![1; n];
        // Let times[i] be the shortest travel time from 0 to i
        let mut times = vec![i64::MAX; n];
        times[0] = 0;
        // If finished[i] is true, times[i] is final, which means it cannot be any smaller
        let mut finished = vec![false; n];
        // The priority queue to implement Dijkstra algorithm
        let mut queue = BinaryHeap::from([(0, 0)]);
        while !queue.is_empty() {
            if let Some((_, current)) = queue.pop() {
                if !finished[current] {
                    finished[current] = true;
                    for next in 0..n {
                        if matrix[current][next] > 0 {
                            let new_time = times[current] + matrix[current][next] as i64;
                            match new_time.cmp(&times[next]) {
                                std::cmp::Ordering::Less => {
                                    times[next] = new_time;
                                    // Reset counter
                                    dp[next] = dp[current];
                                    queue.push((-new_time, next));
                                }
                                std::cmp::Ordering::Equal => {
                                    // Add up possibilities
                                    dp[next] = (dp[next] + dp[current]) % (1_000_000_000 + 7);
                                }
                                std::cmp::Ordering::Greater => {}
                            }
                        }
                    }
                }
            }
        }

        dp[n - 1]
    }
}

fn main() {
    let result1 = Solution::count_paths(
        7,
        vec![
            vec![0, 6, 7],
            vec![0, 1, 2],
            vec![1, 2, 3],
            vec![1, 3, 3],
            vec![6, 3, 3],
            vec![3, 5, 1],
            vec![6, 5, 1],
            vec![2, 5, 1],
            vec![0, 4, 5],
            vec![4, 6, 2],
        ],
    );
    assert_eq!(result1, 4);

    let result2 = Solution::count_paths(2, vec![vec![1, 0, 10]]);
    assert_eq!(result2, 1);

    let result3 = Solution::count_paths(
        6,
        vec![
            vec![3, 0, 4],
            vec![0, 2, 3],
            vec![1, 2, 2],
            vec![4, 1, 3],
            vec![2, 5, 5],
            vec![2, 3, 1],
            vec![0, 4, 1],
            vec![2, 4, 6],
            vec![4, 3, 1],
        ],
    );
    assert_eq!(result3, 2);
}
