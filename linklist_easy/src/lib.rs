
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

impl Solution {
    /* 删除节点
    输入: head = [4,5,1,9], val = 5
    输出: [4,1,9]
    */
    fn delete_node(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode { val: 5, next: head }));
        let mut root = &mut head;
        while let Some(node) = root {
            let next_node = &mut node.next;
            match next_node {
            None => break,
            Some(next_node) => {
                if next_node.val == val {
                    // 当前节点的下一个节点等于目标节点
                    node.next = next_node.next.take();
                    break;
                    }
                }
            }
            root = &mut node.next;
        }
        head.unwrap().next
    }
}

impl Solution {
    /* 删除排序链表中重复的元素
    输入：head = [1,1,2]
    输出：[1,2]
    */
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref()?.next.is_none() {
            return head;
        }
        let mut root = &mut head;

        while root.is_some() && root.as_mut()?.next.is_some() {
            let mut node = root.as_mut().unwrap();
            let next_node = &mut node.next;
            if next_node.as_ref()?.val == node.val {
                node.next = next_node.as_mut()?.next.take();
            } else {
                root = &mut root.as_mut()?.next;
            }
        }
        head
    }
}

