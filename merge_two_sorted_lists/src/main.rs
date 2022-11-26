/*You are given the heads of two sorted linked lists list1 and list2.

Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.

Return the head of the merged linked list.
*/

//Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

use std::mem;

pub fn merge_two_lists(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = None;
    let mut p_next = &mut dummy;
    while l1.is_some() && l2.is_some() {
        let lone = &mut l1;
        let ltwo = &mut l2;
        let l = if lone.as_ref().unwrap().val < ltwo.as_ref().unwrap().val {
            lone
        } else {
            ltwo
        };
        mem::swap(p_next, l);
        mem::swap(l, &mut p_next.as_mut().unwrap().next);
        p_next = &mut p_next.as_mut().unwrap().next;
    }
    mem::swap(p_next, if l1.is_none() { &mut l2 } else { &mut l1 });
    dummy
}

fn main() {
    println!("Input: list1 = [1,2,4], list2 = [1,3,4]");
    println!("Output: [1,1,2,3,4,4]");

    let l4: Option<Box<ListNode>> = Some(Box::new(ListNode::new(4)));
    let _box_lista: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 2,
        next: l4.clone(),
    }));
    let _box_listb: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 3, next: l4 }));

    let _box_list1: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: _box_lista,
    }));

    let _box_list2: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: _box_listb,
    }));

    println!("_box_list1: {:?}", _box_list1);
    println!("_box_list2: {:?}", _box_list2);
    let result = merge_two_lists(_box_list1, _box_list2);
    println!("result = {:?}", result);
}
