/// Category: algorithms
/// Level: Easy
/// Percent: 82.24361%

/// Given head which is a reference node to a singly-linked list. The value of each node in the linked list is either 0 or 1. The linked list holds the binary representation of a number.
///
/// Return the decimal value of the number in the linked list.
///
/// The most significant bit is at the head of the linked list.
///
///
/// Example 1:
///
/// Input: head = [1,0,1]
/// Output: 5
/// Explanation: (101) in base 2 = (5) in base 10
///
///
/// Example 2:
///
/// Input: head = [0]
/// Output: 0
///
///
///
/// Constraints:
///
///
/// 	The Linked List is not empty.
/// 	Number of nodes will not exceed 30.
/// 	Each node's value is either 0 or 1.
///

// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//     pub val: i32,
//     pub next: Option<Box<ListNode>>,
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode { next: None, val }
//     }
// }

// struct Solution {}

impl Solution {
    pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
        let mut res = 0;
        let mut head = head;
        loop {
            if let Some(node) = head {
                res = res * 2 + node.val;
                head = node.next;
            } else {
                break;
            }
        }
        res
    }
}
