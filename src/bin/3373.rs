/// Category: algorithms
/// Level: Hard
/// Percent: 53.825233%

/// There exist two undirected trees with n and m nodes, labeled from [0, n - 1] and [0, m - 1], respectively.
///
/// You are given two 2D integer arrays edges1 and edges2 of lengths n - 1 and m - 1, respectively, where edges1[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the first tree and edges2[i] = [ui, vi] indicates that there is an edge between nodes ui and vi in the second tree.
///
/// Node u is target to node v if the number of edges on the path from u to v is even. Note that a node is always target to itself.
///
/// Return an array of n integers answer, where answer[i] is the maximum possible number of nodes that are target to node i of the first tree if you had to connect one node from the first tree to another node in the second tree.
///
/// Note that queries are independent from each other. That is, for every query you will remove the added edge before proceeding to the next query.
///
///
/// Example 1:
///
///
/// Input: edges1 = [[0,1],[0,2],[2,3],[2,4]], edges2 = [[0,1],[0,2],[0,3],[2,7],[1,4],[4,5],[4,6]]
///
/// Output: [8,7,7,8,8]
///
/// Explanation:
///
///
/// 	For i = 0, connect node 0 from the first tree to node 0 from the second tree.
/// 	For i = 1, connect node 1 from the first tree to node 4 from the second tree.
/// 	For i = 2, connect node 2 from the first tree to node 7 from the second tree.
/// 	For i = 3, connect node 3 from the first tree to node 0 from the second tree.
/// 	For i = 4, connect node 4 from the first tree to node 4 from the second tree.
///
///
///
/// Example 2:
///
///
/// Input: edges1 = [[0,1],[0,2],[0,3],[0,4]], edges2 = [[0,1],[1,2],[2,3]]
///
/// Output: [3,6,6,6,6]
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
/// 	2 <= n, m <= 10⁵
/// 	edges1.length == n - 1
/// 	edges2.length == m - 1
/// 	edges1[i].length == edges2[i].length == 2
/// 	edges1[i] = [ai, bi]
/// 	0 <= ai, bi < n
/// 	edges2[i] = [ui, vi]
/// 	0 <= ui, vi < m
/// 	The input is generated such that edges1 and edges2 represent valid trees.
///

impl Solution {
    pub fn max_target_nodes(edges1: Vec<Vec<i32>>, edges2: Vec<Vec<i32>>) -> Vec<i32> {
        let (adj_list1, n) = edges_to_adj_list(edges1);
        let (adj_list2, m) = edges_to_adj_list(edges2);
        let tree2_even = dfs(0, &adj_list2, &mut vec![-1; m], 1);
        let tree2_max = tree2_even.max(m as i32 - tree2_even);
        let mut color_map = vec![-1; n];
        let tree1_even = dfs(0, &adj_list1, &mut color_map, 1);
        let tree1_odd = n as i32 - tree1_even;
        let res = (0..n)
            .map(|i| if color_map[i] == 1 {tree1_even} else {tree1_odd}+ tree2_max)
            .collect();
        res
    }
}

// This function will answer how many nodes are target to i
fn dfs(i: usize, adj_list: &Vec<Vec<usize>>, visited: &mut Vec<i32>, is_even: i32) -> i32 {
    visited[i] = is_even;
    // Base case: i is a leaf, which has no children
    if adj_list[i].len() == 0 {
        return is_even; // There is one target node: i itself
    }
    // Recursive case
    let mut res = is_even;
    for &neighbor in adj_list[i].iter() {
        if visited[neighbor] == -1 {
            res += dfs(neighbor, adj_list, visited, 1 - is_even);
        }
    }
    res
}

fn edges_to_adj_list(edges: Vec<Vec<i32>>) -> (Vec<Vec<usize>>, usize) {
    let n = edges.len() + 1;
    let mut list = vec![vec![]; n];
    for edge in edges {
        let a = edge[0] as usize;
        let b = edge[1] as usize;
        list[a].push(b);
        list[b].push(a);
    }
    (list, n)
}
