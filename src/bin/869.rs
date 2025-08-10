/// Category: algorithms
/// Level: Medium
/// Percent: 62.158974%

/// You are given an integer n. We reorder the digits in any order (including the original order) such that the leading digit is not zero.
///
/// Return true if and only if we can do this so that the resulting number is a power of two.
///
///
/// Example 1:
///
/// Input: n = 1
/// Output: true
///
///
/// Example 2:
///
/// Input: n = 10
/// Output: false
///
///
///
/// Constraints:
///
///
/// 	1 <= n <= 10⁹
///

impl Solution {
    pub fn reordered_power_of2(n: i32) -> bool {
        let mut bin = 1;
        let digits = count_digits(n);
        while bin <= 10i32.pow(9) {
            if count_digits(bin) == digits {
                return true;
            }
            bin *= 2;
        }
        return false;
    }
}

fn count_digits(n: i32) -> Vec<i32> {
    let mut count = vec![0; 10];
    let mut nn = n as usize;
    while nn > 0 {
        count[nn % 10] += 1;
        nn /= 10;
    }
    count
}
