
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct ListNode{
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self{
        ListNode { val, next: None }
    }
}

struct Solution{}
/* 回文链表
输入：head = [1,2,2,1]
输出：true
*/
// Definition for singly-linked list.
impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none(){
            return true;
        }
        let mut vec = Vec::new();
        let mut root = &head;
        while let Some(node) = root {
            vec.push(node);
            root = &node.next;
        }

        let mut root = &head;
        while let Some(node) = root {
            let mut top = vec.pop().unwrap();
            if top.val != node.val {
                return false;
            }
            root = &node.next;
        }

        true
    }

}

/* 反转链表
输入：head = [1,2,3,4,5]
输出：[5,4,3,2,1]
*/
impl Solution {
    fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
        if head.is_none(){
            return None;
        }
        let mut prev: Option<Box<ListNode>> = None;
        let mut curr: Option<Box<ListNode>> = head;
        while curr.is_some() {
            let mut node = curr.take().unwrap();
            curr = node.next;
            node.next = prev;
            prev = Some(node);
        }
        prev
    }
}

impl Solution {
    // 中间节点
    fn middle_node(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut len = 0.0;
        let mut root = &mut head;
        while root.is_some() {
            len += 1.0;
            root = &mut root.as_mut().unwrap().next;
        }
        let middle_index = (len / 2_f64).floor();
        let mut i = 0.0;
        root = &mut head;
        while i < middle_index {
            i += 1.0;
            root = &mut root.as_mut().unwrap().next;
        }
        root.take()
    }
}

