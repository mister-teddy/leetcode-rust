use std::cell;

/// Category: algorithms
/// Level: Hard
/// Percent: 63.73452%

/// Write a program to solve a Sudoku puzzle by filling the empty cells.
///
/// A sudoku solution must satisfy all of the following rules:
///
///
/// 	Each of the digits 1-9 must occur exactly once in each row.
/// 	Each of the digits 1-9 must occur exactly once in each column.
/// 	Each of the digits 1-9 must occur exactly once in each of the 9 3x3 sub-boxes of the grid.
///
///
/// The '.' character indicates empty cells.
///
///
/// Example 1:
///
/// Input: board = [["5","3",".",".","7",".",".",".","."],["6",".",".","1","9","5",".",".","."],[".","9","8",".",".",".",".","6","."],["8",".",".",".","6",".",".",".","3"],["4",".",".","8",".","3",".",".","1"],["7",".",".",".","2",".",".",".","6"],[".","6",".",".",".",".","2","8","."],[".",".",".","4","1","9",".",".","5"],[".",".",".",".","8",".",".","7","9"]]
/// Output: [["5","3","4","6","7","8","9","1","2"],["6","7","2","1","9","5","3","4","8"],["1","9","8","3","4","2","5","6","7"],["8","5","9","7","6","1","4","2","3"],["4","2","6","8","5","3","7","9","1"],["7","1","3","9","2","4","8","5","6"],["9","6","1","5","3","7","2","8","4"],["2","8","7","4","1","9","6","3","5"],["3","4","5","2","8","6","1","7","9"]]
/// Explanation: The input board is shown above and the only valid solution is shown below:
///
///
///
///
///
/// Constraints:
///
///
/// 	board.length == 9
/// 	board[i].length == 9
/// 	board[i][j] is a digit or '.'.
/// 	It is guaranteed that the input board has only one solution.
///
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        // We can solve this problem with backtracking
        fn backtrack(board: &mut Vec<Vec<char>>) -> bool {
            // Find first empty cell
            for i in 0..9 {
                for j in 0..9 {
                    if board[i][j] == '.' {
                        // Try numbers 1-9
                        for x in '1'..='9' {
                            if is_valid(board, x, i, j) {
                                board[i][j] = x;
                                if backtrack(board) {
                                    return true;
                                }
                                // Backtrack: reset the cell
                                board[i][j] = '.';
                            }
                        }
                        return false; // No valid number found
                    }
                }
            }
            true // Board is complete
        }

        backtrack(board);
    }
}

fn is_valid(board: &Vec<Vec<char>>, x: char, i: usize, j: usize) -> bool {
    // Check rows
    for j in 0..9 {
        if board[i][j] == x {
            return false;
        }
    }
    // Check cols
    for i in 0..9 {
        if board[i][j] == x {
            return false;
        }
    }
    for ii in 0..3 {
        for jj in 0..3 {
            if board[(i / 3) * 3 + ii][(j / 3) * 3 + jj] == x {
                return false;
            }
        }
    }

    return true;
}
