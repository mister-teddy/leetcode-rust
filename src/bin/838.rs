/// Category: algorithms
/// Level: Medium
/// Percent: 57.3645%

/// There are n dominoes in a line, and we place each domino vertically upright. In the beginning, we simultaneously push some of the dominoes either to the left or to the right.
///
/// After each second, each domino that is falling to the left pushes the adjacent domino on the left. Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.
///
/// When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.
///
/// For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.
///
/// You are given a string dominoes representing the initial state where:
///
///
/// 	dominoes[i] = 'L', if the ith domino has been pushed to the left,
/// 	dominoes[i] = 'R', if the ith domino has been pushed to the right, and
/// 	dominoes[i] = '.', if the ith domino has not been pushed.
///
///
/// Return a string representing the final state.
///
///
/// Example 1:
///
/// Input: dominoes = "RR.L"
/// Output: "RR.L"
/// Explanation: The first domino expends no additional force on the second domino.
///
///
/// Example 2:
///
/// Input: dominoes = ".L.R...LR..L.."
/// Output: "LL.RR.LLRRLL.."
///
///
///
/// Constraints:
///
///
/// 	n == dominoes.length
/// 	1 <= n <= 10⁵
/// 	dominoes[i] is either 'L', 'R', or '.'.
///
impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let mut state = dominoes.chars().next().unwrap();
        let mut res = String::new();
        for (i, char) in dominoes.char_indices() {
            let count = i - res.len();
            if char == 'R' {
                res += &state.to_string().repeat(count);
                state = 'R';
            }
            if char == 'L' {
                let count = count + 1;
                if state == '.' || state == 'L' {
                    res += &"L".repeat(count);
                }
                if state == 'R' {
                    res += &"R".repeat(count / 2);
                    if count % 2 == 1 {
                        res += ".";
                    }
                    res += &"L".repeat(count / 2);
                }
                state = '.';
            }
        }
        res += &state.to_string().repeat(dominoes.len() - res.len());
        res.to_string()
    }
}

struct Solution {}

fn main() {
    assert_eq!(Solution::push_dominoes("RR.L".to_string()), "RR.L"); // Example 1
    assert_eq!(
        Solution::push_dominoes(".L.R...LR..L..".to_string()),
        "LL.RR.LLRRLL.."
    ); // Example 2
    assert_eq!(Solution::push_dominoes("LR".to_string()), "LR");
    assert_eq!(Solution::push_dominoes("RL".to_string()), "RL");
    assert_eq!(Solution::push_dominoes("R.L".to_string()), "R.L");
    assert_eq!(Solution::push_dominoes("R...L".to_string()), "RR.LL");
    assert_eq!(Solution::push_dominoes("LLR".to_string()), "LLR");
    assert_eq!(Solution::push_dominoes("L.".to_string()), "L.");
}
