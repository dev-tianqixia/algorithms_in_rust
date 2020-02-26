// a complete binary tree represented in level order using array(vector)
// parent of node at index k: (k-1)/2
// children of node at index k: 2*k+1, 2*k+2
// leaves: [n/2..n], where n == v.len()
struct Heap<T: PartialOrd + Copy> {
    v: Vec<T>
}

impl<T: PartialOrd + Copy> Heap<T> {
    fn new() -> Heap<T> {
        Heap {
            v: Vec::new(),
        }
    }

    fn from(v: Vec<T>) -> Heap<T> {
        let mut heap = Heap {
            v,
        };
        heap.fix();
        heap
    }
}

// private methods
impl<T: PartialOrd + Copy> Heap<T> {
    // sink implements top-down heapify
    // sink is used when heap invariant is violated because a node becomes
    // smaller than one or both of its children.
    fn sink(&mut self, index: usize) {
        let mut i = index;
        // while v[i] is not a leave
        while i < self.v.len()/2 {
            let left = i*2+1; // left child
            let right = i*2+2; // right child
            let j = if right < self.v.len() && self.v[right] > self.v[left] {
                right
            } else {
                left
            };

            if self.v[i] >= self.v[j] {
                break;
            }

            self.v.swap(i, j);
            i = j;
        }
    }

    // swim implements bottom-up reheapify.
    // swim is used when heap invariant is violated because a node becomes:
    //     larger than its parent(for max heap);
    //     smaller than its parent(for min heap).
    fn swim(&mut self, index: usize) {
        let mut i = index;
        while i > 0 && self.v[i] > self.v[(i-1)/2] {
            self.v.swap(i, (i-1)/2);
            i = (i-1)/2;
        }
    }

    // fix the heap to retain the heap invariant
    fn fix(&mut self) {
        for i in (0..self.v.len()/2).rev() {
            self.sink(i);
        }
    }
}

// public methods
impl<T: PartialOrd + Copy> Heap<T> {
    // heap operations normally make a simple modification that violates the heap invariant,
    // then traveling through and modify the heap as required to retain the heap invariant.

    pub fn insert(&mut self, target: T) {
        self.v.push(target);
        self.swim(self.v.len()-1);
    }

    pub fn remove(&mut self) -> Option<T> {
        if self.v.is_empty() {
            None
        } else {
            let i = self.v.len()-1;
            self.v.swap(0, i);

            let res = Some(self.v.pop().unwrap());
            self.sink(0);

            res
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_heap_operations() {
        let mut h = Heap::from(vec![3,7,6,2,1,5,4,8]);

        for i in (1..=8).rev() {
            match h.remove() {
                Some(res) => assert!(res == i),
                None => assert!(1 == 0),
            }
        }

        match h.remove() {
            Some(_) => assert!(0 == 1),
            None => (),
        }
    }
}