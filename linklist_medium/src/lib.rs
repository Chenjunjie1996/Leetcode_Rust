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
