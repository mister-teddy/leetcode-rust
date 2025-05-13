/// Category: algorithms
/// Level: Medium
/// Percent: 26.38835%

/// You are given a string s and an integer t, representing the number of transformations to perform. In one transformation, every character in s is replaced according to the following rules:
///
///
/// 	If the character is 'z', replace it with the string "ab".
/// 	Otherwise, replace it with the next character in the alphabet. For example, 'a' is replaced with 'b', 'b' is replaced with 'c', and so on.
///
///
/// Return the length of the resulting string after exactly t transformations.
///
/// Since the answer may be very large, return it modulo 10⁹ + 7.
///
///
/// Example 1:
///
///
/// Input: s = "abcyy", t = 2
///
/// Output: 7
///
/// Explanation:
///
///
/// 	First Transformation (t = 1):
///
///
/// 		'a' becomes 'b'
/// 		'b' becomes 'c'
/// 		'c' becomes 'd'
/// 		'y' becomes 'z'
/// 		'y' becomes 'z'
/// 		String after the first transformation: "bcdzz"
///
///
/// 	Second Transformation (t = 2):
///
/// 		'b' becomes 'c'
/// 		'c' becomes 'd'
/// 		'd' becomes 'e'
/// 		'z' becomes "ab"
/// 		'z' becomes "ab"
/// 		String after the second transformation: "cdeabab"
///
///
/// 	Final Length of the string: The string is "cdeabab", which has 7 characters.

///
///
///
/// Example 2:
///
///
/// Input: s = "azbk", t = 1
///
/// Output: 5
///
/// Explanation:
///
///
/// 	First Transformation (t = 1):
///
///
/// 		'a' becomes 'b'
/// 		'z' becomes "ab"
/// 		'b' becomes 'c'
/// 		'k' becomes 'l'
/// 		String after the first transformation: "babcl"
///
///
/// 	Final Length of the string: The string is "babcl", which has 5 characters.
///
///
///
///
/// Constraints:
///
///
/// 	1 <= s.length <= 10⁵
/// 	s consists only of lowercase English letters.
/// 	1 <= t <= 10⁵
///

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        fn nth(c: char) -> usize {
            (c as u8 - 'a' as u8) as usize
        }
        const MODULO: i32 = 1_000_000_000 + 7;

        // If we denote len[char][time] to be the length of the character `char` after `time` transformations,
        // "a" will become "b" after 1 transformation
        // So the length of "a" after 1 transformation is the length of "b" without transformation
        // More generally:         len[char][t] = len[char+1][t-1]
        // Edge case: "z" will become "ab" after 1 transformation
        // So:                     len['z'][t]  = len['ab'][t-1]
        // Base on the above recursive logic, We can solve this problem using dynamic programming:
        let mut len = vec![vec![1; t as usize + 1]; nth('z') + 1];
        for i in 1..=t as usize {
            for c in 'a'..='z' {
                if c == 'z' {
                    len[nth('z')][i] = (len[nth('a')][i - 1] + len[nth('b')][i - 1]) % MODULO;
                } else {
                    len[nth(c)][i] = len[nth(c) + 1][i - 1];
                }
            }
        }

        let res = s
            .chars()
            .fold(0, |res, c| (res + len[nth(c)][t as usize]) % MODULO);
        res
    }
}
