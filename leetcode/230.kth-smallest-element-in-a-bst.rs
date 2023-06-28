/*
 * @lc app=leetcode id=230 lang=rust
 *
 * [230] Kth Smallest Element in a BST
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn traverse(&self, root: Option<Rc<RefCell<TreeNode>>>, k: i32, res: &mut i32, rank: &mut i32) {
        if let Some(node) = root {
            self.traverse(node.borrow().left.clone(), k, res, rank);
            *rank += 1;
            if *rank == k {
                *res = node.borrow().val;
                return;
            }
            self.traverse(node.borrow().right.clone(), k, res, rank);
        }
    }
    fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut res = 0;
        let mut rank = 0;
        Solution.traverse(root, k, &mut res, &mut rank);
        res
    }
}
// @lc code=end
