fn _merge(arr: &mut [i32], l: usize, m: usize, h: usize) {
    let n1 = m - l;
    let n2 = h - m;
    let mut left = Vec::with_capacity(n1);
    let mut right = Vec::with_capacity(n2);
    for x in arr[l..m].iter() {
        left.push(*x);
    }
    for y in arr[m..h].iter() {
        right.push(*y);
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = l;
    loop {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i = i + 1;
        } else {
            arr[k] = right[j];
            j = j + 1;
        }
        k = k + 1;
        if i >= n1 {
            arr[k..h].clone_from_slice(&right[j..n2]);
            break;
        }
        if j >= n2 {
            arr[k..h].clone_from_slice(&left[i..n1]);
            break;
        }
    }
}

fn _merge_sort(arr: &mut [i32], l: usize, h: usize) {
    if l < (h - 1) {
        let m = (l + h) / 2;
        println!("{:?}", (l, m, h));
        _merge_sort(arr, l, m);
        _merge_sort(arr, m, h);
        _merge(arr, l, m, h);
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