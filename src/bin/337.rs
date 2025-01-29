/// 337. House Robber III
/// The thief has found himself a new place for his thievery again. There is only one entrance to this area, called root.
///
/// Besides the root, each house has one and only one parent house. After a tour, the smart thief realized that all houses in this place form a binary tree. It will automatically contact the police if two directly-linked houses were broken into on the same night.
///
/// Given the root of the binary tree, return the maximum amount of money the thief can rob without alerting the police.
///
///
/// Example 1:
///
///
/// Input: root = [3,2,3,null,3,null,1]
/// Output: 7
/// Explanation: Maximum amount of money the thief can rob = 3 + 3 + 1 = 7.
/// Example 2:
///
///
/// Input: root = [3,4,5,1,3,null,1]
/// Output: 9
/// Explanation: Maximum amount of money the thief can rob = 4 + 5 = 9.
///
/// Constraints:
///
/// The number of nodes in the tree is in the range [1, 104].
/// 0 <= Node.val <= 104
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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        // We can solve this problem using dynamic programming directly on a tree.
        // The idea is to build up the result from smaller trees, whose base cases
        // are the leaves. To start with leaves, we must use post-order traversal.
        Solution {}.traverse(&root).0
    }

    // Post-order traversal
    // In each node (house), we return two values: the maximum rob-able amount of
    // money if we DO CONSIDER (consider, not decide) robbing the current house
    // and another value for the case we DO NOT CONSIDER at all.
    pub fn traverse(&self, root: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(root) = root {
            let root = root.borrow();
            let money_left = self.traverse(&root.left);
            let money_right = self.traverse(&root.right);
            // Now we make the decision:
            // First decision: rob the current house, and skip the two sibling houses
            let first = root.val + money_left.1 + money_right.1;
            // Second decision: skip the current house, and rob two sibling houses
            let second = money_left.0 + money_right.0;
            return (
                first.max(second),
                second, // We do not consider robbing the current house at all,
                        // so we return only the second decision
            );
        } else {
            // Edge cases
            return (0, 0);
        }
    }
}

fn main() {
    // Example 1
    let root1 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root1.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root1.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root1
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root1
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(Solution::rob(root1), 7);

    // Example 2
    let root2 = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root2.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    root2.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root2
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root2
        .as_ref()
        .unwrap()
        .borrow_mut()
        .left
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root2
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right
        .as_ref()
        .unwrap()
        .borrow_mut()
        .right = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    assert_eq!(Solution::rob(root2), 9);
}
