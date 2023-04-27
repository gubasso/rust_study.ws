// Definition for singly-linked list.

use std::mem;

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

    fn push(&mut self, val: i32) {
        let new_node = ListNode {
            val,
            next: Some(Box::new(mem::replace(self, ListNode::new(0)))),
        };
        *self = new_node;
    }

    fn pop(&mut self) -> Option<Box<ListNode>> {
        // match mem::replace(&mut self.next, None) {
        match self.next.take() {
            None => {
                Some(Box::new(ListNode::new(self.val)))
            },
            Some(next_node) => {
                let val = self.val;
                *self = *next_node;
                Some(Box::new(ListNode::new(val)))
            }
        }
    }

    fn peek(&self) -> Option<&ListNode> {
        Some(&self)
    }

    fn peek_mut(&mut self) -> Option<&mut ListNode> {
        Some(self)
    }

    fn into_iter(self) -> IntoIter {
        IntoIter(self)
    }

    fn iter<'a>(&'a self) -> Iter<'a> {
        Iter { next: Some(self) }
    }

}

struct IntoIter(ListNode);

impl Iterator for IntoIter {
    type Item = Box<ListNode>;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

struct Iter<'a> {
    next: Option<&'a ListNode>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a ListNode;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            node
        })
    }

}


// struct Solution;
//
// impl Solution {
//     pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    fn node_1() -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: 1,
            next: None,
        }))
    }

    fn node_2() -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: 2,
            next: node_1(),
        }))
    }

    fn node_3() -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: 3,
            next: node_2(),
        }))
    }

    fn node_4() -> Option<Box<ListNode>> {
        Some(Box::new(ListNode {
            val: 4,
            next: node_3(),
        }))
    }

    #[test]
    fn push_pop() {
        let mut list = ListNode::new(1);
        assert_eq!(list.pop(), node_1());

        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(Box::new(ListNode::new(3))));
        assert_eq!(list.pop(), Some(Box::new(ListNode::new(2))));

        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(Box::new(ListNode::new(5))));
        assert_eq!(list.pop(), Some(Box::new(ListNode::new(4))));

        assert_eq!(list.pop(), Some(Box::new(ListNode::new(1))));
        assert_eq!(list.pop(), Some(Box::new(ListNode::new(1))));
    }

    #[test]
    fn peek() {
        let mut list = ListNode::new(1);
        assert_eq!(list.peek(), Some(&ListNode::new(1)));
        assert_eq!(list.peek_mut(), Some(&mut ListNode::new(1)));

        list.push(2); list.push(3); list.push(4);

        assert_eq!(list.peek(), Some(&*node_4().unwrap()));
        assert_eq!(list.peek_mut(), Some(&mut *node_4().unwrap()));

        list.peek_mut().map(|node| {
            *node = ListNode {
                val: 72,
                next: node.next.clone(),
            };
        });

    }

    #[test]
    fn into_iter() {
        let mut list = ListNode::new(1);
        list.push(2); list.push(3); list.push(4);
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(Box::new(ListNode::new(4))));
        assert_eq!(iter.next(), Some(Box::new(ListNode::new(3))));
        assert_eq!(iter.next(), Some(Box::new(ListNode::new(2))));
        assert_eq!(iter.next(), Some(Box::new(ListNode::new(1))));
        assert_eq!(iter.next(), Some(Box::new(ListNode::new(1))));
    }

    #[test]
    fn iter() {
        let mut list = ListNode::new(1);
        list.push(2); list.push(3); list.push(4);
        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&*node_4().unwrap()));
        assert_eq!(iter.next(), Some(&*node_3().unwrap()));
        assert_eq!(iter.next(), Some(&*node_2().unwrap()));
        assert_eq!(iter.next(), Some(&*node_1().unwrap()));
    }

    // #[test]
    // fn t1() {
    //     let input = vec![1,2,3,4,5];
    //     let output = vec![3,4,5];
    //     let list_node = get_list_from_vec(input);
    //     // println!("list_node: {:?}", list_node);
    //     let result_list = Solution::middle_node(list_node);
    //     // println!("result_list: {:?}", result_list);
    //     let result_vec = get_vec_from_list(result_list);
    //     // println!("result_vec: {:?}", result_vec);
    //     assert_eq!(result_vec, output);
    // }

    // #[test]
    // fn t2() {
    //    let input = vec![1,2,3,4,5,6];
    //     let output = vec![4,5,6];
    //     let list_node = get_list_from_vec(input);
    //     let result_list = Solution::middle_node(list_node);
    //     let result_vec = get_vec_from_list(result_list);
    //     assert_eq!(result_vec, output);
    // }

}
