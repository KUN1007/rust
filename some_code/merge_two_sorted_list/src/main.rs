// 定义链表节点结构体
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// 合并两个有序链表的函数

pub fn merge_two_lists(
    mut list1: Option<Box<ListNode>>,
    mut list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut head = ListNode::new(0);
    let mut tail = &mut head;

    while let (Some(node1), Some(node2)) = (list1.as_mut(), list2.as_mut()) {
        if node1.val <= node2.val {
            let next_node = node1.next.take();
            tail.next = list1.take();
            list1 = next_node;
        } else {
            let next_node = node2.next.take();
            tail.next = list2.take();
            list2 = next_node;
        }
        tail = tail.next.as_mut().unwrap();
    }

    tail.next = list1.or(list2);

    head.next
}

fn main() {}
