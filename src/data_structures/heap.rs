/*
Vector implementation of MaxHeap
TODO:
    - Generalize Heap struct to allow for MinHeap and other through inheritance
    - Improve exceptions handling through enum matching

*/
pub struct MaxHeap {
    pub size: usize,
    pub items: Vec<i32>,
}

impl Default for MaxHeap {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(dead_code)]
impl MaxHeap {
    pub fn new() -> Self {
        MaxHeap {
            size: 0,
            items: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn parent(&self, i: usize) -> usize {
        if i == 0 {
            0
        } else {
            (i - 1) / 2
        }
    }

    fn left(&self, i: usize) -> usize {
        i * 2 + 1
    }

    fn right(&self, i: usize) -> usize {
        i * 2 + 2
    }

    pub fn max_heapify(&mut self, i: usize) {
        let l = self.left(i);
        let r = self.right(i);
        let mut largest: usize = i;
        if l < self.size && self.items[l] > self.items[i] {
            largest = l;
        }
        if r < self.size && self.items[r] > self.items[largest] {
            largest = r;
        }
        if largest != i {
            self.items.swap(i, largest);
            self.max_heapify(largest);
        }
    }

    pub fn from_vector(v: Vec<i32>) -> Self {
        let mut heap = MaxHeap {
            size: v.len(),
            items: v,
        };
        for i in (0..(heap.size / 2 + 1)).rev() {
            heap.max_heapify(i);
        }
        heap
    }

    pub fn max(&self) -> i32 {
        if self.size > 0 {
            self.items[0]
        } else {
            panic!("heap underflow");
        }
    }

    pub fn extract_max(&mut self) -> i32 {
        if self.size > 0 {
            let max = self.max();
            self.items[0] = self.items[self.size - 1];
            self.size -= 1;
            self.items.pop();
            self.max_heapify(0);
            max
        } else {
            panic!("heap underflow");
        }
    }

    pub fn increase(&mut self, mut i: usize, new_v: i32) {
        if new_v < self.items[i] {
            panic!("new value smaller than current value");
        }
        self.items[i] = new_v;
        let mut parent: usize;
        while i > 0 {
            parent = self.parent(i);
            if self.items[parent] >= self.items[i] {
                break;
            }
            self.items.swap(i, parent);
            i = parent;
        }
    }

    pub fn insert(&mut self, new_v: i32) {
        self.size += 1;
        self.items.push(std::i32::MIN);
        self.increase(self.size - 1, new_v);
    }
}

fn _satisfies_max_heap_property(heap: &MaxHeap, i: usize) -> bool {
    if heap.is_empty() {
        panic!("heap underflow")
    } else if heap.len() == 1 || i >= heap.len() {
        true
    } else {
        (heap.items[heap.parent(i)] >= heap.items[i])
            && _satisfies_max_heap_property(heap, heap.left(i))
            && _satisfies_max_heap_property(heap, heap.left(i))
    }
}

#[allow(dead_code)]
fn satisfies_max_heap_property(heap: &MaxHeap) -> bool {
    _satisfies_max_heap_property(heap, 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_max_heap_insertion() {
        let mut heap = MaxHeap::new();
        heap.insert(4);
        heap.insert(2);
        heap.insert(9);
        heap.insert(11);
        assert_eq!(heap.len(), 4);
        assert!(satisfies_max_heap_property(&heap));
        heap.insert(8);
        assert_eq!(heap.len(), 5);
        assert!(satisfies_max_heap_property(&heap));
        assert_eq!(heap.max(), 11);
    }

    #[test]
    fn test_max_heap_from_vector() {
        let mut heap = MaxHeap::from_vector(vec![4, 2, 0, 11]);
        assert_eq!(heap.len(), 4);
        assert!(satisfies_max_heap_property(&heap));
        heap.insert(8);
        assert_eq!(heap.len(), 5);
        assert!(satisfies_max_heap_property(&heap));
        assert_eq!(heap.max(), 11);
    }

    #[test]
    fn test_max_heap_increase() {
        let mut heap = MaxHeap::from_vector(vec![4, 2, 0, 11, 1]);
        assert!(satisfies_max_heap_property(&heap));
        assert_eq!(heap.max(), 11);
        heap.increase(heap.len() - 1, 20);
        assert!(satisfies_max_heap_property(&heap));
        assert_eq!(heap.max(), 20);
    }

    #[test]
    fn test_max_heap_extract_max() {
        let mut heap = MaxHeap::from_vector(vec![4, 2, 0, 11, 1]);
        assert!(satisfies_max_heap_property(&heap));
        assert_eq!(heap.max(), 11);
        assert_eq!(heap.len(), 5);
        heap.extract_max();
        assert!(satisfies_max_heap_property(&heap));
        assert_eq!(heap.max(), 4);
        assert_eq!(heap.len(), 4);
    }
}
