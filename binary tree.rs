use std::rc::Rc;
use std::cell::RefCell;
use std::io;

// Definition for a binary tree node.
#[derive(Debug)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let node_ref = node.borrow();
            let left_depth = max_depth(node_ref.left.clone());
            let right_depth = max_depth(node_ref.right.clone());
            1 + left_depth.max(right_depth)
        }
        None => 0,
    }
}

fn main() {
    println!("Enter the values of the binary tree nodes separated by spaces (use -1 to denote null nodes):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let values: Vec<Option<i32>> = input.trim().split_whitespace()
        .map(|s| s.parse().ok())
        .collect();

    let root = build_tree(&values, 0);

    println!("Maximum depth of the tree: {}", max_depth(root));
}

fn build_tree(values: &[Option<i32>], index: usize) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(&Some(val)) = values.get(index) {
        let node = Rc::new(RefCell::new(TreeNode::new(val)));
        let left_index = 2 * index + 1;
        let right_index = 2 * index + 2;
        let left_child = build_tree(values, left_index);
        let right_child = build_tree(values, right_index);
        node.borrow_mut().left = left_child;
        node.borrow_mut().right = right_child;
        Some(node)
    } else {
        None
    }
}
