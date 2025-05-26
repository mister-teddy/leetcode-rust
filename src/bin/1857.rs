use std::str::{MatchIndices, Matches};

/// Category: algorithms
/// Level: Hard
/// Percent: 49.462482%

/// There is a directed graph of n colored nodes and m edges. The nodes are numbered from 0 to n - 1.
///
/// You are given a string colors where colors[i] is a lowercase English letter representing the color of the ith node in this graph (0-indexed). You are also given a 2D array edges where edges[j] = [aj, bj] indicates that there is a directed edge from node aj to node bj.
///
/// A valid path in the graph is a sequence of nodes x₁ -> x₂ -> x₃ -> ... -> xk such that there is a directed edge from xi to xi+1 for every 1 <= i < k. The color value of the path is the number of nodes that are colored the most frequently occurring color along that path.
///
/// Return the largest color value of any valid path in the given graph, or -1 if the graph contains a cycle.
///
///
/// Example 1:
///
///
///
/// Input: colors = "abaca", edges = [[0,1],[0,2],[2,3],[3,4]]
/// Output: 3
/// Explanation: The path 0 -> 2 -> 3 -> 4 contains 3 nodes that are colored "a" (red in the above image).
///
///
/// Example 2:
///
///
///
/// Input: colors = "a", edges = [[0,0]]
/// Output: -1
/// Explanation: There is a cycle from 0 to 0.
///
///
///
/// Constraints:
///
///
/// 	n == colors.length
/// 	m == edges.length
/// 	1 <= n <= 10⁵
/// 	0 <= m <= 10⁵
/// 	colors consists of lowercase English letters.
/// 	0 <= aj, bj < n
#[derive(Debug, Default, Clone)]
enum State {
    #[default]
    Untraversed,
    Traversing,
    Traversed([i32; 26]), // Travelsal result is the largest color value for each color with all paths starting from this node
    Cycle,
}

impl Solution {
    pub fn largest_path_value(colors: String, edges: Vec<Vec<i32>>) -> i32 {
        // We can solve this problem using DFS
        let n = colors.len();
        let nodes: Vec<u8> = colors.bytes().map(|b| b - 'a' as u8).collect();

        // For efficient traversal, we will use an adjacency listi
        let mut adj_list = vec![vec![]; n];
        for edge in edges {
            adj_list[edge[0] as usize].push(edge[1] as usize);
        }
        let mut states = vec![State::Untraversed; n];

        fn dfs(i: usize, nodes: &Vec<u8>, matrix: &Vec<Vec<usize>>, states: &mut Vec<State>) {
            match states[i] {
                State::Untraversed => {
                    states[i] = State::Traversing;
                    let color = nodes[i];
                    let mut color_values = [0; 26];
                    if matrix[i].len() == 0 {
                        color_values[color as usize] = 1;
                        states[i] = State::Traversed(color_values);
                    }
                    for &j in &matrix[i] {
                        // Recurse for each adjacenct node
                        dfs(j, nodes, matrix, states);
                        if let State::Traversed(values) = states[j] {
                            for (i, v) in values.iter().enumerate() {
                                color_values[i] = color_values[i]
                                    .max(v + if i == color as usize { 1 } else { 0 });
                                // Then merge back the result
                            }
                        } else {
                            states[i] = State::Cycle;
                            return;
                        }
                    }
                    states[i] = State::Traversed(color_values);
                }
                State::Traversing => {
                    states[i] = State::Cycle;
                }
                _ => {} // Just skip it if the node is already traversed (cached) or have a cycle (quick return)
            }
        }
        let mut res = 0;
        for i in 0..n {
            dfs(i, &nodes, &adj_list, &mut states);
            if let State::Traversed(color_values) = states[i] {
                res = res.max(*color_values.iter().max().unwrap()) // The result to this problem is the largest color value for any node
            } else {
                return -1;
            }
        }
        res
    }
}
