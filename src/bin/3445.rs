use std::{char, vec};

/// Category: algorithms
/// Level: Hard
/// Percent: 18.329746%

/// You are given a string s and an integer k. Your task is to find the maximum difference between the frequency of two characters, freq[a] - freq[b], in a substring subs of s, such that:
///
///
/// 	subs has a size of at least k.
/// 	Character a has an odd frequency in subs.
/// 	Character b has an even frequency in subs.
///
///
/// Return the maximum difference.
///
/// Note that subs can contain more than 2 distinct characters.
///
///
/// Example 1:
///
///
/// Input: s = "12233", k = 4
///
/// Output: -1
///
/// Explanation:
///
/// For the substring "12233", the frequency of '1' is 1 and the frequency of '3' is 2. The difference is 1 - 2 = -1.
///
///
/// Example 2:
///
///
/// Input: s = "1122211", k = 3
///
/// Output: 1
///
/// Explanation:
///
/// For the substring "11222", the frequency of '2' is 3 and the frequency of '1' is 2. The difference is 3 - 2 = 1.
///
///
/// Example 3:
///
///
/// Input: s = "110", k = 3
///
/// Output: -1
///
///
///
/// Constraints:
///
///
/// 	3 <= s.length <= 3 * 10⁴
/// 	s consists only of digits '0' to '4'.
/// 	The input is generated that at least one substring has a character with an even frequency and a character with an odd frequency.
/// 	1 <= k <= s.length
///

impl Solution {
    pub fn max_difference(s: String, k: i32) -> i32 {
        // We can solve this problem using 2 pointers
        let mut res = i32::MIN;
        let chars: Vec<char> = s.chars().collect();

        // This function will encode the parities of the number of character a and the number of character b in the window as a 2-bit number
        fn state(c1: i32, c2: i32) -> usize {
            (((c1 & 1) << 1) | (c2 & 1)) as usize
        }

        for a in '0'..='4' {
            for b in '0'..='4' {
                if a == b {
                    continue;
                }
                // They are the number of characters a, b from the beginning of the string to right, and then left, respectively
                let mut count = (0, 0, 0, 0);
                let mut left = 0;
                let mut best = [i32::MAX; 4];
                for (right, &c) in chars.iter().enumerate() {
                    match c {
                        x if x == a => count.0 += 1,
                        x if x == b => count.1 += 1,
                        _ => {}
                    }
                    // Increase the left pointer as long as it keep the window at least size k, and exclude the case where there is no b
                    while right - left + 1 >= k as usize && count.1 - count.3 >= 2 {
                        let left_state = state(count.2, count.3);
                        // Since the result will be right_a - right_b - (left_a - left_b), and any index between 0 and start can work with right,
                        // we keep the smallest (left_a - left_b) possible
                        best[left_state] = best[left_state].min(count.2 - count.3);
                        match chars[left] {
                            x if x == a => count.2 += 1,
                            x if x == b => count.3 += 1,
                            _ => {}
                        }
                        left += 1;
                    }
                    let right_state = state(count.0, count.1);
                    // Find the matching left_state, as we need to keep the number of a odd, and the number of b even
                    let matching_left_state = right_state ^ 0b10;
                    // Make sure that there is actually a left endpoint
                    if best[matching_left_state] != i32::MAX {
                        res = res.max(count.0 - count.1 - best[matching_left_state]);
                    }
                }
            }
        }
        res
    }
}
