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
    for i in 1..vec.len() {
        pointer.as_mut().unwrap().next = Some(Box::new(ListNode::new(vec[i])));
        pointer = &mut pointer.as_mut().unwrap().next;
    }
    list
}

fn list_to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {
    if list.is_none() { return vec![]; }
    let mut vec = Vec::new();
    let mut pointer = list;
    while let Some(node) = pointer {
        vec.push(node.val);
        pointer = &node.next;
    }
    vec
}

struct Solution;
impl Solution {
    pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode::new(0)));
        dummy.as_mut().unwrap().next = head;
        let mut prev = &mut dummy;

        while let Some(mut n1) = prev.as_mut().unwrap().next.take() {
            if let Some(mut n2) = n1.next.take() {
                n1.next = n2.next.take();
                n2.next = Some(n1);
                prev.as_mut().unwrap().next = Some(n2);
                prev = &mut prev
                    .as_mut().unwrap().next
                    .as_mut().unwrap().next;
            } else {
                prev.as_mut().unwrap().next = Some(n1);
                prev = &mut prev.as_mut().unwrap().next;
            }
        }

        dummy.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t1() {
        let list = vec_to_list(&vec![1,2,3,4]);
        let result = Solution::swap_pairs(list);
        assert_eq!(list_to_vec(&result), vec![2,1,4,3]);
    }

    #[test]
    fn t2() {
        let list = vec_to_list(&vec![1,2,3,4,5]);
        let result = Solution::swap_pairs(list);
        assert_eq!(list_to_vec(&result), vec![2,1,4,3,5]);
    }

    #[test]
    fn t3() {
        let list = vec_to_list(&vec![]);
        let result = Solution::swap_pairs(list);
        assert_eq!(list_to_vec(&result), vec![]);
    }

    #[test]
    fn t4() {
        let list = vec_to_list(&vec![1]);
        let result = Solution::swap_pairs(list);
        assert_eq!(list_to_vec(&result), vec![1]);
    }

}
