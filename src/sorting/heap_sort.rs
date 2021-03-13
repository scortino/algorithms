// TODO: in-place version

use crate::data_structures::MaxHeap;

#[allow(dead_code)]
pub fn heap_sort(arr: Vec<i32>) -> Vec<i32> {
    let mut heapified_arr = MaxHeap::from_vector(arr);
    for i in (1..heapified_arr.len()).rev() {
        heapified_arr.items.swap(0, i);
        heapified_arr.size -= 1;
        heapified_arr.max_heapify(0);
    }
    heapified_arr.items
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let arr = vec![];
        assert_eq!(heap_sort(arr), vec![]);
    }

    #[test]
    fn one_element() {
        let arr = vec![1];
        assert_eq!(heap_sort(arr), vec![1]);
    }

    #[test]
    fn already_sorted() {
        let arr = vec![1, 2, 3];
        assert_eq!(heap_sort(arr), vec![1, 2, 3]);
    }

    #[test]
    fn even_number_of_elements() {
        let arr = vec![1, 3, 5, 4];
        assert_eq!(heap_sort(arr), vec![1, 3, 4, 5]);
    }

    #[test]
    fn odd_number_of_elements() {
        let arr = vec![1, 42, 3, 5, 2];
        assert_eq!(heap_sort(arr), vec![1, 2, 3, 5, 42]);
    }

    #[test]
    fn repeated_elements() {
        let arr = vec![42, 42, 42, 42];
        assert_eq!(heap_sort(arr), vec![42, 42, 42, 42]);
    }

    #[test]
    fn longer() {
        let arr = vec![10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        assert_eq!(heap_sort(arr), vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
