/// Category: algorithms
/// Level: Medium
/// Percent: 48.699276%

/// A digit string is good if the digits (0-indexed) at even indices are even and the digits at odd indices are prime (2, 3, 5, or 7).
///
///
/// 	For example, "2582" is good because the digits (2 and 8) at even positions are even and the digits (5 and 2) at odd positions are prime. However, "3245" is not good because 3 is at an even index but is not even.
///
///
/// Given an integer n, return the total number of good digit strings of length n. Since the answer may be large, return it modulo 10⁹ + 7.
///
/// A digit string is a string consisting of digits 0 through 9 that may contain leading zeros.
///
///
/// Example 1:
///
/// Input: n = 1
/// Output: 5
/// Explanation: The good numbers of length 1 are "0", "2", "4", "6", "8".
///
///
/// Example 2:
///
/// Input: n = 4
/// Output: 400
///
///
/// Example 3:
///
/// Input: n = 50
/// Output: 564908303
///
///
///
/// Constraints:
///
///
/// 	1 <= n <= 10¹⁵
///

const MODULO: i64 = 10i64.pow(9) + 7;

impl Solution {
    fn pow(x: i64, y: i64) -> i64 {
        if y == 0 {
            return 1;
        }
        if y % 2 == 1 {
            return Self::pow(x, y - 1) * x % MODULO;
        }
        return Self::pow(x * x % MODULO, y / 2);
    }

    pub fn count_good_numbers(n: i64) -> i32 {
        // With 5 possible digits in even positions
        // and 4 possible digits in odd positions
        // The total combinations are 20^(n/2) * (n%2)*5
        // Now the difficult part is how to calculate 20 to the power of 10^15
        // Using halven powering
        (Self::pow(20, n / 2) * 5_i64.pow((n % 2) as u32) % MODULO) as i32
    }
}
