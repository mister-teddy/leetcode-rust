/// # 2466. Count Ways To Build Good Strings
///
/// Given the integers zero, one, low, and high, we can construct a string by starting with an empty string, and then at each step perform either of the following:
/// Append the character '0' zero times.
/// Append the character '1' one times.
/// This can be performed any number of times.
/// A good string is a string constructed by the above process having a length between low and high (inclusive).
/// Return the number of different good strings that can be constructed satisfying these properties. Since the answer can be large, return it modulo 109 + 7.
///
/// Example 1:
/// Input: low = 3, high = 3, zero = 1, one = 1
/// Output: 8
/// Explanation:
/// One possible valid good string is "011".
/// It can be constructed as follows: "" -> "0" -> "01" -> "011".
/// All binary strings from "000" to "111" are good strings in this example.
///
/// Example 2:
/// Input: low = 2, high = 3, zero = 1, one = 2
/// Output: 5
/// Explanation: The good strings are "00", "11", "000", "110", and "011".
///
/// Constraints:
/// 1 <= low <= high <= 105
/// 1 <= zero, one <= low
struct Solution {}

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        // Since the answer can be large, return it modulo 109 + 7.
        let modulo = 1_000_000_007;
        // We can solve this problem with dynamic programming
        // Let dp[i] be the number of different strings with length i constructed by the above process
        let mut dp = vec![0; high as usize + 1];
        dp[zero as usize] += 1;
        dp[one as usize] += 1;
        for i in 1..dp.len() {
            // A string with length x can be constructed by appending zero(es) to the string with length x - zero
            // or appending one(s) to the string with length x - one
            dp[i] +=
                (dp[i.saturating_sub(zero as usize)] + dp[i.saturating_sub(one as usize)]) % modulo;
        }
        // Return the accumulated combinations with lengths between low and high inclusively
        dp.iter()
            .skip(low as usize)
            .take((high - low) as usize + 1)
            .fold(0, |total, x| (total + x) % modulo)
    }
}

fn main() {
    assert_eq!(Solution::count_good_strings(3, 3, 1, 1), 8);
    assert_eq!(Solution::count_good_strings(2, 3, 1, 2), 5);
}
