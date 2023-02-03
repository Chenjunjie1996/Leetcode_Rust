
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