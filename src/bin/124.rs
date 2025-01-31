/// 124. Binary Tree Maximum Path Sum
/// Solved
/// Hard

/// Topics
/// Companies
/// A path in a binary tree is a sequence of nodes where each pair of adjacent nodes in the sequence has an edge connecting them. A node can only appear in the sequence at most once. Note that the path does not need to pass through the root.

/// The path sum of a path is the sum of the node's values in the path.

/// Given the root of a binary tree, return the maximum path sum of any non-empty path.

/// Example 1:

/// Input: root = [1,2,3]
/// Output: 6
/// Explanation: The optimal path is 2 -> 1 -> 3 with a path sum of 2 + 1 + 3 = 6.
/// Example 2:

/// Input: root = [-10,9,20,null,null,15,7]
/// Output: 42
/// Explanation: The optimal path is 15 -> 20 -> 7 with a path sum of 15 + 20 + 7 = 42.

/// Constraints:

/// The number of nodes in the tree is in the range [1, 3 * 104].
/// -1000 <= Node.val <= 1000
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
    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // We can solve this problem using dynamic programming directly on a tree
        // Let f(root) be the maximum path sum for the given root. The result of the solution is f(original_root).0
        // Let f'(root) be the maximum path sum that can be connected to the parent node. The condition to be met
        // is that f' can only be built with one of the children, not both.
        // Base cases: trees with only one node => f(root) = f'(root) = root.val
        // Recursive cases:
        //   - f'(root) = root.val + max(f'(root.left), f'(root.right), 0)
        //   - f(root) = max(f(root.left), f(root.right), root.val + f'(root.left) + f'(root.right))
        Solution {}.f(&root).unwrap().0
    }

    //                                                            f    f'
    fn f(&self, root: &Option<Rc<RefCell<TreeNode>>>) -> Option<(i32, i32)> {
        if let Some(root) = root {
            let root = root.borrow();
            let left = self.f(&root.left);
            let right = self.f(&root.right);

            let mut f1 = root.val.max(0);
            let mut f0 = root.val;
            if let Some(left) = left {
                f1 = f1.max(root.val + left.1);
                f0 += left.1;
            }
            if let Some(right) = right {
                f1 = f1.max(root.val + right.1);
                f0 += right.1;
            }
            if let Some(left) = left {
                f0 = f0.max(left.0)
            }
            if let Some(right) = right {
                f0 = f0.max(right.0)
            }
            Some((f0, f1))
        } else {
            None
        }
    }
}

fn main() {
    let root1 = Some(Rc::new(RefCell::new(TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
    })));
    assert_eq!(Solution::max_path_sum(root1), 6);

    let root2 = Some(Rc::new(RefCell::new(TreeNode {
        val: -10,
        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        }))),
    })));
    assert_eq!(Solution::max_path_sum(root2), 42);

    let root3 = Some(Rc::new(RefCell::new(TreeNode {
        val: -3,
        left: None,
        right: None,
    })));
    assert_eq!(Solution::max_path_sum(root3), -3);
}
