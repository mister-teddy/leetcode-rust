// 38. Count and Say
// Hint
// The count-and-say sequence is a sequence of digit strings defined by the recursive formula:
// countAndSay(1) = "1"
// countAndSay(n) is the run-length encoding of countAndSay(n - 1).
// Run-length encoding (RLE) is a string compression method that works by replacing consecutive identical characters (repeated 2 or more times) with the concatenation of the character and the number marking the count of the characters (length of the run). For example, to compress the string "3322251" we replace "33" with "23", replace "222" with "32", replace "5" with "15" and replace "1" with "11". Thus the compressed string becomes "23321511".
// Given a positive integer n, return the nth element of the count-and-say sequence.

// Example 1:
// Input: n = 4
// Output: "1211"
// Explanation:
// countAndSay(1) = "1"
// countAndSay(2) = RLE of "1" = "11"
// countAndSay(3) = RLE of "11" = "21"
// countAndSay(4) = RLE of "21" = "1211"

// Example 2:
// Input: n = 1
// Output: "1"
// Explanation:
// This is the base case.

// Constraints:
// 1 <= n <= 30

// Follow up: Could you solve it iteratively?
struct Solution {}

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let n = n as usize;
        let mut res = "1".to_string();
        for _ in 0..n - 1 {
            let mut new = "".to_string();
            let mut i = 0;
            while i < res.len() {
                let mut count = 1;
                while i < res.len() - 1 && res.chars().nth(i) == res.chars().nth(i + 1) {
                    count += 1;
                    i += 1;
                }
                new = format!("{}{}{}", new, count, res.chars().nth(i).unwrap());
                i += 1;
            }
            res = new;
        }
        res
    }
}

fn main() {
    assert_eq!(Solution::count_and_say(4), "1211");
    assert_eq!(Solution::count_and_say(1), "1");
}
