/// # 712. Minimum ASCII Delete Sum for Two Strings
/// Given two strings s1 and s2, return the lowest ASCII sum of deleted characters to make two strings equal.
///
/// ## Example 1:
/// Input: s1 = "sea", s2 = "eat"
/// Output: 231
/// Explanation: Deleting "s" from "sea" adds the ASCII value of "s" (115) to the sum.
/// Deleting "t" from "eat" adds 116 to the sum.
/// At the end, both strings are equal, and 115 + 116 = 231 is the minimum sum possible to achieve this.
///
/// ## Example 2:
/// Input: s1 = "delete", s2 = "leet"
/// Output: 403
/// Explanation: Deleting "dee" from "delete" to turn the string into "let",
/// adds 100[d] + 101[e] + 101[e] to the sum.
/// Deleting "e" from "leet" adds 101[e] to the sum.
/// At the end, both strings are equal to "let", and the answer is 100+101+101+101 = 403.
/// If instead we turned both strings into "lee" or "eet", we would get answers of 433 or 417, which are higher.
///
/// ## Constraints:
/// 1 <= s1.length, s2.length <= 1000
/// s1 and s2 consist of lowercase English letters.
pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
    let mut dp: Vec<i32> = vec![0; s2.len() + 1];
    let bytes1 = s1.into_bytes();
    let bytes2 = s2.into_bytes();

    for j in 1..=bytes2.len() {
        dp[j] = bytes2[j - 1] as i32 + dp[j - 1];
    }

    for i in 1..=bytes1.len() {
        let mut tmp = dp.clone();
        tmp[0] += bytes1[i - 1] as i32;
        for j in 1..=bytes2.len() {
            let mut val = if bytes1[i - 1] == bytes2[j - 1] {
                dp[j - 1]
            } else {
                dp[j - 1] + bytes1[i - 1] as i32 + bytes2[j - 1] as i32
            };
            val = core::cmp::min(val, dp[j] + bytes1[i - 1] as i32);
            val = core::cmp::min(val, tmp[j - 1] + bytes2[j - 1] as i32);
            tmp[j] = val;
        }
        dp = tmp;
    }
    dp.last().unwrap().clone()
}

fn main() {
    assert_eq!(
        minimum_delete_sum("sea".to_string(), "eat".to_string()),
        231
    );
    assert_eq!(
        minimum_delete_sum("delete".to_string(), "leet".to_string()),
        403
    );
    assert_eq!(minimum_delete_sum("s".to_string(), "s".to_string()), 0);
    assert_eq!(minimum_delete_sum("a".repeat(1000), "a".repeat(1000)), 0);
    assert_eq!(
        minimum_delete_sum("a".repeat(1000), "b".repeat(1000)),
        'a' as i32 * 1000 + 'b' as i32 * 1000
    );
}
