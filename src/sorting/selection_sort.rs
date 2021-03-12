#[allow(dead_code)]
pub fn selection_sort(arr: &mut [i32]) {
    let mut min_idx: usize;
    let mut min_val: i32;
    for i in 0..arr.len() {
        min_idx = i;
        min_val = arr[i];
        for (j, v2) in arr.iter().enumerate().skip(i + 1) {
            if *v2 < min_val {
                min_idx = j;
                min_val = *v2;
            }
        }
        arr[min_idx] = arr[i];
        arr[i] = min_val;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let mut arr = [];
        selection_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn one_element() {
        let mut arr = [1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn even_number_of_elements() {
        let mut arr = [1, 3, 5, 4];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 3, 4, 5]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = [1, 42, 3, 5, 2];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 5, 42]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = [42, 42, 42, 42];
        selection_sort(&mut arr);
        assert_eq!(arr, [42, 42, 42, 42]);
    }
}
