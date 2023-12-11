use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

pub struct Solution;

impl Solution {
    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        match root {
            Some(node) => {
                let remaining_sum = target_sum - node.borrow().val;

                // Якщо це листок і сума на шляху дорівнює цільовій сумі, повертаємо true
                if node.borrow().left.is_none()
                    && node.borrow().right.is_none()
                    && remaining_sum == 0 {
                    return true;
                }

                // Рекурсивно перевіряємо ліве та праве піддерева
                Solution::has_path_sum(node.borrow().left.clone(), remaining_sum)
                    || Solution::has_path_sum(node.borrow().right.clone(), remaining_sum)
            }
            None => false, // Порожнє дерево, не має шляху
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ex1() {
        // let root = Some(Rc::new(RefCell::new(TreeNode {
        //     val: 5,
        //     left: Some(Rc::new(RefCell::new(TreeNode {
        //         val: 4,
        //         left: Some(Rc::new(RefCell::new(TreeNode::new(11)))),
        //         right: None,
        //     }))),
        //     right: Some(Rc::new(RefCell::new(TreeNode {
        //         val: 8,
        //         left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
        //         right: Some(Rc::new(RefCell::new(TreeNode {
        //             val: 4,
        //             left: None,
        //             right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
        //         }))),
        //     }))),
        // })));
        // assert_eq!(Solution::has_path_sum(root, 22), true)
    }
}