fn _partition(arr: &mut [i32], lo: usize, hi: usize) -> usize {
    let pivot = arr[hi - 1];
    let mut i = lo;
    for j in lo..(hi - 1) {
        if arr[j] <= pivot {
            i += 1;
            arr.swap(i - 1, j);
        }
    }
    arr.swap(i, hi - 1);
    i
}

fn _quick_sort(arr: &mut [i32], lo: usize, hi: usize) {
    if lo < hi - 1 {
        let q = _partition(arr, lo, hi);
        _quick_sort(arr, lo, q);
        _quick_sort(arr, q, hi);
    }
}

#[allow(dead_code)]
pub fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len > 1 {
        _quick_sort(arr, 0, len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let mut arr = [];
        quick_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn one_element() {
        let mut arr = [1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn even_number_of_elements() {
        let mut arr = [1, 3, 5, 4];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 3, 4, 5]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = [1, 42, 3, 5, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 5, 42]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = [42, 42, 42, 42];
        quick_sort(&mut arr);
        assert_eq!(arr, [42, 42, 42, 42]);
    }

    #[test]
    fn longer() {
        let mut arr = [10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
