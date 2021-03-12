fn _merge(arr: &mut [i32], lo: usize, mid: usize, hi: usize) {
    let n1 = mid - lo;
    let n2 = hi - mid;
    let mut left = Vec::with_capacity(n1);
    let mut right = Vec::with_capacity(n2);
    for x in arr[lo..mid].iter() {
        left.push(*x);
    }
    for y in arr[mid..hi].iter() {
        right.push(*y);
    }
    let (mut i, mut j, mut k) = (0, 0, lo);
    loop {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
        k += 1;
        if i >= n1 {
            arr[k..hi].clone_from_slice(&right[j..n2]);
            break;
        }
        if j >= n2 {
            arr[k..hi].clone_from_slice(&left[i..n1]);
            break;
        }
    }
}

fn _merge_sort(arr: &mut [i32], lo: usize, hi: usize) {
    if lo < (hi - 1) {
        let mid = (lo + hi) / 2;
        println!("{:?}", (lo, mid, hi));
        _merge_sort(arr, lo, mid);
        _merge_sort(arr, mid, hi);
        _merge(arr, lo, mid, hi);
    }
}

#[allow(dead_code)]
pub fn merge_sort(arr: &mut [i32]) {
    let len = arr.len();
    if arr.len() > 1 {
        _merge_sort(arr, 0, len);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let mut arr = [];
        merge_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn one_element() {
        let mut arr = [1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn even_number_of_elements() {
        let mut arr = [1, 3, 5, 4];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 3, 4, 5]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = [1, 42, 3, 5, 2];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 5, 42]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = [42, 42, 42, 42];
        merge_sort(&mut arr);
        assert_eq!(arr, [42, 42, 42, 42]);
    }

    #[test]
    fn longer() {
        let mut arr = [10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
