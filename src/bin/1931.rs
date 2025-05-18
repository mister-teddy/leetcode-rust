/// Category: algorithms
/// Level: Hard
/// Percent: 56.859062%

/// You are given two integers m and n. Consider an m x n grid where each cell is initially white. You can paint each cell red, green, or blue. All cells must be painted.
///
/// Return the number of ways to color the grid with no two adjacent cells having the same color. Since the answer can be very large, return it modulo 10⁹ + 7.
///
///
/// Example 1:
///
/// Input: m = 1, n = 1
/// Output: 3
/// Explanation: The three possible colorings are shown in the image above.
///
///
/// Example 2:
///
/// Input: m = 1, n = 2
/// Output: 6
/// Explanation: The six possible colorings are shown in the image above.
///
///
/// Example 3:
///
/// Input: m = 5, n = 5
/// Output: 580986
///
///
///
/// Constraints:
///
///
/// 	1 <= m <= 5
/// 	1 <= n <= 1000
impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        let m = m as usize;
        let n = n as usize;
        const MODULO: i64 = 1_000_000_000 + 7;
        // We can solve this problem with dynamic programming
        // But first, we need to calculate all possible states of a column (3*2^(m-1) possible states)
        let states = calc_states(m, vec![]);

        // let dp[i][state] be the number of ways to paint upto `grid[m][i]` with column `i` ends up painted as `state`
        let mut dp = vec![vec![0; states.len()]; n];
        dp[0].fill(1);
        for i in 1..n {
            for j in 0..states.len() {
                for k in 0..states.len() {
                    if is_transformable(&states[j], &states[k]) {
                        dp[i][j] += dp[i - 1][k];
                    }
                }
                dp[i][j] %= MODULO;
            }
        }
        (dp[n - 1].iter().sum::<i64>() % MODULO) as i32
    }
}

fn calc_states(m: usize, state: Vec<u8>) -> Vec<Vec<u8>> {
    if state.len() == m {
        return vec![state];
    } else {
        let mut res = vec![];
        for i in 0..=2 {
            if i != *state.last().unwrap_or(&3) {
                let mut new_state = state.clone();
                new_state.push(i);
                res.extend(calc_states(m, new_state));
            }
        }
        res
    }
}

fn is_transformable(state1: &Vec<u8>, state2: &Vec<u8>) -> bool {
    for (i, &color) in state1.iter().enumerate() {
        if color == state2[i] {
            return false;
        }
    }
    true
}
