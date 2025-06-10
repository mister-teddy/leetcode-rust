/// Category: algorithms
/// Level: Hard
/// Percent: 41.97567%

/// Given two integers n and k, return the kth lexicographically smallest integer in the range [1, n].
///
///
/// Example 1:
///
/// Input: n = 13, k = 2
/// Output: 10
/// Explanation: The lexicographical order is [1, 10, 11, 12, 13, 2, 3, 4, 5, 6, 7, 8, 9], so the second smallest number is 10.
///
///
/// Example 2:
///
/// Input: n = 1, k = 1
/// Output: 1
///
///
///
/// Constraints:
///
///
/// 	1 <= k <= n <= 10⁹
///
fn count_children_of_prefix(prefix: i32, n: i32) -> i32 {
    let mut res = 0;
    let mut left = prefix as i64;
    let mut right = left + 1;
    let n = n as i64;
    while left <= n {
        res += right - left;
        left *= 10;
        right *= 10;
        right = right.min(n + 1);
    }
    res as i32
}

fn find_k_from(start: i32, n: i32, k: i32) -> i32 {
    if k == 1 {
        return start;
    }
    let count = count_children_of_prefix(start, n);
    if k <= count {
        return find_k_from(start * 10, n, k - 1);
    }
    return find_k_from(start + 1, n, k - count);
}

impl Solution {
    pub fn find_kth_number(n: i32, k: i32) -> i32 {
        find_k_from(1, n, k)
    }
}

// struct Solution {}

// fn main() {
//     assert_eq!(count_children_of_prefix(1, 10), 2);
//     assert_eq!(count_children_of_prefix(1, 13), 5);
//     assert_eq!(count_children_of_prefix(1, 230), 1 + 10 + 100);
//     assert_eq!(count_children_of_prefix(12, 1000), 1 + 10);

//     // Check the main function
//     assert_eq!(Solution::find_kth_number(1000, 1000), 999);
// }
