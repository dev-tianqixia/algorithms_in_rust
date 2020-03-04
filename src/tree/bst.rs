use std::cmp;
use std::cmp::Ordering::{Less, Greater, Equal};

// a binary search tree is a binary tree that satifies:
// any node is larger than all nodes in its left subtree and
// smaller than all nodes in its right subtree

// inspired by the following references:
// https://algs4.cs.princeton.edu/32bst/
// https://codereview.stackexchange.com/questions/133209/binary-tree-implementation-in-rust
// https://github.com/TheAlgorithms/Rust/blob/master/src/data_structures/binary_search_tree.rs 

////////////////////////////////////////////////////////////////////////////////
// internal tree node
////////////////////////////////////////////////////////////////////////////////
struct Node<T: Ord + Copy> {
    data: T,
    size: usize,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

fn size<T: Ord + Copy>(node: &Option<Box<Node<T>>>) -> usize {
    match node {
        Some(node_box) => node_box.size,
        None => 0,
    }
}

fn insert<T: Ord + Copy>(node: Option<Box<Node<T>>>, target: T) -> Option<Box<Node<T>>> {
    if let Some(mut node_box) = node {
        match node_box.data.cmp(&target) {
            Less => node_box.right = insert(node_box.right, target),
            Greater | Equal => node_box.left = insert(node_box.left, target),
        };

        node_box.size = size(&node_box.left) + 1 + size(&node_box.right);
        Some(node_box)
    } else {
        Some(Box::new(Node::new(target)))
    }
}

// recursive search as a stand-alone function
fn search<T: Ord + Copy>(node: &Option<Box<Node<T>>>, target: T) -> &Option<Box<Node<T>>> {
    match node {
        Some(ref node_box) => {
            match node_box.data.cmp(&target) {
                Less => search(&node_box.right, target),
                Greater => search(&node_box.left, target),
                Equal => node,
            }
        },
        None => node
    }
}

fn successor<T: Ord + Copy>(mut node: &Option<Box<Node<T>>>) -> &Option<Box<Node<T>>> {
    while let Some(ref node_box) = node {
        match node_box.left {
            ref next @ Some(_) => node = next,
            None => return node,
        }
    }
    node
}

fn successor_mut<T: Ord + Copy>(mut node: &mut Option<Box<Node<T>>>) -> &mut Option<Box<Node<T>>> {
    while let Some(ref mut node_box) = node {
        match node_box.left {
            ref mut next @ Some(_) => node = next,
            None => return &mut node_box.right,
        }
    };
    node
}

fn delete_successor<T: Ord + Copy>(node: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    if let Some(mut node_box) = node {
        match node_box.left {
            next @ Some(_) => {
                node_box.left = delete_successor(next);
                node_box.size -= 1;
                Some(node_box)
            },
            None => node_box.right,
        }
    } else {
        node
    }
}

fn delete<T: Ord+Copy>(node: Option<Box<Node<T>>>, target: T) -> Option<Box<Node<T>>> {
    if let Some(mut node_box) = node {
        match node_box.data.cmp(&target) {
            Less => {
                // try to delete target in right subtree
                node_box.right = delete(node_box.right, target);
            },
            Greater => {
                // try to delete target in left subtree
                node_box.left = delete(node_box.left, target);
            },
            Equal => {
                match (node_box.left.take(), node_box.right.take()) {
                    (None, None) => return None,
                    (sub_tree @ Some(_), None) | (None, sub_tree @ Some(_)) => return sub_tree,
                    (left, right) => {
                        // attach left sub tree back, as we took it
                        node_box.left = left;
                        // successor can be safely unwrapped
                        node_box.data = successor(&right).as_ref().unwrap().data;
                        // delete min from right sub tree
                        node_box.right = delete_successor(right);
                    },
                };
            },
        };

        node_box.size = size(&node_box.left) + 1 + size(&node_box.right);
        Some(node_box)
    } else {
        node
    }
}

impl<T: Ord + Copy> Node<T> {
    fn new(data: T) -> Self {
        Self {
            data: data,
            size: 1,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, data: T) {
        if data <= self.data {
            // insert to left subtree
            match self.left {
                Some(ref mut node_box) => node_box.insert(data),
                None => self.left = Some(Box::new(Node::new(data))),
            };
        } else {
            // insert to right sub tree
            match self.right {
                Some(ref mut node_box) => node_box.insert(data),
                None => self.right = Some(Box::new(Node::new(data))),
            };
        }
    }

    fn search(&self, target: T) -> bool {
        if target == self.data {
            true
        } else if target <= self.data {
            match self.left {
                Some(ref node_box) => node_box.search(target),
                None => false,
            }
        } else /*target > self.data*/ {
            match self.right {
                Some(ref node_box) => node_box.search(target),
                None => false,
            }
        }
    }

    fn depth(&self) -> usize {
        let left_depth = match self.left {
            Some(ref node_box) => node_box.depth(),
            None => 0,
        };

        let right_depth = match self.right {
            Some(ref node_box) => node_box.depth(),
            None => 0,
        };

        cmp::max(left_depth, right_depth) + 1
    }

    fn min(&self) -> &T {
        match self.left {
            Some(ref node_box) => node_box.min(),
            None => &self.data,
        }
    }

    fn max(&self) -> &T {
        match self.right {
            Some(ref node_box) => node_box.max(),
            None => &self.data,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// exposed BST type
////////////////////////////////////////////////////////////////////////////////

pub struct BST<T: Ord + Copy> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord + Copy> BST<T> {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn insert(mut self, data: T) -> Self {
        self.root = insert(self.root, data);
        self
    }

    pub fn search(&self, target: T) -> bool {
        match search(&self.root, target) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn size(&self) -> usize {
        size(&self.root)
    }

    pub fn depth(&self) -> usize {
        match &self.root {
            Some(node) => node.depth(),
            None => 0,
        }
    }

    pub fn min(&self) -> Option<&T> {
        match &self.root {
            Some(node) => Some(node.min()),
            None => None,
        }
    }

    pub fn max(&self) -> Option<&T> {
        match &self.root {
            Some(node) => Some(node.max()),
            None => None,
        }
    }

    pub fn delete(mut self, target: T) -> Self {
        self.root = delete(self.root, target);
        self
    }

    pub fn delete_min(mut self) -> Self {
        self.root = delete_successor(self.root);
        self
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }
}

////////////////////////////////////////////////////////////////////////////////
// iterators
////////////////////////////////////////////////////////////////////////////////

pub struct Iter<'a, T: Ord + Copy> {
    stack: Vec<&'a Node<T>>,
}

impl<'a, T: Ord + Copy> Iter<'a, T> {
    fn new(bst: &'a BST<T>) -> Iter<'a, T> {
        match &bst.root {
            Some(node) => {
                let mut iter = Iter{stack: Vec::new()};
                iter.push_left(node);
                iter
            },
            None => Iter{stack: Vec::new()},
        }
    }

    // push all the nodes whose data is less than or equal to the current node's data onto the stack
    fn push_left(&mut self, node: &'a Node<T>) {
        self.stack.push(node);
        match node.left {
            Some(ref node_box) => {
                self.push_left(node_box);
            },
            None => (),
        }
    }
}

impl<'a, T: Ord + Copy> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(node) => {
                match node.right {
                    Some(ref node_box) => self.push_left(node_box),
                    None => (),
                };
                Some(&node.data)
            },
            None => None,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////
// tests
////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_operation() {
        let t = BST::new().insert(1);
        assert!(t.search(1));
    }

    #[test]
    fn test_depth() {
        let mut t = BST::new().insert(3).insert(2).insert(1);
        assert_eq!(t.depth(), 3);

        t = t.insert(4).insert(5);
        assert_eq!(t.depth(), 3);

        t = t.insert(6);
        assert_eq!(t.depth(), 4);
    }

    #[test]
    fn test_iteration() {
        let mut t = BST::new();
        t = t.insert(3).insert(2).insert(1);

        for (i, &num) in t.iter().enumerate() {
            assert_eq!(i+1, num);
        }
        for (i, &num) in t.iter().enumerate() {
            assert_eq!(i+1, num);
        }
    }

    #[test]
    fn test_min_max() {
        let mut t = BST::new();
        assert_eq!(t.min(), None);
        assert_eq!(t.max(), None);

        t = t.insert(3).insert(2).insert(1);

        assert_eq!(*t.min().unwrap(), 1);
        assert_eq!(*t.max().unwrap(), 3);

        t = t.insert(3);
        assert_eq!(*t.max().unwrap(), 3);

        t = t.insert(4);
        assert_eq!(*t.max().unwrap(), 4);
    }

    #[test]
    fn test_delete_min() {
        let mut t = BST::new().insert(4).insert(3).insert(2).insert(1);

        t = t.delete_min();
        assert_eq!(*t.min().unwrap(), 2);
        assert_eq!(t.size(), 3);
        t = t.delete_min().delete_min();
        assert_eq!(*t.min().unwrap(), 4);
        assert_eq!(t.size(), 1);
    }

    #[test]
    fn test_delete() {
        let mut t = BST::new();
        t = t.insert(4).insert(3).insert(2).insert(1);

        t = t.delete(2);
        assert!(!t.search(2));
        t = t.delete(3);
        assert!(!t.search(3));
        t = t.delete(4);
        assert!(!t.search(4));
        assert!(t.search(1));

        for (i, &num) in t.iter().enumerate() {
            assert_eq!(i+1, num);
            if i > 1 {
                panic!("should only have 2 elements left");
            }
        }

        t = t.insert(4);
        assert!(t.search(4));
    }

    #[test]
    fn test_size() {
        let mut t = BST::new().insert(1).insert(1).insert(1);
        assert_eq!(t.size(), 3);

        t = t.delete(1);
        assert_eq!(t.size(), 2);

        t = t.delete(0);
        assert_eq!(t.size(), 2);

        t = t.delete(1);
        assert_eq!(t.size(), 1);

        t = t.insert(1).insert(1).insert(1).insert(1);
        assert_eq!(t.size(), 5);
    }
}
