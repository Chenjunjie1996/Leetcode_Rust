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
        while node.is_some() || stack.len()>0 {
            while let Some(n) = node {
                ans.push(n.borrow().val);
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                node = n.borrow_mut().right.take();
            }
        }
        ans
    }

    // 递归法
    pub fn preorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::traverse_preorder(&root)
    }

    pub fn traverse_preorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(r) = root {
            let mut vec = vec![r.borrow().val];
            let mut vec_left = Self::traverse_preorder(&r.borrow().left);
            let mut vec_right = Self::traverse_preorder(&r.borrow().right);
            vec.append(&mut vec_left);
            vec.append(&mut vec_right);
            vec  
        } else {
            vec![]
        }
    }
}


impl Solution {
    pub fn inorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::traverse_inorder(&root)
    }
    pub fn traverse_inorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        if let Some(r) = root {
            let mut ans_left = Self::traverse_inorder(&r.borrow_mut().left.take());
            let mut ans_right = Self::traverse_inorder(&r.borrow_mut().right.take());
            ans.append(&mut ans_left);
            ans.push(r.borrow().val);
            ans.append(&mut ans_right);
        }
        ans
    }

    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;

        while node.is_some() || stack.len()>0 {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }
            if let Some(n) = stack.pop(){
                ans.push(n.borrow().val);
                node = n.borrow_mut().right.take();
            }
        }
        ans
    }
}


impl Solution {
    pub fn postorder_traversal_recursion(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        Self::traverse_postorder(&root)
    }

    pub fn traverse_postorder(root: &Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];

        if let Some(r) = root {
            let mut ans_left = Self::traverse_postorder(&r.borrow().left);
            let mut ans_right = Self::traverse_postorder(&r.borrow().right);

            ans.append(&mut ans_left);
            ans.append(&mut ans_right);
            ans.push(r.borrow().val);
        }
        ans
    }

    pub fn postorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut ans = vec![];
        let mut stack = vec![];
        let mut node = root;
        let mut prev = None;

        while node.is_some() || stack.len() > 0 {
            while let Some(n) = node {
                node = n.borrow_mut().left.take();
                stack.push(n);
            }

            if let Some(n) = stack.pop() {
                if n.borrow().right.is_none() || n.borrow().right == prev {
                    ans.push(n.borrow().val);
                    prev = Some(n);
                    node = None;
                } else {
                    node = n.borrow_mut().right.take();
                    stack.push(n);
                }
            }
        }
        ans
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
    let mut ans = Vec::new();
    let mut stack = Vec::new();
    if root.is_none(){
        return ans;
    }
    stack.push(root.unwrap());
    while stack.is_empty()!= true{
        let num = stack.len();
        let mut level = Vec::new();
        for _i in 0..num{
            let tmp = stack.remove(0);
            level.push(tmp.borrow_mut().val);
            if tmp.borrow_mut().left.is_some(){
                stack.push(tmp.borrow_mut().left.take().unwrap());
            }
            if tmp.borrow_mut().right.is_some(){
                stack.push(tmp.borrow_mut().right.take().unwrap());
            }
        }
        ans.push(level);
    }
    ans
}
