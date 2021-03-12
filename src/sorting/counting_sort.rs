#[allow(dead_code)]
pub fn counting_sort(arr: &mut [u32], max: usize) {
    let len = arr.len();
    let mut sorted: Vec<u32> = vec![0; len];
    let mut count: Vec<usize> = vec![0; max + 1];
    for val in arr.iter() {
        count[*val as usize] += 1;
    }
    for i in 1..(max + 1) {
        count[i] += count[i - 1];
    }
    for j in (0..len).rev() {
        sorted[count[arr[j] as usize] - 1] = arr[j];
        count[arr[j] as usize] -= 1;
    }
    arr.clone_from_slice(&sorted);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one_element() {
        let mut arr = [1];
        counting_sort(&mut arr, 1);
        assert_eq!(arr, [1]);
    }

    #[test]
    fn already_sorted() {
        let mut arr = [1, 2, 3];
        counting_sort(&mut arr, 3);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn even_number_of_elements() {
        let mut arr = [1, 3, 5, 4];
        counting_sort(&mut arr, 5);
        assert_eq!(arr, [1, 3, 4, 5]);
    }

    #[test]
    fn odd_number_of_elements() {
        let mut arr = [1, 42, 3, 5, 2];
        counting_sort(&mut arr, 42);
        assert_eq!(arr, [1, 2, 3, 5, 42]);
    }

    #[test]
    fn repeated_elements() {
        let mut arr = [42, 42, 42, 42];
        counting_sort(&mut arr, 42);
        assert_eq!(arr, [42, 42, 42, 42]);
    }
}
