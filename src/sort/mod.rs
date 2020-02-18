pub mod insertion_sort;
pub mod merge_sort;
pub mod quick_sort;

#[cfg(test)]
mod tests {
    use super::insertion_sort::insertion_sort;
    use super::merge_sort::merge_sort_basic;
    use super::quick_sort::quick_sort_basic;

    fn assert_sorted<T: PartialOrd>(v: &Vec<T>) {
        for i in 0..v.len()-1 {
            assert!(v[i] <= v[i+1]);
        }
    }

    #[test]
    fn test_quick_sort() {
        assert_sorted(quick_sort_basic(&mut vec![1]));
        assert_sorted(quick_sort_basic(&mut vec![1,1,1,1,1]));
        assert_sorted(quick_sort_basic(&mut vec![4,2,6,7,1,10]));
        assert_sorted(quick_sort_basic(&mut vec!["w", "h", "p", "a", "x"]));
    }

    #[test]
    fn test_insertion_sort() {
        assert_sorted(insertion_sort(&mut vec![1]));
        assert_sorted(insertion_sort(&mut vec![1,1,1,1,1]));
        assert_sorted(insertion_sort(&mut vec![4,2,6,7,1,10]));
        assert_sorted(insertion_sort(&mut vec!["w", "h", "p", "a", "x"]));
    }

    #[test]
    fn test_merge_sort() {
        assert_sorted(merge_sort_basic(&mut vec![1]));
    }
}