use std::collections::HashMap;


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

impl Solution {
    /* 链表相加
    输入：l1 = [7,2,4,3], l2 = [5,6,4]
    输出：[7,8,0,7]
    */
    pub fn add_two_num(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>
    ) -> Option<Box<ListNode>> {
        let l1_sum = get_sum(&mut l1);
        let l2_sum = get_sum(&mut l2);
        let mut sum = l1_sum + l2_sum;
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut root = &mut head;
        if sum == 0{
            return head;
        }
        while sum != 0 {
            let node = Some(Box::new(ListNode::new(sum.wrapping_rem(10) as i32)));
            root.as_mut().unwrap().next = node;
            root = &mut root.as_mut().unwrap().next;
            sum = sum.wrapping_div_euclid(10);
        }
        head.unwrap().next
    }
}

fn get_sum(head: &mut Option<Box<ListNode>>) -> i128 {
    let mut root = head;
    let mut sum: i128 = 0;
    let mut bit: i128 = 1;
    while let Some(node) = root {
      sum += node.val as i128 * bit;
      bit *= 10;
      root = &mut node.next;
    }
    sum
}

impl Solution {
    /* 两两交换
    给定 1->2->3->4, 你应该返回 2->1->4->3.
    */
    pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
          return head;
        }
        let mut l1 = head;
        let mut result = ListNode::new(0);
        let mut res = &mut result;
        while let Some(mut l1_node) = l1 {
          l1 = l1_node.next.take(); // 使用take获取所有权,l1 = 2->3->4, l1_node = 1 此时l1_node.next=None
          if let Some(mut l2_node) = l1 {
            l1 = l2_node.next.take(); // l1 = 3->4, l2 = 2, 此时l2_node.next=None
            l2_node.next = Some(l1_node); // l2 = 2->1
            res.next = Some(l2_node); // res = 0->2->1,
            res = res.next.as_mut().unwrap().next.as_mut().unwrap(); // res = 1, res指向末尾
          } else {
            // 走到这里说明总的节点个数为奇数个, l1为最后一个节点，没有下一个节点。直接将l1添加到末尾
            res.next = Some(l1_node);
            res = res.next.as_mut().unwrap();
          }
        }
        result.next
      }
    }


impl Solution {
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        /*删除链表的倒数第N个节点
        输入：head = [1,2,3,4,5], n = 2
        输出：[1,2,3,5]
        */
        if head.is_none(){
            return head;
        }
        let mut slow = &head;
        let mut fast = &head;
        let mut i = 1;
        while i < n {
            // 快指针先走n步
            fast = &fast.as_ref().unwrap().next;
            i += 1;
          }
        while fast.as_ref().unwrap().next.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }
          // 这里必须clone一下因为head已经borrow了，没办法再move
        head = delete_node(head.clone(), slow);
        head
    }
}

fn delete_node(head: Option<Box<ListNode>>, target: &Option<Box<ListNode>>) -> Option<Box<ListNode>>{
    let mut phead = Some(Box::new(ListNode { val: 1, next: head }));
    let mut root = &mut phead;
    while root.as_mut().unwrap().next.is_some() {
        if &root.as_mut().unwrap().next == target {
            let target_next = &target.as_ref().unwrap().next;
            root.as_mut().unwrap().next = target_next.clone();
            break;
        }
        root = &mut root.as_mut().unwrap().next;
    }
    phead
}

impl Solution {
    /* 旋转链表
    一个链表的头节点 head ，旋转链表，将链表每个节点向右移动 k 个位置。
    输入：head = [1,2,3,4,5], k = 2
    输出：[4,5,1,2,3]
    */
    pub fn rotate(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }
        // rust 链表无法成环，拆成两个链表再连接
        let mut len = 0;
        let mut head_refer  = &head;
        // 记录长度
        while head_refer.as_ref().is_some() {
            len += 1;
            head_refer = &head_refer.as_ref().unwrap().next;
        }
        let mut k = (len - k) % len;
        if k==0 {
            return head;
        }
        let mut head_mut_refer = &mut head;
        while k>0 {
            k -= 1;
            head_mut_refer = &mut head_mut_refer.as_mut().unwrap().next;
        }
        // right = 4->5  head = 1->2->3
        let mut right = head_mut_refer.as_mut().unwrap().next.take();
        let mut right_mut_ref = &mut right;
        while right_mut_ref.as_mut().unwrap().next.is_some(){
            right_mut_ref = &mut right_mut_ref.as_mut().unwrap().next;
        }
        right_mut_ref.as_mut().unwrap().next = head;
        right 
    }
}

impl Solution {
    /*从链表中删去总和值为零的连续节点
    输入：head = [1,2,-3,3,1]
    输出：[3,1]
    答案 [1,2,1] 也是正确的。
    */
    pub fn remove_zero_sum_sublists(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none(){ return head; }
        let phead = Some(Box::new(ListNode { val: 0, next: head }));
        let mut hash_map = HashMap::new();
        let mut head_clone = phead.clone();
        let mut head_refer = &phead;
        let mut sum = 0;

        while head_refer.is_some() {
            sum = sum + &head_refer.as_ref().unwrap().val;
            hash_map.insert(sum, head_refer.clone());
            head_refer = &head_refer.as_ref().unwrap().next;
        }
        let mut head_refer = &mut head_clone;
        sum = 0;
        while head_refer.is_some() {
          sum += &head_refer.as_mut()?.val;
          if sum == 0{
            let node = hash_map.get_mut(&sum).unwrap();
            head_refer.as_mut().unwrap().next = node.as_mut().unwrap().next.take();
            head_refer = &mut head_refer.as_mut().unwrap().next;
          }
        }
        head_clone?.next
    }
}

impl Solution {
    /* 链表中的下一个更大节点
    输入：head = [2,1,5]
    输出：[5,5,0]
    输入：head = [2,7,4,3,5]
    输出：[7,0,5,5,0]
    */
    // 单调栈
    pub fn next_larger_nodes(head: Option<Box<ListNode>>) -> Vec<i32> {
        if head.is_none() {
          return vec![];
        }
        let mut arr = vec![]; // 存储链表中的元素
        let mut stack = vec![]; // 存储 arr 的元素下标
        let mut head_refer = &head;
        while head_refer.is_some() {
          arr.push(head_refer.as_ref().unwrap().val);
          head_refer = &head_refer.as_ref().unwrap().next;
        }
        let mut v = vec![0; arr.len()];
        stack.push(0);
        for i in 0..arr.len() {
          while arr[i] > arr[*stack.last().unwrap() as usize] {
            v[*stack.last().unwrap() as usize] = arr[i];
            stack.pop();
            if stack.len() == 0 {
              break;
            }
          }
          stack.push(i as i32);
        }
        v
      }
}