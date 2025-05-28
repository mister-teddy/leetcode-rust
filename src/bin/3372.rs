/// Category: algorithms
/// Level: Medium
/// Percent: 47.972633%

/// There exist two undirected trees with n and m nodes, with distinct labels in ranges [0, n - 1] and [0, m - 1], respectively.
///
/// You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1, respectively, where edges1[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the first tree and edges2[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the second tree. You are also given an integer k.
///
/// Node u is target to node v if the number of edges on the path from u to v is less than or equal to k. Note that a node is always target to itself.
///
/// Return an array of n integers answer, where answer[i] is the maximum possible number of nodes target to node i of the first tree if you have to connect one node from the first tree to another node in the second tree.
///
/// Note that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.
///
///
/// Example 1:
///
///
/// Input: edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]], k = 2
///
/// Output: [9,7,9,8,8]
///
/// Explanation:
///
///
/// 	For i = 0, connect node 0 from the first tree to node 0 from the second tree.
/// 	For i = 1, connect node 1 from the first tree to node 0 from the second tree.
/// 	For i = 2, connect node 2 from the first tree to node 4 from the second tree.
/// 	For i = 3, connect node 3 from the first tree to node 4 from the second tree.
/// 	For i = 4, connect node 4 from the first tree to node 4 from the second tree.
///
///
///
/// Example 2:
///
///
/// Input: edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]], k = 1
///
/// Output: [6,3,3,3,3]
///
/// Explanation:
///
/// For every i, connect node i of the first tree with any node of the second tree.
///
///
///
/// Constraints:
///
///
/// 	2 <= n, m <= 1000
/// 	edges1.length == n - 1
/// 	edges2.length == m - 1
/// 	edges1[i].length == edges2[i].length == 2
/// 	edges1[i] = [ai, bi]
/// 	0 <= ai, bi < n
/// 	edges2[i] = [ui, vi]
/// 	0 <= ui, vi < m
/// 	The input is generated such that edges1 and edges2 represent valid trees.
/// 	0 <= k <= 1000
///
fn bfs(i: usize, adj_list: &Vec<Vec<usize>>, k: i32) -> i32 {
    let mut res = 0; // The result is the arget nodes count
    let mut queue = vec![i];
    let mut visited = vec![false; adj_list.len()];
    let mut k = k;
    while k >= 0 && !queue.is_empty() {
        let mut new_queue = vec![];
        for current in queue {
            if !visited[current] {
                visited[current] = true;
                res += 1;
                new_queue.extend(adj_list[current].clone());
            }
        }
        queue = new_queue;
        k -= 1;
    }
    res
}

fn edges_to_adj_list(edges: Vec<Vec<i32>>) -> Vec<Vec<usize>> {
    let mut list = vec![];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        if list.len() <= a {
            list.resize(a + 1, vec![]);
        }
        list[a].push(b);
        if list.len() <= b {
            list.resize(b + 1, vec![]);
        }
        list[b].push(a);
    }
    list
}

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>, k: i32) -> Vec<i32> {
        let adj_list1 = edges_to_adj_list(edges1);
        let adj_list2 = edges_to_adj_list(edges2);
        let max_target_nodes_from_2nd_tree = (0..adj_list2.len())
            .map(|i| bfs(i, &adj_list2, k - 1))
            .max()
            .unwrap();
        let res = (0..adj_list1.len())
            .map(|i| bfs(i, &adj_list1, k) + max_target_nodes_from_2nd_tree)
            .collect();
        res
    }
}
