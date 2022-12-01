/*Given the head of a singly linked list, reverse the list, and return the reversed list.*/
// Definition for singly-linked list.
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
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);
    while let Some(mut node) = curr {
        curr = node.next;
        node.next = prev;
        prev = Some(node);
    }
    prev
}
fn main() {
    println!("Input: list1 = [1,2,3,4,5]");
    println!("Output: [5,4,3,2,1]");

    let l5: Option<Box<ListNode>> = Some(Box::new(ListNode::new(5)));
    let _box_lista: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 4,
        next: l5.clone(),
    }));

    let _box_lista_2: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 3,
        next: _box_lista,
    }));

    let _box_lista_3: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 2,
        next: _box_lista_2,
    }));

    let _box_lista_4: Option<Box<ListNode>> = Some(Box::new(ListNode {
        val: 1,
        next: _box_lista_3,
    }));

    println!("_box_lista_4: {:?}", _box_lista_4);

    let result = reverse_list(_box_lista_4);
    println!("result = {:?}", result);
}
