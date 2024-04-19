pub mod sorting;

#[cfg(test)]
mod tests {
    use crate::sorting::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![3, 2, 1, 5, 4];
        quick_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![3, 2, 1, 5, 4];
        selection_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![3, 2, 1, 5, 4];
        insertion_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![3, 2, 1, 5, 4];
        merge_sort(&mut arr, |a, b| a < b);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}
