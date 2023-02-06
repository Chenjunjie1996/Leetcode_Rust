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
