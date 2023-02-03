//Definition for a binary tree node.
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

/* 二叉树层次遍历
// 输入: [1,null,2,3]
//    1
//     \
//      2
//     /
//    3
 输出: [1,3,2]
*/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn level_order_bottom(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    
}
