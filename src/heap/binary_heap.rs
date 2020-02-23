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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        Heap::new(vec![0]);
    }
}