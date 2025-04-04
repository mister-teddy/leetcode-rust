// 1123. Lowest Common Ancestor of Deepest Leaves
//
// Given the root of a binary tree, return the lowest common ancestor of its deepest leaves.

// Recall that:
// The node of a binary tree is a leaf if and only if it has no children
// The depth of the root of the tree is 0. if the depth of a node is d, the depth of each of its children is d + 1.
// The lowest common ancestor of a set S of nodes, is the node A with the largest depth such that every node in S is in the subtree with root A.

// Example 1:
// Input: root = [3,5,1,6,2,0,8,null,null,7,4]
// Output: [2,7,4]
// Explanation: We return the node with value 2, colored in yellow in the diagram.
// The nodes coloured in blue are the deepest leaf-nodes of the tree.
// Note that nodes 6, 0, and 8 are also leaf nodes, but the depth of them is 2, but the depth of nodes 7 and 4 is 3.
//
// Example 2:
// Input: root = [1]
// Output: [1]
// Explanation: The root is the deepest node in the tree, and it's the lca of itself.
//
// Example 3:
// Input: root = [0,1,3,null,2]
// Output: [2]
// Explanation: The deepest leaf node in the tree is 2, the lca of one node is itself.

// Constraints:
// The number of nodes in the tree will be in the range [1, 1000].
// 0 <= Node.val <= 1000
// The values of the nodes in the tree are unique.
struct Solution {}

use core::panic;
use std::cell::RefCell;
use std::num::ParseIntError;
use std::rc::Rc;

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

    /// Constructs a binary tree from a level-order Vec<Option<i32>>
    pub fn from_vec(data: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
        if data.is_empty() || data[0].is_none() {
            return None;
        }

        let nodes: Vec<Option<Rc<RefCell<TreeNode>>>> = data
            .iter()
            .map(|&x| x.map(|v| Rc::new(RefCell::new(TreeNode::new(v)))))
            .collect();

        let mut i = 1;
        for node in nodes.iter().flatten() {
            if i < nodes.len() {
                node.borrow_mut().left = nodes[i].clone();
                i += 1;
            }
            if i < nodes.len() {
                node.borrow_mut().right = nodes[i].clone();
                i += 1;
            }
        }

        nodes[0].clone()
    }
}

impl Solution {
    pub fn lca_deepest_leaves(
        root: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut queue = vec![root.unwrap()];
        let mut first = None;
        let mut last = None;
        let mut parents = vec![None; 1001];
        while !queue.is_empty() {
            first = None;
            last = None;
            let mut layer = vec![];
            while let Some(node) = queue.pop() {
                if first.is_none() {
                    first.replace(node.clone());
                }
                last.replace(node.clone());
                if let Some(left) = &node.borrow().left {
                    layer.push(left.clone());
                    parents[left.borrow().val as usize].replace(node.clone());
                }
                if let Some(right) = &node.borrow().right {
                    layer.push(right.clone());
                    parents[right.borrow().val as usize].replace(node.clone());
                }
            }
            queue.extend(layer);
        }

        loop {
            let first_val = first.as_ref().unwrap().borrow().val;
            let last_val = last.as_ref().unwrap().borrow().val;
            if first_val == last_val {
                break;
            }
            first.replace(parents[first_val as usize].as_ref().unwrap().clone());
            last.replace(parents[last_val as usize].as_ref().unwrap().clone());
        }
        first
    }
}

fn main() {
    // Example 1
    let root = TreeNode::from_vec(vec![
        Some(3),
        Some(5),
        Some(1),
        Some(6),
        Some(2),
        Some(0),
        Some(8),
        None,
        None,
        Some(7),
        Some(4),
    ]);
    assert_eq!(Solution::lca_deepest_leaves(root).unwrap().borrow().val, 2);

    // Example 2
    let root = TreeNode::from_vec(vec![Some(1)]);
    assert_eq!(Solution::lca_deepest_leaves(root).unwrap().borrow().val, 1);

    // Example 3
    let root = TreeNode::from_vec(vec![Some(0), Some(1), Some(3), None, Some(2)]);
    assert_eq!(Solution::lca_deepest_leaves(root).unwrap().borrow().val, 2);
}
