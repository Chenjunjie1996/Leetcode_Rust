/*
as_ref / as_mut：带有 Option<T> 时可以使用，可直接获得其中的值的引用
as_ref 取Option中值的不可变引用，Option<T>变为Option<&T>
as_mut 取Option中值的可变引用，Option<T>变为Option<&mut T>
xxx.as_mut().unwrap()：用得多的情况，先变为引用，再取出引用的值
take：先取出其中的值，再留下一个 None 值

可以修改a的值(因为a是可变的)，而不能对a引用的值进行修改
let mut a = &Foo(2);
a = &Foo(5);  // ok
a.0 = 7;      // [E0594]: cannot assign to `a.0` which is behind a `&` reference

不可以修改b的值，但可以对b引用的值进行修改
let b = &mut Foo(42);
b = &Foo(36); // [E0384]: cannot assign twice to immutable variable `b`
b.0 = 100;    // ok
*/
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

impl Solution {
    /*链表遍历
    */
    pub fn walk_through(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut root = &mut head;
        while let Some(node) = root {
            let next_node = &mut node.next;
            // 使用as_mut获取next_node的引用，使用&mut获取.next的引用。以此来获取root下一个节点的下一个节点的引用。直接使用unwrap会导致所有权的move
            let next_node_next = &mut next_node.as_mut()?.next;
            // 这里面不能再直接使用head，因为head的所有权已经借给了root，在循环体中未归还
            // other code...
            root = &mut node.next;
        }
        head
    }
}
