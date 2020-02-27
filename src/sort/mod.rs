pub mod heap_sort;
pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::heap_sort::heap_sort;
    use super::insertion_sort::insertion_sort;
    use super::insertion_sort::shell_sort;
    use super::merge_sort::merge_sort_top_down;
    use super::merge_sort::merge_sort_bottom_up;
    use super::quick_sort::quick_sort_basic;

    fn generate_random_large(size: usize) -> Vec<usize> {
        let mut gen = rand::thread_rng();
        let mut v = Vec::with_capacity(size);

        for i in 0..size {
            v.push(gen.gen_range(0, i+1));
        }
        assert!(v.len() == size);

        v
    }

    fn assert_sorted<T: PartialOrd>(v: &Vec<T>) {
        for i in 0..v.len()-1 {
            assert!(v[i] <= v[i+1]);
        }
    }

    #[test]
    fn test_heap_sort() {
        assert_sorted(heap_sort(&mut vec![1]));
        assert_sorted(heap_sort(&mut vec![1,1,1,1,1]));
        assert_sorted(heap_sort(&mut vec![4,2,6,7,1,10]));
        assert_sorted(heap_sort(&mut vec!["w", "h", "p", "a", "x"]));
        assert_sorted(heap_sort(&mut generate_random_large(10000)));
    }

    #[test]
    fn test_insertion_sort() {
        assert_sorted(insertion_sort(&mut vec![1]));
        assert_sorted(insertion_sort(&mut vec![1,1,1,1,1]));
        assert_sorted(insertion_sort(&mut vec![4,2,6,7,1,10]));
        assert_sorted(insertion_sort(&mut vec!["w", "h", "p", "a", "x"]));
        assert_sorted(insertion_sort(&mut generate_random_large(10000)));
    }

    #[test]
    fn test_shell_sort() {
        assert_sorted(shell_sort(&mut vec![1]));
        assert_sorted(shell_sort(&mut vec![1,1,1,1,1]));
        assert_sorted(shell_sort(&mut vec![4,2,6,7,1,10]));
        assert_sorted(shell_sort(&mut vec!["w", "h", "p", "a", "x"]));
        assert_sorted(shell_sort(&mut generate_random_large(10000)));
    }

    #[test]
    fn test_merge_sort_top_down() {
        assert_sorted(merge_sort_top_down(&mut vec![1]));
        assert_sorted(merge_sort_top_down(&mut vec![1,1,1,1,1]));
        assert_sorted(merge_sort_top_down(&mut vec![4,2,6,7,1,10]));
        assert_sorted(merge_sort_top_down(&mut vec!["w", "h", "p", "a", "x"]));
        assert_sorted(merge_sort_top_down(&mut generate_random_large(10000)));
    }

    #[test]
    fn test_merge_sort_bottom_up() {
        assert_sorted(merge_sort_bottom_up(&mut vec![1]));
        assert_sorted(merge_sort_bottom_up(&mut vec![1,1,1,1,1]));
        assert_sorted(merge_sort_bottom_up(&mut vec![4,2,6,7,1,10]));
        assert_sorted(merge_sort_bottom_up(&mut vec!["w", "h", "p", "a", "x"]));
        assert_sorted(merge_sort_bottom_up(&mut generate_random_large(10000)));
    }

    #[test]
    fn test_quick_sort() {
        assert_sorted(quick_sort_basic(&mut vec![1]));
        assert_sorted(quick_sort_basic(&mut vec![1,1,1,1,1]));
        assert_sorted(quick_sort_basic(&mut vec![4,2,6,7,1,10]));
        assert_sorted(quick_sort_basic(&mut vec!["w", "h", "p", "a", "x"]));
        assert_sorted(quick_sort_basic(&mut generate_random_large(10000)));
    }
}