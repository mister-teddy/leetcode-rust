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
        // This problem can be solved with dynamic programming
        // Let dp[i] be the number of ways you can travel from 0 -> i in the shortest amount of time
        let n = n as usize;
        let mut dp = vec![1; n];

        // We also need a list to store the shortest travel time to each intersection
        // Let st[i] be the shortest amount of time to travel from 0 -> i
        let mut st = vec![u64::MAX; n];
        st[0] = 0;

        // For faster indexing, we build an adjacency matrix based on roads
        let mut matrix = vec![vec![i32::MAX; n]; n];
        for road in roads {
            let a = road[0] as usize;
            let b = road[1] as usize;
            matrix[a][b] = road[2];
            matrix[b][a] = road[2];
        }

        for i in 1..n {
            // To get to i, we can try traveling from any intersection between 0..i-1
            for j in 0..i {
                let time_travel_from_i = st[j] + matrix[i][j] as u64;
                match time_travel_from_i.cmp(&st[i]) {
                    std::cmp::Ordering::Equal => {
                        // If we find a new way and it has the same travel time, we combine the possibilities, i.e., add up the number of ways
                        dp[i] = (dp[i] + dp[j]) % (1_000_000_000 + 7);
                    }
                    std::cmp::Ordering::Less => {
                        // If we find a new way and it has a shorter travel time, we drop the old ways, and start counting from this shorter way instead
                        st[i] = time_travel_from_i;
                        dp[i] = dp[j];
                    }
                    std::cmp::Ordering::Greater => {
                        // If we find a new way but it is more time consuming, just ignore it
                    }
                }
            }
        }

        println!("{:?}", dp);
        println!("{:?}", st);

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
