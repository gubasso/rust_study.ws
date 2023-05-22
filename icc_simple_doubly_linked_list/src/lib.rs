use std::mem;

#[derive(Debug)]
struct ListNode {
    val: i32,
    next: Link,
    prev: Link,
}

type Link = Option<Box<ListNode>>;

pub struct List {
    head: Link,
    tail: Link,
}

impl List {

    pub fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    fn push(&mut self, elem: i32) {
        let new_node = ListNode {
            val: elem,
            next: self.head.take(),
            prev: None,
        };
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_pop() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        list.push(4);
        list.push(5);
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert_eq!(list.pop(), None);
    }

}
