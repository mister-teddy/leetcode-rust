/// Category: algorithms
/// Level: Medium
/// Percent: 45.71833%

/// You are given a directed graph of n nodes numbered from 0 to n - 1, where each node has at most one outgoing edge.
///
/// The graph is represented with a given 0-indexed array edges of size n, indicating that there is a directed edge from node i to node edges[i]. If there is no outgoing edge from i, then edges[i] == -1.
///
/// You are also given two integers node1 and node2.
///
/// Return the index of the node that can be reached from both node1 and node2, such that the maximum between the distance from node1 to that node, and from node2 to that node is minimized. If there are multiple answers, return the node with the smallest index, and if no possible answer exists, return -1.
///
/// Note that edges may contain cycles.
///
///
/// Example 1:
///
/// Input: edges = [2,2,3,-1], node1 = 0, node2 = 1
/// Output: 2
/// Explanation: The distance from node 0 to node 2 is 1, and the distance from node 1 to node 2 is 1.
/// The maximum of those two distances is 1. It can be proven that we cannot get a node with a smaller maximum distance than 1, so we return node 2.
///
///
/// Example 2:
///
/// Input: edges = [1,2,-1], node1 = 0, node2 = 2
/// Output: 2
/// Explanation: The distance from node 0 to node 2 is 2, and the distance from node 2 to itself is 0.
/// The maximum of those two distances is 2. It can be proven that we cannot get a node with a smaller maximum distance than 2, so we return node 2.
///
///
///
/// Constraints:
///
///
/// 	n == edges.length
/// 	2 <= n <= 10⁵
/// 	-1 <= edges[i] < n
/// 	edges[i] != i
/// 	0 <= node1, node2 < n
///

impl Solution {
    pub fn closest_meeting_node(edges: Vec<i32>, node1: i32, node2: i32) -> i32 {
        let n = edges.len();
        let bfs = |start: usize| -> Vec<i32> {
            let mut distances = vec![-1i32; n];
            distances[start] = 0;

            let mut queue = vec![start];
            while queue.len() > 0 {
                let top = queue.pop().unwrap();
                if edges[top] > -1 {
                    let next = edges[top] as usize;
                    if distances[next] == -1 {
                        // unvisited
                        distances[next] = distances[top] + 1;
                        queue.push(next);
                    }
                }
            }

            distances
        };
        let distances1 = bfs(node1 as usize);
        let distances2 = bfs(node2 as usize);

        let res = distances1
            .iter()
            .zip(distances2.iter())
            .enumerate()
            .filter(|(_, (&d1, &d2))| d1 != -1 && d2 != -1)
            .min_by_key(|(_, (&d1, &d2))| d1.max(d2))
            .and_then(|(node, (_, _))| Some(node as i32))
            .unwrap_or(-1);

        res
    }
}
