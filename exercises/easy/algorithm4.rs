/*
	binary_search tree
	This problem requires you to implement a basic interface for a binary tree
*/


use std::cmp::Ordering;
use std::fmt::Debug;


#[derive(Debug)]
struct TreeNode<T>
where
    T: Ord+Debug,
{
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

#[derive(Debug)]
struct BinarySearchTree<T>
where
    T: Ord+ Debug,
{
    root: Option<Box<TreeNode<T>>>,
}

impl<T: Debug> TreeNode<T>
where
    T: Ord,
{
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

impl<T> BinarySearchTree<T>
where
    T: Ord+Copy+Debug,
{

    fn new() -> Self {
        BinarySearchTree { root: None }
    }

    // Insert a value into the BST
    fn insert(&mut self, value: T) where T: Debug {
        //TODO
        if self.root.is_some() {
            if self.root.as_ref().unwrap().value < value {
                if self.root.as_ref().unwrap().right.is_none() {
                    self.root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.root.as_mut().unwrap().right.as_mut().unwrap().insert(value);
                }
            }
            if self.root.as_ref().unwrap().value > value {
                if self.root.as_ref().unwrap().left.is_none() {
                    self.root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(value)));
                } else {
                    self.root.as_mut().unwrap().left.as_mut().unwrap().insert(value);
                }
            }
        } else {
            self.root = Some(Box::new(TreeNode::new(value)));
        }
    }

    // Search for a value in the BST
    fn search(&self, value: T) -> bool where T: Debug {
        //TODO
        let mut node = self.root.as_ref();
        while node.is_some() {
            println!("node value is {:?}", node.unwrap().value);
            match node.unwrap().value.cmp(&value) {
                Ordering::Equal => return true,
                Ordering::Less => node = node.unwrap().right.as_ref(),
                Ordering::Greater => node = node.unwrap().left.as_ref(),
            }
        }
        false
    }
}

impl<T: Debug> TreeNode<T>
where
    T: Ord+Copy,
{
    // Insert a node into the tree
    fn insert(&mut self, value: T) {
        //TODO
        if self.value < value {
            if self.right.is_none() {
                self.right = Some(Box::new(TreeNode::new(value)));
            } else {
                self.right.as_mut().unwrap().insert(value);
            }
        }
        if self.value > value {
            if self.left.is_none() {
                self.left = Some(Box::new(TreeNode::new(value)));
            } else {
                self.left.as_mut().unwrap().insert(value);
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insert_and_search() {
        let mut bst = BinarySearchTree::new();

        
        assert_eq!(bst.search(1), false);

        
        bst.insert(5);
        bst.insert(3);
        bst.insert(7);
        bst.insert(2);
        bst.insert(4);
        
        assert_eq!(bst.search(5), true);
        assert_eq!(bst.search(3), true);
        assert_eq!(bst.search(7), true);
        assert_eq!(bst.search(2), true);
        assert_eq!(bst.search(4), true);

        
        assert_eq!(bst.search(1), false);
        assert_eq!(bst.search(6), false);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut bst = BinarySearchTree::new();

        
        bst.insert(1);
        bst.insert(1);

        
        assert_eq!(bst.search(1), true);

        
        match bst.root {
            Some(ref node) => {
                assert!(node.left.is_none());
                assert!(node.right.is_none());
            },
            None => panic!("Root should not be None after insertion"),
        }
    }
}    


