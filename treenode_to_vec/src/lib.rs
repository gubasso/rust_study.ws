use std::collections::VecDeque;
use std::cell::RefCell;
use std::rc::Rc;


#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn from_tn_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut queue = Vec::new();

    if let Some(root) = root {
        queue.push(root.clone());
    }

    while !queue.is_empty() {
        if let Some(node) = Some(queue.remove(0)) {
            // let node_ref = node.borrow();
            result.push(node.borrow().val);

            if let Some(left) = node.borrow().left.clone() {
                queue.push(left);
            }

            if let Some(right) = node.borrow().right.clone() {
                queue.push(right);
            }
        }
    }

    result
}

// fn from_tn_to_vec(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
//     let mut result = Vec::new();
//     let mut queue = VecDeque::new();
//
//     if let Some(root) = root {
//         queue.push_back(root.clone());
//     }
//
//     while !queue.is_empty() {
//         if let Some(node) = queue.pop_front() {
//             // let node_ref = node.borrow();
//             result.push(node.borrow().val);
//
//             if let Some(left) = node.borrow().left.clone() {
//                 queue.push_back(left);
//             }
//
//             if let Some(right) = node.borrow().right.clone() {
//                 queue.push_back(right);
//             }
//         }
//     }
//
//     result
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_from_tn_to_vec_1() {
        let left_2 = Rc::new(RefCell::new(TreeNode::new(1)));
        let right_2 = Rc::new(RefCell::new(TreeNode::new(3)));
        let left_7 = Rc::new(RefCell::new(TreeNode::new(6)));
        let right_7 = Rc::new(RefCell::new(TreeNode::new(9)));
        let left_4 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(left_2.clone()),
            right: Some(right_2.clone()),
        }));
        let right_4 = Rc::new(RefCell::new(TreeNode {
            val: 7,
            left: Some(left_7.clone()),
            right: Some(right_7.clone()),
        }));

        let root = Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(left_4.clone()),
            right: Some(right_4.clone()),
        }));
        let goal_result = vec![4, 2, 7, 1, 3, 6, 9];
        println!("{:?}", goal_result);
        let mut target_result = Vec::new();
        target_result = from_tn_to_vec(Some(root.clone()));
        println!("{:?}", target_result);
        assert_eq!(goal_result, target_result);
    }

}
