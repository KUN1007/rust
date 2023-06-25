use std::cell::RefCell;
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
}

struct Solution;

impl Solution {
    fn traversal(root: Option<Rc<RefCell<TreeNode>>>, a: &mut Vec<i32>) {
        if let Some(node) = root {
            Self::traversal(node.borrow().left.clone(), a);
            a.push(node.borrow().val);
            Self::traversal(node.borrow().right.clone(), a);
        }
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        let mut a = Vec::new();
        Self::traversal(root, &mut a);
        a[(k - 1) as usize]
    }
}

fn main() {
    // Example usage
    let root = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    let kth_smallest = Solution::kth_smallest(root, 2);
    println!("Kth Smallest: {}", kth_smallest);
}
