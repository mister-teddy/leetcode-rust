/// 2685. Count the Number of Complete Components
/// Medium
///
/// Topics
/// Companies
///
/// Hint
/// You are given an integer n. There is an undirected graph with n vertices, numbered from 0 to n - 1.
/// You are given a 2D integer array edges where edges[i] = [ai, bi] denotes that there exists an
/// undirected edge connecting vertices ai and bi.
///
/// Return the number of complete connected components of the graph.
///
/// A connected component is a subgraph of a graph in which there exists a path between any two vertices,
/// and no vertex of the subgraph shares an edge with a vertex outside of the subgraph.
///
/// A connected component is said to be complete if there exists an edge between every pair of its vertices.
///
/// Example 1:
///
/// Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4]]
/// Output: 3
/// Explanation: From the picture above, one can see that all of the components of this graph are complete.
///
/// Example 2:
///
/// Input: n = 6, edges = [[0,1],[0,2],[1,2],[3,4],[3,5]]
/// Output: 1
/// Explanation: The component containing vertices 0, 1, and 2 is complete since there is an edge between
/// every pair of two vertices. On the other hand, the component containing vertices 3, 4, and 5 is not
/// complete since there is no edge between vertices 4 and 5. Thus, the number of complete components in
/// this graph is 1.
///
/// Constraints:
///
/// 1 <= n <= 50
/// 0 <= edges.length <= n * (n - 1) / 2
/// edges[i].length == 2
/// 0 <= ai, bi <= n - 1
/// ai != bi
/// There are no repeated edges.
struct Solution {}

impl Solution {
    // This function should push all connected vertices into the &mut component vector
    fn dfs(component: &mut Vec<usize>, x: usize, matrix: &Vec<Vec<bool>>) {
        component.push(x);
        for i in 0..matrix.len() {
            if matrix[x][i] && !component.contains(&i) {
                Solution::dfs(component, i, matrix);
            }
        }
    }

    pub fn count_complete_components(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        // We can use DFS to detect connected components
        // Then count how many of them are complete connected

        // First, we build a matrix for efficient adjacent lookup
        let n = n as usize;
        let mut matrix = vec![vec![false; n]; n];
        for edge in edges {
            let a = edge[0] as usize;
            let b = edge[1] as usize;
            matrix[a][b] = true;
            matrix[b][a] = true;
        }

        // Then we build a list of connected components
        let mut check = vec![false; n];
        let mut components = vec![];
        for i in 0..n {
            if !check[i] {
                let mut component = vec![];
                Solution::dfs(&mut component, i, &matrix);
                for vertex in &component {
                    check[*vertex] = true;
                }
                components.push(component);
            }
        }

        // Finally, count how many of "completely" connected components are there
        components
            .iter()
            .filter(|component| -> bool {
                for a in component.iter() {
                    for b in component.iter() {
                        if a != b && !matrix[*a][*b] {
                            return false;
                        }
                    }
                }
                true
            })
            .count() as i32
    }
}

fn main() {
    assert_eq!(Solution::count_complete_components(1, vec![]), 1);

    assert_eq!(Solution::count_complete_components(50, vec![]), 50);

    assert_eq!(
        Solution::count_complete_components(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4]]
        ),
        3
    );

    assert_eq!(
        Solution::count_complete_components(
            6,
            vec![vec![0, 1], vec![0, 2], vec![1, 2], vec![3, 4], vec![3, 5]]
        ),
        1
    );
}
