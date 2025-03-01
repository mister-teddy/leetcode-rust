/// 790. Domino and Tromino Tiling
///
/// You have two types of tiles: a 2 x 1 domino shape and a tromino shape. You may rotate these shapes.
///
/// Given an integer n, return the number of ways to tile an 2 x n board. Since the answer may be very large, return it modulo 109 + 7.
///
/// In a tiling, every square must be covered by a tile. Two tilings are different if and only if there are two 4-directionally adjacent cells on the board such that exactly one of the tilings has both squares occupied by a tile.
///
/// Example 1:
/// Input: n = 3
/// Output: 5
/// Explanation: The five different ways are show above.
///
/// Example 2:
/// Input: n = 1
/// Output: 1
///
/// Constraints:
/// 1 <= n <= 1000
struct Solution {}

/// The number of ways to tile a board with different ends
#[derive(Clone, Debug)]
struct State {
    // Completed tiling
    full: u64,
    // Upper incomplete tiling
    up: u64,
    // Lower incomplete tiling
    down: u64,
}

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const MODULO: u64 = 1_000_000_007;
        // We can solve this problem with dynamic programming
        // Let dp[i] be the number of ways to tile an 2xi board
        let mut dp = vec![
            State {
                full: 1, // Although a 2x0 tiling doesn't make any sense, it can be a valid starting point to add up combinations
                up: 0,   // These are incomplete tilings, so they cannot be used as starting points
                down: 0
            };
            n as usize + 1
        ];
        for i in 2..=n as usize {
            let smaller_one = &dp[i - 1];
            let smaller_two = &dp[i - 2];
            dp[i] = State {
                // There are 4 ways to fill a complete tiling: append a vertical domino, append 2 horizontal domino, 2 ways of appending a tromino to the matching incomplete tiling
                full: (smaller_two.full + smaller_one.full + smaller_one.up + smaller_one.down)
                    % MODULO,
                // There are 2 ways to fill an upper incomplete tiling: append a horizontal domino to a lower incomplete tiling, or append a tromino to a complete tiling
                up: (smaller_one.down + smaller_two.full) % MODULO,
                // The same
                down: (smaller_one.up + smaller_two.full) % MODULO,
            };
        }
        dp.last().unwrap().full as i32
    }
}

fn main() {
    assert_eq!(Solution::num_tilings(3), 5);
    assert_eq!(Solution::num_tilings(1), 1);
    assert_eq!(Solution::num_tilings(4), 11);
    assert_eq!(Solution::num_tilings(5), 24);
    assert_eq!(Solution::num_tilings(6), 53);
    assert_eq!(Solution::num_tilings(7), 117);
    assert_eq!(Solution::num_tilings(30), 312_342_182);
}
