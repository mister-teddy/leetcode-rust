use std::collections::HashMap;

/// Category: algorithms
/// Level: Hard
/// Percent: 27.423534%

/// You are given two integers n and maxValue, which are used to describe an ideal array.
///
/// A 0-indexed integer array arr of length n is considered ideal if the following conditions hold:
///
///
/// 	Every arr[i] is a value from 1 to maxValue, for 0 <= i < n.
/// 	Every arr[i] is divisible by arr[i - 1], for 0 < i < n.
///
///
/// Return the number of distinct ideal arrays of length n. Since the answer may be very large, return it modulo 10⁹ + 7.
///
///
/// Example 1:
///
/// Input: n = 2, maxValue = 5
/// Output: 10
/// Explanation: The following are the possible ideal arrays:
/// - Arrays starting with the value 1 (5 arrays): [1,1], [1,2], [1,3], [1,4], [1,5]
/// - Arrays starting with the value 2 (2 arrays): [2,2], [2,4]
/// - Arrays starting with the value 3 (1 array): [3,3]
/// - Arrays starting with the value 4 (1 array): [4,4]
/// - Arrays starting with the value 5 (1 array): [5,5]
/// There are a total of 5 + 2 + 1 + 1 + 1 = 10 distinct ideal arrays.
///
///
/// Example 2:
///
/// Input: n = 5, maxValue = 3
/// Output: 11
/// Explanation: The following are the possible ideal arrays:
/// - Arrays starting with the value 1 (9 arrays):
///    - With no other distinct values (1 array): [1,1,1,1,1]
///    - With 2nd distinct value 2 (4 arrays): [1,1,1,1,2], [1,1,1,2,2], [1,1,2,2,2], [1,2,2,2,2]
///    - With 2nd distinct value 3 (4 arrays): [1,1,1,1,3], [1,1,1,3,3], [1,1,3,3,3], [1,3,3,3,3]
/// - Arrays starting with the value 2 (1 array): [2,2,2,2,2]
/// - Arrays starting with the value 3 (1 array): [3,3,3,3,3]
/// There are a total of 9 + 1 + 1 = 11 distinct ideal arrays.
///
///
///
/// Constraints:
///
///
/// 	2 <= n <= 10⁴
/// 	1 <= maxValue <= 10⁴

const MODULO: i32 = 1_000_000_007;

fn f(n: usize) -> usize {
    if n == 0 {
        return 1;
    }
    (n as u64 * f(n - 1) as u64 % MODULO as u64) as usize
}

fn f_range(n: usize, p: usize) -> usize {
    if p <= n {
        return n;
    }
    (p as u64 * f_range(n, p - 1) as u64 % MODULO as u64) as usize
}

fn count_factors(p: usize, n: usize) -> usize {
    if n % p != 0 {
        return 0;
    }
    1 + count_factors(p, n / p)
}

impl Solution {
    pub fn ideal_arrays_time_limit_exceeded(n: i32, max_value: i32) -> i32 {
        // We can solve this problem with dynamic programming
        // Let `dp[i][j]` be the number of distinct ideal arrays of length i, end with j
        // - `dp[1][*] = 1`
        // - `dp[i][j] = dp[i-1][k]` with all `j%k == 0`
        let n = n as usize;
        let max_value = max_value as usize;
        let mut dp = vec![vec![0 as i32; max_value + 1]; n + 1];
        dp[1].fill(1);
        for i in 1..n {
            for j in 1..=max_value {
                for k in (j..=max_value).step_by(j) {
                    dp[i + 1][k] += dp[i][j];
                    dp[i + 1][k] %= MODULO;
                }
            }
        }
        dp[n].iter().fold(0, |sum, x| (sum + x) % MODULO)
    }

    pub fn ideal_arrays(n: i32, max_value: i32) -> i32 {
        // Use sieve to count prime factors of all numbers from 1..max_value
        let n = n as usize;
        let max_value = max_value as usize;
        let mut factors: Vec<HashMap<usize, usize>> =
            (0..=max_value).map(|_| HashMap::new()).collect();
        for i in 2..=max_value {
            // prime
            if factors[i].is_empty() {
                for j in (i..=max_value).step_by(i) {
                    // update all non-prime dividents
                    *factors[j].entry(i).or_insert(0) += count_factors(i, j)
                }
            }
        }

        // sub problem: calculate dp[i][j] = i! / j! / (i-j)!
        // => 1.2.3.4...i / 1.2.3.4...j / 1.2.3.4..(i-j)
        // => dp[i][j] = dp[i-1][j]*(i+j) <=> dp[i-1][j]*i + dp[i-1][j]*j
        let mut dp = vec![vec![0; 65]; n + 64];
        dp[0][0] = 1;
        for i in 1..n + 64 {
            dp[i][0] = 1;
            for j in 1..65 {
                dp[i][j] = (dp[i - 1][j] + dp[i - 1][j - 1]) % MODULO;
            }
        }

        factors
            .iter()
            .skip(1)
            .map(|factor| {
                factor.iter().fold(1u64, |c, (_, p)| {
                    (c * dp[n + *p - 1][*p] as u64) % MODULO as u64
                })
            })
            .fold(0i32, |sum, x| (sum + x as i32) % MODULO)
    }
}
