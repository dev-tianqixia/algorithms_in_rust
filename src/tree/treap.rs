// treap aka randomized binary search tree
// treap is a data structure that stores pairs (X, Y) in a binary tree in such a way:
// it is a binary search tree by X and a binary heap by Y

// references:
// https://cp-algorithms.com/data_structures/treap.html

use rand::Rng;
use rand::distributions::{Distribution, Standard};
use std::cmp::Ordering::{Less, Greater, Equal};
use std::mem::swap;

struct Node<T> {
    key: T,
    priority: u64,
    size: usize,
    left: Vertex<T>,
    right: Vertex<T>,
}

type Vertex<T> = Option<Box<Node<T>>>;

fn size<T>(v: &Vertex<T>) -> usize {
    match v {
        Some(n) => n.size,
        None => 0,
    }
}

fn update_size<T>(v: Vertex<T>) -> Vertex<T> {
    if let Some(mut n) = v {
        n.size = size(&n.left) + 1 + size(&n.right);
        Some(n)
    } else {
        None
    }
}

fn split<T>(v: Vertex<T>, target: &T) -> (Vertex<T>, Vertex<T>) where T: Ord {
    if let Some(mut node) = v {
        match target.cmp(&node.key) {
            Less => {
                let (l, r) = split(node.left, target);
                node.left = r;
                (l, update_size(Some(node)))
            },
            Greater | Equal => {
                let (l, r) = split(node.right, target);
                node.right = l;
                (update_size(Some(node)), r)
            },
        }
    } else {
        (None, None)
    }
}

// prerequisite: all elements in v1 is less than v2
fn merge<T>(v1: Vertex<T>, v2: Vertex<T>) -> Vertex<T> {
    match (v1, v2) {
        (Some(mut n1), Some(mut n2)) => {
            match n1.priority.cmp(&n2.priority) {
                Less => {
                    n2.left = merge(Some(n1), n2.left);
                    update_size(Some(n2))
                },
                Greater => {
                    n1.right = merge(n1.right, Some(n2));
                    update_size(Some(n1))
                },
                Equal => panic!("nodes cannot have equal priority"),
            }
        },
        (v @ Some(_), None) | (None, v @ Some(_)) => v,
        (None, None) => None,
    }
}

fn insert<T>(v: Vertex<T>, mut target: Node<T>) -> Vertex<T> where T: Ord {
    if let Some(mut n) = v {
        match target.priority.cmp(&n.priority) {
            Less => {
                match target.key.cmp(&n.key) {
                    Less => {
                        n.left = insert(n.left, target);
                        update_size(Some(n))
                    },
                    Greater | Equal => {
                        n.right = insert(n.right, target);
                        update_size(Some(n))
                    },
                }
            },
            Greater => {
                let (l, r) = split(Some(n), &target.key);
                target.left = l; target.right = r;
                update_size(Some(Box::new(target)))
            },
            Equal => panic!("nodes cannot have equal priority"),
        }
    } else {
        Some(Box::new(target))
    }
}

fn delete<T>(v: Vertex<T>, target: T) -> Vertex<T> where T: Ord {
    if let Some(mut n) = v {
        match target.cmp(&n.key) {
            Less => {
                n.left = delete(n.left, target);
                update_size(Some(n))
            },
            Greater => {
                n.right = delete(n.right, target);
                update_size(Some(n))
            },
            Equal => {
                merge(n.left, n.right)
            },
        }
    } else {
        None
    }
}

fn union<T>(v1: Vertex<T>, v2: Vertex<T>) -> Vertex<T> where T: Ord {
    match (v1, v2) {
        (Some(mut n1), Some(mut n2)) => {
            match n1.priority.cmp(&n2.priority) {
                Less => {
                    let (n1_left, n1_right) = split(Some(n1), &n2.key);
                    n2.left = union(n2.left, n1_left);
                    n2.right = union(n2.right, n1_right);
                    update_size(Some(n2))
                },
                Greater => {
                    let (n2_left, n2_right) = split(Some(n2), &n1.key);
                    n1.left = union(n1.left, n2_left);
                    n1.right = union(n1.right, n2_right);
                    update_size(Some(n1))
                },
                Equal => panic!("nodes cannot have equal priority"),
            }
        },
        (v @ Some(_), None) | (None, v @ Some(_)) => v,
        (None, None) => None,
    }
}

fn get_priority<T>(v: &Vertex<T>) -> u64 {
    if let Some(ref n) = v {
        n.priority
    } else {
        0
    }
}

fn heapify<T>(v: &mut Vertex<T>) {
    if let Some(ref mut n) = v {
        let max = match get_priority(&n.left).cmp(&get_priority(&n.right)) {
            Less => &mut n.right,
            Greater | Equal => &mut n.left,
        };
        if get_priority(max) > n.priority {
            swap(&mut n.priority, &mut max.as_mut().unwrap().priority);
            heapify(max);
        }
    }
}

fn build<T>(source: &[T]) -> Vertex<T> where T: Ord + Copy {
    if !source.is_empty() {
        let mid = source.len()/2;
        let mut n = Node::new(source[mid]);
        n.left = build(&source[0..mid]);
        n.right = build(&source[mid+1..source.len()]);

        let mut v = Some(Box::new(n));
        heapify(&mut v);
        v
    } else {
        None
    }
}

fn generate_random_priority<T>() -> T where Standard: Distribution<T> {
    rand::thread_rng().gen()
}

impl<T> Node<T> where T: Ord {
    fn new(key: T) -> Self {
        Node {
            key: key,
            priority: generate_random_priority(),
            size: 1,
            left: None,
            right: None,
        }
    }
}

pub struct Treap<T> {
    root: Vertex<T>,
}

impl<T> Treap<T> where T: Ord {
    pub fn new() -> Self {
        Treap {
            root: None,
        }
    }

    pub fn size(&self) -> usize {
        size(&self.root)
    }

    pub fn insert(mut self, target: T) -> Self {
        self.root = insert(self.root, Node::new(target));
        self
    }

    pub fn delete(mut self, target: T) -> Self {
        self.root = delete(self.root, target);
        self
    }
}

impl<T> Treap<T> where T: Ord + Copy {    
    // construct a treap from sorted slice of T
    pub fn from(source: &[T]) -> Self {
        Treap {
            root: build(source),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_helper<T>(v: &Vertex<T>) -> bool where T: Ord {
        if let Some(ref n) = v {
            match (&n.left, &n.right) {
                (lv @ Some(_), rv @ Some(_)) => {
                    if n.key > lv.as_ref().unwrap().key && n.priority > lv.as_ref().unwrap().priority && 
                        n.key <= rv.as_ref().unwrap().key && n.priority > rv.as_ref().unwrap().priority {
                        assert_helper(lv) && assert_helper(rv)
                    } else {
                        false
                    }
                },
                (lv @ Some(_), None) => {
                    if n.key > lv.as_ref().unwrap().key && n.priority > lv.as_ref().unwrap().priority {
                        assert_helper(lv)
                    } else {
                        false
                    }
                },
                (None, rv @ Some(_)) => {
                    if n.key <= rv.as_ref().unwrap().key && n.priority > rv.as_ref().unwrap().priority {
                        assert_helper(rv)
                    } else {
                        false
                    }
                },
                (None, None) => true,
            }
        } else {
            true
        }
    }

    fn treap_assert<T>(t: &Treap<T>) -> bool where T: Ord{
        if t.root.is_some() {
            assert_helper(&t.root)
        } else {
            true
        }
    }

    #[test]
    fn test_from() {
        let t = Treap::from(&vec![1,2,3,4,5,6,7,8,9]);
        assert!(treap_assert(&t));
    }

    #[test]
    fn test_treap_insertion() {
        let mut t = Treap::new();
        for i in 0..1000 {
            t = t.insert(i);
        }

        assert!(treap_assert(&t));
        assert_eq!(t.size(), 1000);
    }

    #[test]
    fn test_treap_insert_and_delete() {
        let mut t = Treap::new();
        t = t.insert(10).insert(5).insert(1);

        assert!(treap_assert(&t));
        assert_eq!(t.size(), 3);

        t = t.delete(5);
        assert!(treap_assert(&t));
        assert_eq!(t.size(), 2);

        t = t.delete(3);
        assert!(treap_assert(&t));
        assert_eq!(t.size(), 2);

        t = t.delete(10).delete(1);
        assert!(treap_assert(&t));
        assert_eq!(t.size(), 0);
    }
}