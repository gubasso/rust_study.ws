// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}

fn vec_to_list(vec: &Vec<i32>) -> Option<Box<ListNode>> {
    if vec.is_empty() { return None; };
    let mut list = Some(Box::new(ListNode::new(vec[0])));
    let mut pointer = &mut list;

    for e in vec.iter().skip(1) {
        pointer.as_mut().unwrap().next = Some(Box::new(ListNode::new(*e)));
        pointer = &mut pointer.as_mut().unwrap().next;
    }

    list
}

fn list_to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
    if list.is_none() { return vec![]; };
    let mut vec = Vec::new();
    let mut pointer = list;

    while let Some(node) = pointer {
        vec.push(node.val);
        pointer = &pointer.as_ref().unwrap().next;
    }

    vec
}


pub fn invert_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut curr) = (None, head);

    while let Some(mut node) = curr.take() {
        curr = node.next;
        node.next = prev.take();
        prev = Some(node);
    }

    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_utils() {
        let list = vec_to_list(&vec![1,2,3,4,5]);
        let vec = list_to_vec(&list);
        // println!("{:?}", list);
        // println!("{:?}", vec);
    }

    #[test]
    fn t1() {
        let list = vec_to_list(&vec![1,2,3,4,5]);
        let result = invert_linked_list(list);
        assert_eq!(list_to_vec(&result), vec![5,4,3,2,1]);
    }

}
