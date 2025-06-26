/// Category: algorithms
/// Level: Hard
/// Percent: 41.455944%

/// A k-mirror number is a positive integer without leading zeros that reads the same both forward and backward in base-10 as well as in base-k.
///
///
/// 	For example, 9 is a 2-mirror number. The representation of 9 in base-10 and base-2 are 9 and 1001 respectively, which read the same both forward and backward.
/// 	On the contrary, 4 is not a 2-mirror number. The representation of 4 in base-2 is 100, which does not read the same both forward and backward.
///
///
/// Given the base k and the number n, return the sum of the n smallest k-mirror numbers.
///
///
/// Example 1:
///
/// Input: k = 2, n = 5
/// Output: 25
/// Explanation:
/// The 5 smallest 2-mirror numbers and their representations in base-2 are listed as follows:
///   base-10    base-2
///     1          1
///     3          11
///     5          101
///     7          111
///     9          1001
/// Their sum = 1 + 3 + 5 + 7 + 9 = 25.
///
///
/// Example 2:
///
/// Input: k = 3, n = 7
/// Output: 499
/// Explanation:
/// The 7 smallest 3-mirror numbers are and their representations in base-3 are listed as follows:
///   base-10    base-3
///     1          1
///     2          2
///     4          11
///     8          22
///     121        11111
///     151        12121
///     212        21212
/// Their sum = 1 + 2 + 4 + 8 + 121 + 151 + 212 = 499.
///
///
/// Example 3:
///
/// Input: k = 7, n = 17
/// Output: 20379000
/// Explanation: The 17 smallest 7-mirror numbers are:
/// 1, 2, 3, 4, 5, 6, 8, 121, 171, 242, 292, 16561, 65656, 2137312, 4602064, 6597956, 6958596
///
///
///
/// Constraints:
///
///
/// 	2 <= k <= 9
/// 	1 <= n <= 30
///
impl Solution {
    pub fn k_mirror(k: i32, n: i32) -> i64 {
        fn is_palindrome(digits: Vec<u8>) -> bool {
            let len = digits.len();
            for i in 0..len / 2 {
                if digits[i] != digits[len - 1 - i] {
                    return false;
                }
            }
            true
        }

        fn to_base_k(mut num: i64, k: i64) -> Vec<u8> {
            let mut digits = Vec::new();
            while num > 0 {
                digits.push((num % k) as u8);
                num /= k;
            }
            digits.reverse();
            digits
        }

        let mut count = 0;
        let mut sum = 0;
        let mut mi = MirrorGenerator {
            counter: 0,
            length: 0,
        };

        while count < n {
            let next_decimal_palindrome = mi.next().unwrap();
            if is_palindrome(to_base_k(next_decimal_palindrome, k as i64)) {
                sum += next_decimal_palindrome;
                count += 1;
            }
        }

        sum
    }
}

struct MirrorGenerator {
    counter: i64,
    length: u32,
}

impl Iterator for MirrorGenerator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let half_len = (self.length + 1) / 2;
        let upper_bound = 10_i64.pow(half_len);

        if self.counter + 1 == upper_bound {
            self.length += 1;
            if self.length % 2 == 0 {
                self.counter /= 10;
            }
        }
        self.counter += 1;

        let mut res = self.counter;
        let mut x = if self.length % 2 == 1 { res / 10 } else { res };
        while x > 0 {
            res = res * 10 + (x % 10);
            x /= 10;
        }

        return Some(res);
    }
}
