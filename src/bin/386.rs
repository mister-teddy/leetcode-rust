/// Category: algorithms
/// Level: Medium
/// Percent: 73.058556%

/// Given an integer n, return all the numbers in the range [1, n] sorted in lexicographical order.
///
/// You must write an algorithm that runs in O(n) time and uses O(1) extra space.
///
///
/// Example 1:
/// Input: n = 13
/// Output: [1,10,11,12,13,2,3,4,5,6,7,8,9]
/// Example 2:
/// Input: n = 2
/// Output: [1,2]
///
///
/// Constraints:
///
///
/// 	1 <= n <= 5 * 10⁴
///

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        fn recurse(prefix: i32, n: i32) -> Vec<i32> {
            let mut res = vec![];
            for i in if prefix == 0 { 1 } else { 0 }..=9 {
                let num = prefix * 10 + i;
                if num <= n {
                    res.push(num);
                    res.append(&mut recurse(num, n));
                }
            }
            res
        }
        recurse(0, n)
    }
}
