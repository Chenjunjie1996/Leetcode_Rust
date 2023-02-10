/*
Rc(Reference Counted)
用来突破单一所有权的限制。其基本操作是通过clone()增加引用计数。
let i1 = Rc::new(1);
let i2 = i1.clone();
let i3 = i1.clone()
pritnln!("{}", Rc::strong_count(&i1)) // 3

RefCell，提供内部包装类型的内部可变性，用来突破mut变量才能被修改（外部可变性）的限制
let cell = RefCell::new(10);
{
    let mut mut_ref = cell.borrow_mut();
    *mut_ref += 1
}
println!("{}",cell.borrow()) // 11
*/
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

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

/*
        1
       / \
      2   3
     /\  /\
    4 5 6 7  
如果你按照 根节点 -> 左孩子 -> 右孩子 的方式遍历，即「先序遍历」，每次先遍历根节点，遍历结果为 1 2 4 5 3 6 7；

同理，如果你按照 左孩子 -> 根节点 -> 右孩子 的方式遍历，即「中序序遍历」，遍历结果为 4 2 5 1 6 3 7；

如果你按照 左孩子 -> 右孩子 -> 根节点 的方式遍历，即「后序序遍历」，遍历结果为 4 5 2 6 7 3 1；

最后，层次遍历就是按照每一层从左向右的方式进行遍历，遍历结果为 1 2 3 4 5 6 7。

*/
struct Solution{}

impl Solution {
    // 迭代法
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;
        while stack.len()>0 || 
    }

    // 递归法
    pub fn preorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::traverse(&root)
    }

    pub fn traverse(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(r) = root {
            let mut vec = vec![r.borrow().val];
            let mut vec_left = Self::traverse(&r.borrow().left);
            let mut vec_right = Self::traverse(&r.borrow().right);
            vec.append(&mut vec_left);
            vec.append(&mut vec_right);
            vec  
        } else {
            vec![]
        }
    }
}


impl Solution {
    
}


pub fn inorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    /* 中序遍历 首先中序遍历根结点的左子树，然后访问根结点，最后中序遍历其右子树。
    */
    let mut v = vec![];
    if root.is_none(){return v;}
    let mut stack = vec![];
    while stack.len() != 0 || !root.is_none() {
        while !root.is_none() {
            // 一直添加左子树直到为空
            let node = root.unwrap();
            root = node.borrow_mut().left.take();
            stack.push(node);
        }
        // 从栈中弹出，取节点的右子树
        root = stack.pop();
        v.push(root.as_mut().unwrap().borrow().val);
        root = root.unwrap().borrow_mut().right.take();
    }
    v
}

pub fn postorder_traversal(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    /*首先后序遍历根结点的左子树，然后后序遍历根结点的右子树，最后访问根结点。
    输入: [1,null,2,3]
       1
        \
         2
        /
       3
    输出: [3,2,1]
    */
    let mut v = vec![];
    let mut queue = VecDeque::new();
    if root.is_none(){return v;}
    let mut stack = vec![root];
    while stack.len() != 0 {
        let mut node = stack.pop().unwrap().unwrap(); // stack.pop返回option所以这里需要执行两次unwrap
    }

}

pub fn level_order_bottom(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    /* 二叉树层次遍历
    // 输入: [1,null,2,3]
    //    1
    //     \
    //      2
    //     /
    //    3
    输出: [1,3,2]
    */
    let mut v = vec![];
    if root.is_none() {
      return v;
    }
    let mut queue = VecDeque::new();
    queue.push_front(root);
    while queue.len() != 0 {
      let mut node_vec = vec![];
      let mut val_vec = vec![];
      while queue.len() != 0 {
        let node = queue.pop_front().unwrap().unwrap();
        val_vec.push(node.borrow().val); // 记录当前行的所有元素的val
        node_vec.push(node); // 将当前队列中的所有元素出队并保存，即当前行的所有元素
      }
      v.push(val_vec);
      for i in &node_vec {
        // 把当前行所有元素的下一行元素入队
        let node = i;
        if !node.borrow().left.is_none() {
          queue.push_back(node.borrow_mut().left.take());
        }
        if !node.borrow().right.is_none() {
          queue.push_back(node.borrow_mut().right.take());
        }
      }
    }
    let mut i = 0;
    let mut res = vec![];
    while i < v.len() {
      res.push(v[v.len() - i - 1].clone());
      i += 1;
    }
    res
  }
