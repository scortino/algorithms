fn sink(arr: &mut [i32], i: usize, size: usize) {
    let (l, r) = (2 * i + 1, 2 * i + 2); // root's children
    let mut largest = i; // assign i here
    if l < size && arr[l] > arr[i] {
        largest = l;
    }
    if r < size && arr[r] > arr[largest] {
        largest = r;
    }
    if largest != i {
        arr.swap(i, largest);
        sink(arr, largest, size);
    }
}

fn heapify(arr: &mut [i32]) {
    let len = arr.len();
    for i in (0..(len / 2 + 1)).rev() {
        sink(arr, i, len);
    }
}

#[allow(dead_code)]
pub fn heap_sort(arr: &mut [i32]) {
    heapify(arr);
    let len = arr.len();
    let mut size = len;
    for i in (1..len).rev() {
        println!("{:?}", arr);
        arr.swap(0, i);
        size -= 1;
        sink(arr, 0, size);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn empty() {
        let mut arr = [];
        heap_sort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn one_element() {
        let mut arr = [1];
        heap_sort(&mut arr);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn even_number_of_elements() {
        let mut arr = [1, 3, 5, 4];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 3, 4, 5]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = [1, 42, 3, 5, 2];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 5, 42]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = [42, 42, 42, 42];
        heap_sort(&mut arr);
        assert_eq!(arr, [42, 42, 42, 42]);
    }

    #[test]
    fn longer() {
        let mut arr = [10, 8, 4, 3, 1, 9, 2, 7, 5, 6];
        heap_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
