// a complete binary tree represented in level order using array(vector)
// parent of node k: (k-1)/2
// children of node k: 2*k+1, 2*k+2
struct Heap<T: PartialOrd> {
    v: Vec<T>
}

impl<T: PartialOrd> Heap<T> {
    fn new(v: Vec<T>) -> Heap<T> {
        Heap {
            v: v
        }
    }
}

// heap operations normally make a simple modification that violates the heap invariant,
// then traveling through and modify the heap as required to regain the heap invariant.
impl<T: PartialOrd> Heap<T> {
    // swim implements bottom-up reheapify
    fn swim(&mut self, index: usize) {
        let mut i = index;
        while i > 0 && self.v[i] > self.v[(i-1)/2] {
            self.v.swap(i, (i-1)/2);
            i = (i-1)/2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        Heap::new(vec![0]);
    }
}