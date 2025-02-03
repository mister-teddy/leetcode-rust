/// 95. Unique Binary Search Trees II
///
/// Given an integer n, return all the structurally unique BST's (binary search trees),
/// which has exactly n nodes of unique values from 1 to n. Return the answer in any order.
///
/// Example 1:
/// Input: n = 3
/// Output: [[1,null,2,null,3],[1,null,3,2],[2,1,3],[3,1,null,null,2],[3,2,null,1]]
///
/// Example 2:
/// Input: n = 1
/// Output: [[1]]
///
/// Constraints:
/// 1 <= n <= 8
struct Solution {}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

impl Solution {
    pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        // We can solve this problem using recursive/back tracking.
        Solution {}.gen(1, n)
    }

    pub fn gen(&self, from: i32, to: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
        let mut res = vec![];
        // Pick one number as the root
        for val in from..=to {
            // The smaller numbers go into the left subtree
            let lefts = self.gen(from, val - 1);
            // The larger numbers go into the right subtree
            // And the recursive algorithm will handle all possible combinations from these numbers
            let rights = self.gen(val + 1, to);
            // Save each combination tree into the result
            for left in lefts {
                for right in rights.clone() {
                    let root = TreeNode {
                        val,
                        left: left.clone(),
                        right,
                    };
                    res.push(Some(Rc::new(RefCell::new(root))));
                }
            }
        }
        if res.is_empty() {
            vec![None] // Return None here is necessary
        } else {
            res
        }
    }
}

fn main() {
    let result = Solution::generate_trees(3);
    assert_eq!(result.len(), 5);

    let result = Solution::generate_trees(1);
    assert_eq!(result.len(), 1);
}
