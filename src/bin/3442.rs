/// Category: algorithms
/// Level: Easy
/// Percent: 48.673187%

/// You are given a string s consisting of lowercase English letters.
///
/// Your task is to find the maximum difference diff = a₁ - a₂ between the frequency of characters a₁ and a₂ in the string such that:
///
///
/// 	a₁ has an odd frequency in the string.
/// 	a₂ has an even frequency in the string.
///
///
/// Return this maximum difference.
///
///
/// Example 1:
///
///
/// Input: s = "aaaaabbc"
///
/// Output: 3
///
/// Explanation:
///
///
/// 	The character 'a' has an odd frequency of 5, and 'b' has an even frequency of 2.
/// 	The maximum difference is 5 - 2 = 3.
///
///
///
/// Example 2:
///
///
/// Input: s = "abcabcab"
///
/// Output: 1
///
/// Explanation:
///
///
/// 	The character 'a' has an odd frequency of 3, and 'c' has an even frequency of 2.
/// 	The maximum difference is 3 - 2 = 1.
///
///
///
///
/// Constraints:
///
///
/// 	3 <= s.length <= 100
/// 	s consists only of lowercase English letters.
/// 	s contains at least one character with an odd frequency and one with an even frequency.
///

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freqs = vec![0; 26];
        for b in s.bytes() {
            freqs[b as usize - 'a' as usize] += 1;
        }
        let max_odd = freqs
            .iter()
            .filter(|&&f| f % 2 == 1)
            .max()
            .expect("s contains at least one character with an odd frequency");
        let min_even = freqs
            .iter()
            .filter(|&&f| f != 0 && f % 2 == 0)
            .min()
            .expect("s contains at least one character with an even frequency");
        max_odd - min_even
    }
}
