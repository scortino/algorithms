#[allow(dead_code)]
pub fn insertion_sort(arr: &mut [i32]) {
    let mut i: usize;
    let mut key: i32;
    for j in 1..arr.len() {
        key = arr[j];
        i = j;
        while i > 0 && arr[i-1] > key {
            arr[i] = arr[i-1];
            i = i - 1;
        }
        arr[i] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let mut arr = [];
        insertion_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn one_element() {
        let mut arr = [1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn even_number_of_elements() {
        let mut arr = [1, 3, 5, 4];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 3, 4, 5]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = [1, 42, 3, 5, 2];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 5, 42]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = [42, 42, 42, 42];
        insertion_sort(&mut arr);
        assert_eq!(arr, [42, 42, 42, 42]);
    }
}