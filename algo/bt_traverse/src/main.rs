use std::cell::RefCell;
use std::rc::Rc;

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

// 前序遍历
pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        println!("{}", node.borrow().val); // 处理当前节点
        preorder_traversal(node.borrow().left.clone()); // 递归遍历左子树
        preorder_traversal(node.borrow().right.clone()); // 递归遍历右子树
    }
}

// 中序遍历
pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        inorder_traversal(node.borrow().left.clone()); // 递归遍历左子树
        println!("{}", node.borrow().val); // 处理当前节点
        inorder_traversal(node.borrow().right.clone()); // 递归遍历右子树
    }
}

// 后序遍历
pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node) = root {
        postorder_traversal(node.borrow().left.clone()); // 递归遍历左子树
        postorder_traversal(node.borrow().right.clone()); // 递归遍历右子树
        println!("{}", node.borrow().val); // 处理当前节点
    }
}

fn main() {
    // 构建二叉树
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    root.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

    // 前序遍历
    println!("Preorder traversal:");
    preorder_traversal(root.clone());

    // 中序遍历
    println!("Inorder traversal:");
    inorder_traversal(root.clone());

    // 后序遍历
    println!("Postorder traversal:");
    postorder_traversal(root);
}
