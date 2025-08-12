/// Category: algorithms
/// Level: Medium
/// Percent: 33.52535%

/// Given two positive integers n and x.
///
/// Return the number of ways n can be expressed as the sum of the xth power of unique positive integers, in other words, the number of sets of unique integers [n₁, n₂, ..., nk] where n = n₁x + n₂x + ... + nkx.
///
/// Since the result can be very large, return it modulo 10⁹ + 7.
///
/// For example, if n = 160 and x = 3, one way to express n is n = 2³ + 3³ + 5³.
///
///
/// Example 1:
///
/// Input: n = 10, x = 2
/// Output: 1
/// Explanation: We can express n as the following: n = 3² + 1² = 10.
/// It can be shown that it is the only way to express 10 as the sum of the 2nd power of unique integers.
///
///
/// Example 2:
///
/// Input: n = 4, x = 1
/// Output: 2
/// Explanation: We can express n in the following ways:
/// - n = 4¹ = 4.
/// - n = 3¹ + 1¹ = 4.
///
///
///
/// Constraints:
///
///
/// 	1 <= n <= 300
/// 	1 <= x <= 5
///

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        // First we generating all possible powers <= n
        let mut powers = vec![];
        let mut p = 1i32;
        loop {
            let power = p.pow(x as u32);
            if power <= n {
                powers.push(power);
                p += 1;
            } else {
                break;
            }
        }
        // Then we solve it using dynamic programming
        count(n as usize, powers)
    }
}

const MODULO: i32 = 10i32.pow(9) + 7;

fn count(n: usize, powers: Vec<i32>) -> i32 {
    // Let dp[i][j] be the number ways to express i using powers[j-1] and optionally powers[0..j-2]
    // dp[0][0] = 1 // There is one way to express 0 from nothing
    // dp[*][*] = 0 // No way to express any number (that is not 0) using no operand
    let mut dp = vec![vec![0; powers.len() + 1]; n as usize + 1];
    dp[0][0] = 1;
    // dp[i][j] = dp[k][j-powers[i]] for all k in 0..i
    for i in 1..=n {
        for j in 1..=powers.len() {
            for k in 0..j {
                if i >= powers[j - 1] as usize {
                    dp[i][j] = (dp[i][j] + dp[i - powers[j - 1] as usize][k]) % MODULO;
                }
            }
        }
    }
    dp[n].iter().fold(0, |sum, x| (sum + x) % MODULO)
}
